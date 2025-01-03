use crate::{
    events::matching::{EventData, EventInputData},
    script::ops::{
        http::op_http_request,
        kv::{op_kv_get, op_kv_remove, op_kv_set},
        logging::op_log,
        twitch::{
            op_twitch_create_stream_marker, op_twitch_delete_all_chat_messages,
            op_twitch_delete_chat_message, op_twitch_get_follower, op_twitch_get_user_by_username,
            op_twitch_is_mod, op_twitch_is_vip, op_twitch_send_chat,
            op_twitch_send_chat_announcement, op_twitch_send_shoutout,
        },
        vtftk::{
            op_vtftk_get_items_by_ids, op_vtftk_get_items_by_names, op_vtftk_get_sounds_by_ids,
            op_vtftk_get_sounds_by_names, op_vtftk_play_sound, op_vtftk_play_sound_seq,
            op_vtftk_throw_items, op_vtftk_tts_generate, op_vtftk_tts_generate_parsed,
            op_vtftk_tts_get_voices,
        },
    },
};
use anyhow::Context;
use deno_core::{
    serde_v8::to_v8,
    v8::{self, Global, Local},
    JsRuntime, PollEventLoopOptions, RuntimeOptions,
};
use serde::{Deserialize, Serialize};
use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::Poll,
};
use tokio::{
    sync::{mpsc, oneshot},
    task::{self, LocalSet},
};
use twitch_api::types::{DisplayName, UserId, UserName};
use uuid::Uuid;

/// Snapshot of the script engine runtime, see [build.rs](../../build.rs)
static SCRIPT_RUNTIME_SNAPSHOT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/SCRIPT_RUNTIME_SNAPSHOT.bin"));

deno_core::extension!(
    api_extension,
    ops = [
        // HTTP
        op_http_request,
        // Logging
        op_log,
        // Twitch
        op_twitch_send_chat,
        op_twitch_is_mod,
        op_twitch_is_vip,
        op_twitch_get_user_by_username,
        op_twitch_get_follower,
        op_twitch_send_chat_announcement,
        op_twitch_send_shoutout,
        op_twitch_delete_chat_message,
        op_twitch_delete_all_chat_messages,
        op_twitch_create_stream_marker,
        // KV
        op_kv_get,
        op_kv_set,
        op_kv_remove,
        // VTFTK Sounds
        op_vtftk_play_sound,
        op_vtftk_play_sound_seq,
        op_vtftk_get_sounds_by_names,
        op_vtftk_get_sounds_by_ids,
        // VTFTK Items
        op_vtftk_get_items_by_names,
        op_vtftk_get_items_by_ids,
        op_vtftk_throw_items,
        // TTS Monster
        op_vtftk_tts_generate,
        op_vtftk_tts_get_voices,
        op_vtftk_tts_generate_parsed,
    ],
    docs = "Extension providing APIs to the JS runtime"
);

/// Context passed to the JS runtime that is tracked
/// across async calls for handling logging sources
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RuntimeExecutionContext {
    /// Runtime execution started from a event
    Event { event_id: Uuid },
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
        data: EventData,
    ) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(ScriptExecutorMessage::EventScript {
                ctx,
                script,
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
            .expect("failed to create script async runtime");

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
                        data,
                        tx,
                    } => {
                        let result = execute_script(&mut js_runtime, ctx, script, data);
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandContext {
    pub message_id: String,
    pub full_message: String,
    pub message: String,
    pub user: CommandContextUser,
    pub args: Vec<String>,
    pub input_data: EventInputData,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandContextUser {
    pub id: UserId,
    pub name: UserName,
    pub display_name: DisplayName,
}

/// Executes the provided command
///
/// Returns a promise value that resolves when the command is complete
fn execute_command(
    runtime: &mut JsRuntime,
    ctx: RuntimeExecutionContext,
    script: String,
    cmd_ctx: CommandContext,
) -> anyhow::Result<v8::Global<v8::Value>> {
    // Get the handle scope
    let scope = &mut runtime.handle_scope();

    // Wrap code in async function to allow await
    let code = format!("async (ctx) => {{ {script} }}");

    // Eval user code to create the async function
    let event_fn: Local<'_, v8::Function> =
        JsRuntime::eval(scope, &code).context("failed to create script function")?;

    // Get the global object
    let global = scope.get_current_context().global(scope);

    // Create object keys
    let api_key = to_v8(scope, "api")?;
    let internal_key = to_v8(scope, "internal")?;
    let execute_command_outlet_key = to_v8(scope, "executeCommandOutlet")?;

    // Get API object
    let api: Local<'_, v8::Object> = global
        .get(scope, api_key)
        .context("api unavailable")?
        .try_cast()?;

    // Get internal API object
    let internal: Local<'_, v8::Object> = api
        .get(scope, internal_key)
        .context("internal api unavailable")?
        .try_cast()?;

    // Get executeCommandOutlet function
    let execute_command_outlet: Local<'_, v8::Function> = internal
        .get(scope, execute_command_outlet_key)
        .context("executeCommandOutlet missing")?
        .try_cast()?;

    let global_value = global.try_cast()?;
    let ctx_value = to_v8(scope, ctx)?;
    let cmd_ctx_value = to_v8(scope, cmd_ctx)?;
    let event_fn_value = event_fn.try_cast()?;

    let result = execute_command_outlet
        .call(
            scope,
            global_value,
            &[ctx_value, cmd_ctx_value, event_fn_value],
        )
        .context("function provided no return value")?;

    Ok(Global::new(scope, result))
}

/// Executes the provided script using the provided event
///
/// Returns a promise value that resolves when the script is complete
fn execute_script(
    runtime: &mut JsRuntime,
    ctx: RuntimeExecutionContext,
    script: String,
    data: EventData,
) -> anyhow::Result<v8::Global<v8::Value>> {
    // Get the handle scope
    let scope = &mut runtime.handle_scope();

    // Wrap code in async function to allow await
    let code = format!("async (event) => {{ {script} }}");

    // Eval user code to create the async function
    let event_fn: Local<'_, v8::Function> =
        JsRuntime::eval(scope, &code).context("failed to create script function")?;

    // Get the global object
    let global = scope.get_current_context().global(scope);

    // Create object keys
    let api_key = to_v8(scope, "api")?;
    let internal_key = to_v8(scope, "internal")?;
    let execute_event_outlet_key = to_v8(scope, "executeEventOutlet")?;

    // Get API object
    let api: Local<'_, v8::Object> = global
        .get(scope, api_key)
        .context("api unavailable")?
        .try_cast()?;

    // Get internal API object
    let internal: Local<'_, v8::Object> = api
        .get(scope, internal_key)
        .context("internal api unavailable")?
        .try_cast()?;

    // Get executeEventOutlet function
    let execute_event_outlet: Local<'_, v8::Function> = internal
        .get(scope, execute_event_outlet_key)
        .context("executeEventOutlet missing")?
        .try_cast()?;

    let global_value = global.try_cast()?;
    let ctx_value = to_v8(scope, ctx)?;
    let data_value = to_v8(scope, data)?;
    let event_fn_value = event_fn.try_cast()?;

    let result = execute_event_outlet
        .call(
            scope,
            global_value,
            &[ctx_value, data_value, event_fn_value],
        )
        .context("function provided no return value")?;

    Ok(Global::new(scope, result))
}
