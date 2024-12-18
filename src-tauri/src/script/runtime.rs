use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::Poll,
};

use anyhow::Context;
use deno_core::{
    serde_v8::{from_v8, to_v8},
    v8::{self, Global, Local},
    JsRuntime, PollEventLoopOptions, RuntimeOptions,
};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, oneshot},
    task::{self, LocalSet},
};
use twitch_api::types::{DisplayName, UserId, UserName};
use uuid::Uuid;

use crate::{
    database::entity::script_events::ScriptEvent,
    events::matching::{EventData, EventInputData},
};

use super::ops::{
    http::op_http_get,
    kv::{op_kv_get, op_kv_remove, op_kv_set},
    logging::{op_log_debug, op_log_error, op_log_info, op_log_warn},
    twitch::{
        op_twitch_get_user_by_username, op_twitch_is_mod, op_twitch_is_vip, op_twitch_send_chat,
        op_twitch_send_chat_announcement, op_twitch_send_shoutout,
    },
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
        op_twitch_get_user_by_username,
        op_twitch_send_chat_announcement,
        op_twitch_send_shoutout,
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

/// Context passed to the JS runtime that is tracked
/// across async calls for handling logging sources
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RuntimeExecutionContext {
    /// Runtime execution started from a script
    Script { script_id: Uuid },
    /// Runtime execution started from a command
    Command { command_id: Uuid },
}

#[derive(Debug)]
pub enum ScriptExecutorMessage {
    /// Tell the executor to run the event callbacks in the provided code
    /// on the runtime
    EventScript {
        /// Context for logging
        ctx: RuntimeExecutionContext,
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
        /// Context for logging
        ctx: RuntimeExecutionContext,
        /// The script code to run
        script: String,
        /// Context for the command run
        cmd_ctx: CommandContext,
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
        ctx: RuntimeExecutionContext,
        script: String,
        event: ScriptEvent,
        data: EventData,
    ) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(ScriptExecutorMessage::EventScript {
                ctx,
                script,
                event,
                data,
                tx,
            })
            .await
            .context("executor is not running")?;

        rx.await.context("executor closed without response")?
    }

    pub async fn execute_command(
        &self,
        ctx: RuntimeExecutionContext,
        script: String,
        cmd_ctx: CommandContext,
    ) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(ScriptExecutorMessage::CommandScript {
                ctx,
                script,
                cmd_ctx,
                tx,
            })
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

fn spawn_script_promise(
    js_runtime: &mut JsRuntime,
    global_promise: anyhow::Result<v8::Global<v8::Value>>,
    tx: oneshot::Sender<anyhow::Result<()>>,
) {
    let global_promise = match global_promise {
        Ok(value) => value,
        Err(err) => {
            _ = tx.send(Err(err));
            return;
        }
    };

    let resolve = js_runtime.resolve(global_promise);
    task::spawn_local(async move {
        let result = resolve.await;
        _ = tx.send(result.map(|_| ()));
    });
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

        // Create runtime
        let mut js_runtime = JsRuntime::new(RuntimeOptions {
            startup_snapshot: Some(SCRIPT_RUNTIME_SNAPSHOT),
            extensions: vec![api_extension::init_ops()],

            ..Default::default()
        });

        let mut local_set = LocalSet::new();

        runtime.block_on(poll_fn(|cx| {
            // Initial pass when not messages are available
            {
                // Poll the promises local set
                _ = Pin::new(&mut local_set).poll(cx);

                // Poll event loop for any promises
                let _ = js_runtime.poll_event_loop(cx, PollEventLoopOptions::default());
            }

            // Poll incoming script execute messages
            while let Poll::Ready(msg) = rx.poll_recv(cx) {
                let msg = match msg {
                    Some(msg) => msg,
                    None => return Poll::Ready(()),
                };

                let _task_guard = local_set.enter();

                match msg {
                    ScriptExecutorMessage::EventScript {
                        ctx,
                        script,
                        event,
                        data,
                        tx,
                    } => {
                        let result = execute_script(&mut js_runtime, ctx, script, event, data);
                        spawn_script_promise(&mut js_runtime, result, tx)
                    }
                    ScriptExecutorMessage::CommandScript {
                        ctx,
                        script,
                        cmd_ctx,
                        tx,
                    } => {
                        let result = execute_command(&mut js_runtime, ctx, script, cmd_ctx);
                        spawn_script_promise(&mut js_runtime, result, tx)
                    }
                    ScriptExecutorMessage::EventsList { script, tx } => {
                        let result = get_script_events(&mut js_runtime, script);
                        _ = tx.send(result);
                    }
                }

                // Poll the promises local set
                _ = Pin::new(&mut local_set).poll(cx);

                // Poll the event loop
                let _ = js_runtime.poll_event_loop(cx, PollEventLoopOptions::default());
            }

            Poll::Pending
        }));
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

/// Executes the provided command'
///
/// Returns a promise value that resolves when the command is complete
fn execute_command(
    runtime: &mut JsRuntime,
    ctx: RuntimeExecutionContext,
    script: String,
    cmd_ctx: CommandContext,
) -> anyhow::Result<v8::Global<v8::Value>> {
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
        let cmd_ctx_value = to_v8(scope, cmd_ctx)?;

        let result = local_fn
            .call(scope, global, &[ctx_value, cmd_ctx_value])
            .context("function provided no return value")?;
        Global::new(scope, result)
    };

    Ok(global_promise)
}

/// Executes the provided script using the provided event
///
/// Returns a promise value that resolves when the script is complete
fn execute_script(
    runtime: &mut JsRuntime,
    ctx: RuntimeExecutionContext,
    script: String,
    event: ScriptEvent,
    data: EventData,
) -> anyhow::Result<v8::Global<v8::Value>> {
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

        let ctx_value = to_v8(scope, ctx)?;
        let event_value = to_v8(scope, event)?;
        let data_value = to_v8(scope, data)?;

        let result = local_fn
            .call(scope, global, &[ctx_value, event_value, data_value])
            .context("function provided no return value")?;
        Global::new(scope, result)
    };

    Ok(global_promise)
}

/// Executes a script, uses the wrapper code to determine which events the user
/// has subscribed to
fn get_script_events(runtime: &mut JsRuntime, script: String) -> anyhow::Result<Vec<ScriptEvent>> {
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
        .filter_map(|name| serde_json::from_str::<ScriptEvent>(&format!("\"{name}\"")).ok())
        .collect();

    Ok(events)
}
