use anyhow::Context;
use deno_core::{
    serde_v8::{from_v8, to_v8},
    v8::{self, Global, Local},
    JsRuntime, PollEventLoopOptions, RuntimeOptions,
};
use log::debug;
use tokio::sync::{mpsc, oneshot};

use super::{
    events::ScriptExecuteEvent,
    ops::{
        http::op_http_get,
        kv::{op_kv_get, op_kv_remove, op_kv_set},
        logging::{op_log_debug, op_log_error, op_log_info, op_log_warn},
        twitch::{op_twitch_is_mod, op_twitch_is_vip, op_twitch_send_chat},
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
        op_kv_remove
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
        event: ScriptExecuteEvent,
        /// Channel to send back the result
        tx: oneshot::Sender<anyhow::Result<()>>,
    },

    /// Tells the executor to run the provided scripts and report the
    /// names of the events that the script subscribes to
    EventsList {
        /// The script code to run
        script: String,
        /// Channel to send back the result
        tx: oneshot::Sender<anyhow::Result<Vec<String>>>,
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
    pub async fn execute(&self, script: String, event: ScriptExecuteEvent) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(ScriptExecutorMessage::EventScript { script, event, tx })
            .await
            .context("executor is not running")?;

        rx.await.context("executor closed without response")?
    }

    pub async fn get_events(&self, script: String) -> anyhow::Result<Vec<String>> {
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
                    ScriptExecutorMessage::EventScript { script, event, tx } => {
                        debug!("started script execution");
                        let result = execute_script(&mut runtime, script, event).await;
                        _ = tx.send(result);

                        debug!("completed script execution");
                    }
                    ScriptExecutorMessage::EventsList { script, tx } => {
                        let result = get_script_events(&mut runtime, script).await;
                        _ = tx.send(result);
                    }
                }
            }
        })
    });

    ScriptExecutorHandle { tx }
}

/// Invokes the `_triggerEvent` function from the runtime.js wrapper code to trigger
/// a specific event in any JS script code.
///
/// Will await for the returned promise is complete to handle the full completion
/// of any async outcomes
async fn trigger_event(
    runtime: &mut JsRuntime,
    event_handlers: Global<v8::Value>,
    event: ScriptExecuteEvent,
) -> anyhow::Result<()> {
    // Trigger events and wait till they complete
    let global_promise: v8::Global<v8::Value> = {
        // Get the handle scope
        let scope = &mut runtime.handle_scope();

        // Get the global object
        let global = scope.get_current_context().global(scope);

        let trigger_event_key = v8::String::new(scope, "_triggerEvent")
            .context("failed to create trigger event key")?;
        let trigger_event_value = global
            .get(scope, trigger_event_key.into())
            .context("failed to get trigger event value")?;
        let trigger_event_function: v8::Local<v8::Function> = trigger_event_value
            .try_into()
            .context("_triggerEvent was not a function")?;

        let event_data_object = to_v8(scope, event).context("failed to create event object")?;

        let local_event_handlers = v8::Local::new(scope, event_handlers);

        let result = trigger_event_function
            .call(
                scope,
                global.into(),
                &[local_event_handlers, event_data_object],
            )
            .context("failed to call event trigger")?;

        Global::new(scope, result)
    };

    let resolve = runtime.resolve(global_promise);

    // Run event loop to completion
    let _result = runtime
        .with_event_loop_promise(resolve, PollEventLoopOptions::default())
        .await?;

    Ok(())
}

static JS_CALL_WRAPPER: &str = include_str!("./wrapper_call.js");
static JS_EVENTS_WRAPPER: &str = include_str!("./wrapper_events.js");

/// Executes the provided script using the provided event
async fn execute_script(
    runtime: &mut JsRuntime,
    script: String,
    event: ScriptExecuteEvent,
) -> anyhow::Result<()> {
    let script = JS_CALL_WRAPPER.replace("USER_CODE;", &script);

    // Execute script
    let output = runtime.execute_script("<anon>", script)?;

    trigger_event(runtime, output, event).await?;

    Ok(())
}

/// Executes a script, uses the wrapper code to determine which events the user
/// has subscribed to
async fn get_script_events(runtime: &mut JsRuntime, script: String) -> anyhow::Result<Vec<String>> {
    let script = JS_EVENTS_WRAPPER.replace("USER_CODE;", &script);

    // Execute script
    let output = runtime.execute_script("<anon>", script)?;

    let names: Vec<String> = {
        let mut scope = runtime.handle_scope();
        let local = Local::new(&mut scope, output);
        from_v8(&mut scope, local).context("invalid events output")?
    };

    Ok(names)
}
