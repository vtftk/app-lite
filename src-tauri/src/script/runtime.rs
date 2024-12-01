use anyhow::Context;
use deno_core::{
    serde_v8::{from_v8, to_v8},
    v8::{self, Global},
    JsRuntime, PollEventLoopOptions, RuntimeOptions,
};
use log::debug;
use tokio::{
    sync::{mpsc, oneshot},
    task::LocalSet,
};

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

/// Message the JS executor will receive to handle running a script
struct ScriptExecutorMessage {
    /// The script code to run
    script: String,
    /// The event to trigger within the code
    event: ScriptExecuteEvent,
    /// Channel to send back the result
    tx: oneshot::Sender<anyhow::Result<()>>,
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
            .send(ScriptExecutorMessage { script, event, tx })
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
        let set = LocalSet::new();

        set.block_on(&runtime, async {
            while let Some(msg) = rx.recv().await {
                set.spawn_local(async move {
                    let result = execute_script(msg.script, msg.event).await;
                    _ = msg.tx.send(result);
                });
            }
        })
    });

    ScriptExecutorHandle { tx }
}

fn create_js_runtime() -> JsRuntime {
    // Create runtime
    JsRuntime::new(RuntimeOptions {
        startup_snapshot: Some(SCRIPT_RUNTIME_SNAPSHOT),
        extensions: vec![api_extension::init_ops()],
        ..Default::default()
    })
}

/// Invokes the `_triggerEvent` function from the runtime.js wrapper code to trigger
/// a specific event in any JS script code.
///
/// Will await for the returned promise is complete to handle the full completion
/// of any async outcomes
async fn trigger_event(runtime: &mut JsRuntime, event: ScriptExecuteEvent) -> anyhow::Result<()> {
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

        let result = trigger_event_function
            .call(scope, global.into(), &[event_data_object])
            .context("failed to call event trigger")?;

        Global::new(scope, result)
    };

    // Run event loop to completion
    runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await?;

    let _result = runtime.resolve(global_promise).await?;

    Ok(())
}

/// Executes the provided script using the provided event
async fn execute_script(script: String, event: ScriptExecuteEvent) -> anyhow::Result<()> {
    // Create runtime
    let mut runtime = create_js_runtime();

    debug!("executing script: {}", script);

    // Execute script
    let _ = runtime.execute_script("<anon>", script)?;

    debug!("fully executed local");

    trigger_event(&mut runtime, event).await?;

    debug!("fully executed script");

    Ok(())
}

/// Executes a script, uses the wrapper code to determine which events the user
/// has subscribed to
pub fn get_script_events(script: String) -> anyhow::Result<Vec<String>> {
    // Create runtime
    let mut runtime = create_js_runtime();

    // Execute script
    let _ = runtime.execute_script("<anon>", script);

    // Get the handle scope
    let scope = &mut runtime.handle_scope();

    // Get the global object
    let global = scope.get_current_context().global(scope);

    let get_events_key =
        v8::String::new(scope, "_getEvents").context("failed to get events key")?;
    let get_events_value = global
        .get(scope, get_events_key.into())
        .context("failed to get events value")?;
    let get_events_function: v8::Local<v8::Function> = get_events_value
        .try_into()
        .context("_getEvents was not a function")?;

    let result = get_events_function
        .call(scope, global.into(), &[])
        .context("expected events return value")?;

    let event_names: Vec<String> = from_v8(scope, result).context("failed to get events return")?;

    Ok(event_names)
}
