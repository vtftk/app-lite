use anyhow::{anyhow, Context};
use deno_core::*;
use events::{JsEventMessage, SCRIPT_EVENT_PRODUCER};
use serde::Serialize;
use serde_v8::{from_v8, to_v8};
use tokio::sync::oneshot;

pub mod events;

#[derive(Debug, Serialize)]
pub struct JsHttpResponse {
    ok: bool,
    status: u16,
    response: String,
}

/// Operation for performing a GET request to a specific URL from JS
#[op2(async)]
#[serde]
async fn op_http_get(#[string] url: String) -> anyhow::Result<serde_json::Value> {
    let response = reqwest::get(url)
        .await
        .context("failed to perform get request")?;

    let status = response.status();
    let body = response.text().await?;
    let ok = status.is_success();

    let value = serde_json::to_value(JsHttpResponse {
        ok,
        status: status.as_u16(),
        response: body,
    })?;

    Ok(value)
}

/// Operation for sending a chat message from JS
#[op2(async)]
async fn op_twitch_send_chat(#[string] message: String) -> anyhow::Result<()> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::TwitchSendChat {
                message,
                return_tx: tx,
            })
            .await
            .context("failed to send event")?;

        rx.await.context("event producer is closed")?
    } else {
        Err(anyhow!("no event producer is available"))
    }
}

const WRAPPER_SCRIPT: &str = include_str!("../../../script/wrapper.js");

deno_core::extension!(
    api_extension,
    ops = [op_http_get, op_twitch_send_chat],
    docs = "API integration"
);

#[serde(rename_all = "snake_case")]
#[derive(Debug, Serialize)]
pub enum ScriptExecuteEvent {
    Chat { message: String },
}

/// Executes a script, uses the wrapper code to determine which events the user
/// has subscribed to
pub async fn get_script_events(script: String) -> anyhow::Result<Vec<String>> {
    // Apply the wrapper script
    let script = format!("{}\n\n{}", WRAPPER_SCRIPT, script);

    // Create runtime
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![api_extension::ext()],
        ..Default::default()
    });

    // Execute script
    let _ = runtime.execute_script("<anon>", script);

    // Get the handle scope
    let scope = &mut runtime.handle_scope();

    // Get the global object
    let global = scope.get_current_context().global(scope);

    let get_events_key =
        v8::String::new(scope, "_triggerEvent").context("failed to create trigger event key")?;
    let get_events_value = global
        .get(scope, get_events_key.into())
        .context("failed to get trigger event value")?;
    let get_events_function: v8::Local<v8::Function> = get_events_value
        .try_into()
        .context("_triggerEvent was not a function")?;

    let result = get_events_function
        .call(scope, global.into(), &[])
        .context("expected events return value")?;

    let event_names: Vec<String> = from_v8(scope, result).context("failed to get events return")?;

    Ok(event_names)
}

/// Executes the provided script using the provided event
pub async fn execute_script(script: String, event: ScriptExecuteEvent) -> anyhow::Result<()> {
    // Apply the wrapper script
    let script = format!("{}\n\n{}", WRAPPER_SCRIPT, script);

    // Create runtime
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![api_extension::ext()],
        ..Default::default()
    });

    // Execute script
    let _ = runtime.execute_script("<anon>", script);

    // Call the event trigger
    {
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

        trigger_event_function
            .call(scope, global.into(), &[event_data_object])
            .context("failed to call event trigger")?;
    }

    // Run event loop to completion
    runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await?;

    Ok(())
}
