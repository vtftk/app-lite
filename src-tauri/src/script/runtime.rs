use anyhow::Context;
use deno_core::{
    serde_v8::{from_v8, to_v8},
    v8::{self, Global, Local},
    JsRuntime, PollEventLoopOptions, RuntimeOptions,
};
use log::debug;
use serde::Serialize;
use tokio::sync::{mpsc, oneshot};
use twitch_api::types::{DisplayName, UserId, UserName};

use crate::{
    database::entity::scripts::ScriptEvent,
    events::matching::{EventData, EventInputData},
};

use super::ops::{
    http::op_http_get,
    kv::{op_kv_get, op_kv_remove, op_kv_set},
    logging::{op_log_debug, op_log_error, op_log_info, op_log_warn},
    twitch::{op_twitch_is_mod, op_twitch_is_vip, op_twitch_send_chat},
    vtftk::{
        op_vtftk_play_sound, op_vtftk_play_sound_seq, op_vtftk_tts_generate,
        op_vtftk_tts_generate_parsed, op_vtftk_tts_get_voices,
    },
};

/// Snapshot of the script engine runtime, see [build.rs](../../build.rs)
static SCRIPT_RUNTIME_SNAPSHOT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/SCRIPT_RUNTIME_SNAPSHOT.bin"));

deno_core::extension!(
    api_extension,
    ops = [
        op_http_get,
        op_twitch_send_chat,
        op_log_debug,
        op_log_info,
        op_log_warn,
        op_log_error,
        op_twitch_is_mod,
        op_twitch_is_vip,
        op_kv_get,
        op_kv_set,
        op_kv_remove,
        op_vtftk_play_sound,
        op_vtftk_tts_generate,
        op_vtftk_tts_get_voices,
        op_vtftk_tts_generate_parsed,
        op_vtftk_play_sound_seq
    ],
    docs = "Extension providing APIs to the JS runtime"
);

pub enum ScriptExecutorMessage {
    /// Tell the executor to run the event callbacks in the provided code
    /// on the runtime
    EventScript {
        /// The script code to run
        script: String,
        /// The event to trigger within the code
        event: ScriptEvent,
        /// Data for the event
        data: EventData,
        /// Channel to send back the result
        tx: oneshot::Sender<anyhow::Result<()>>,
    },

    /// Tell the executor to run the event callbacks in the provided code
    /// on the runtime
    CommandScript {
        /// The script code to run
        script: String,
        /// Context for the command run
        ctx: CommandContext,
        /// Channel to send back the result
        tx: oneshot::Sender<anyhow::Result<()>>,
    },

    /// Tells the executor to run the provided scripts and report the
    /// names of the events that the script subscribes to
    EventsList {
        /// The script code to run
        script: String,
        /// Channel to send back the result
        tx: oneshot::Sender<anyhow::Result<Vec<ScriptEvent>>>,
    },
}

/// Handle for accessing the script executor
#[derive(Clone)]
pub struct ScriptExecutorHandle {
    /// Channel for sending the execute message
    tx: mpsc::Sender<ScriptExecutorMessage>,
}

impl ScriptExecutorHandle {
    /// Execute the provided `script` using `event` on the runtime this handle
    /// is linked to, returning the result
    pub async fn execute(
        &self,
        script: String,
        event: ScriptEvent,
        data: EventData,
    ) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(ScriptExecutorMessage::EventScript {
                script,
                event,
                data,
                tx,
            })
            .await
            .context("executor is not running")?;

        rx.await.context("executor closed without response")?
    }

    pub async fn execute_command(&self, script: String, ctx: CommandContext) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(ScriptExecutorMessage::CommandScript { script, ctx, tx })
            .await
            .context("executor is not running")?;

        rx.await.context("executor closed without response")?
    }

    pub async fn get_events(&self, script: String) -> anyhow::Result<Vec<ScriptEvent>> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(ScriptExecutorMessage::EventsList { script, tx })
            .await
            .context("executor is not running")?;

        rx.await.context("executor closed without response")?
    }
}

/// Creates a dedicated thread for receiving script execution requests. The
/// thread will process the script execution requests providing the responses
///
/// The JS runtime is !Send and thus it cannot be shared across tokio async tasks
/// so here its provided a dedicated single threaded runtime and its own thread
pub fn create_script_executor() -> ScriptExecutorHandle {
    let (tx, mut rx) = mpsc::channel::<ScriptExecutorMessage>(5);

    std::thread::spawn(move || {
        // Create a new tokio runtime in the dedicated thread
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            // Create runtime
            let mut runtime = JsRuntime::new(RuntimeOptions {
                startup_snapshot: Some(SCRIPT_RUNTIME_SNAPSHOT),
                extensions: vec![api_extension::init_ops()],

                ..Default::default()
            });

            while let Some(msg) = rx.recv().await {
                match msg {
                    ScriptExecutorMessage::EventScript {
                        script,
                        event,
                        data,
                        tx,
                    } => {
                        debug!("started script execution");
                        let result = execute_script(&mut runtime, script, event, data).await;
                        _ = tx.send(result);

                        debug!("completed script execution");
                    }
                    ScriptExecutorMessage::EventsList { script, tx } => {
                        let result = get_script_events(&mut runtime, script).await;
                        _ = tx.send(result);
                    }
                    ScriptExecutorMessage::CommandScript { script, ctx, tx } => {
                        let result = execute_command(&mut runtime, script, ctx).await;
                        _ = tx.send(result);
                    }
                }
            }
        })
    });

    ScriptExecutorHandle { tx }
}

static JS_CALL_WRAPPER: &str = include_str!("../../../script/wrapper_call.js");
static JS_EVENTS_WRAPPER: &str = include_str!("../../../script/wrapper_events.js");
static JS_COMMAND_WRAPPER: &str = include_str!("../../../script/wrapper_command.js");

#[derive(Debug, Serialize)]
#[serde(rename = "camelCase")]
pub struct CommandContext {
    pub full_message: String,
    pub message: String,
    pub user: CommandContextUser,
    pub args: Vec<String>,
    pub input_data: EventInputData,
}

#[derive(Debug, Serialize)]
#[serde(rename = "camelCase")]
pub struct CommandContextUser {
    pub id: UserId,
    pub name: UserName,
    pub display_name: DisplayName,
}

/// Executes the provided command
async fn execute_command(
    runtime: &mut JsRuntime,
    script: String,
    ctx: CommandContext,
) -> anyhow::Result<()> {
    let script = JS_COMMAND_WRAPPER.replace("USER_CODE;", &script);

    // Execute script (Wrapper returns a function)
    let output = runtime.execute_script("<anon>", script)?;

    let global_promise: v8::Global<v8::Value> = {
        // Get the handle scope
        let scope = &mut runtime.handle_scope();

        // Get the global object
        let global = scope.get_current_context().global(scope).cast();

        let local = Local::new(scope, output);
        let local_fn: Local<'_, v8::Function> = local
            .try_into()
            .context("wrapper didn't produce function")?;

        let ctx_value = to_v8(scope, ctx)?;

        let result = local_fn
            .call(scope, global, &[ctx_value])
            .context("function provided no return value")?;
        Global::new(scope, result)
    };

    let resolve = runtime.resolve(global_promise);

    // Run event loop to completion
    let _result = runtime
        .with_event_loop_promise(resolve, PollEventLoopOptions::default())
        .await?;

    Ok(())
}

/// Executes the provided script using the provided event
async fn execute_script(
    runtime: &mut JsRuntime,
    script: String,
    event: ScriptEvent,
    data: EventData,
) -> anyhow::Result<()> {
    let script = JS_CALL_WRAPPER.replace("USER_CODE;", &script);

    // Execute script
    let output = runtime.execute_script("<anon>", script)?;

    let global_promise: v8::Global<v8::Value> = {
        // Get the handle scope
        let scope = &mut runtime.handle_scope();

        // Get the global object
        let global = scope.get_current_context().global(scope).cast();

        let local = Local::new(scope, output);
        let local_fn: Local<'_, v8::Function> = local
            .try_into()
            .context("wrapper didn't produce function")?;

        let event_value = to_v8(scope, event)?;
        let data_value = to_v8(scope, data)?;

        let result = local_fn
            .call(scope, global, &[event_value, data_value])
            .context("function provided no return value")?;
        Global::new(scope, result)
    };

    let resolve = runtime.resolve(global_promise);

    // Run event loop to completion
    let _result = runtime
        .with_event_loop_promise(resolve, PollEventLoopOptions::default())
        .await?;

    Ok(())
}

/// Executes a script, uses the wrapper code to determine which events the user
/// has subscribed to
async fn get_script_events(
    runtime: &mut JsRuntime,
    script: String,
) -> anyhow::Result<Vec<ScriptEvent>> {
    let script = JS_EVENTS_WRAPPER.replace("USER_CODE;", &script);

    // Execute script
    let output = runtime.execute_script("<anon>", script)?;

    let names: Vec<String> = {
        let mut scope = runtime.handle_scope();
        let local = Local::new(&mut scope, output);
        from_v8(&mut scope, local).context("invalid events output")?
    };

    let events = names
        .into_iter()
        // Parse event names
        .filter_map(|name| serde_json::from_str::<ScriptEvent>(&name).ok())
        .collect();

    Ok(events)
}
