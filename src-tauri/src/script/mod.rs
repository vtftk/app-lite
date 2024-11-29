use deno_core::*;
use error::AnyError;
use futures::{channel::mpsc, future::poll_fn, SinkExt, StreamExt};
use std::{sync::Arc, task::Poll, time::Duration};
use tokio::time::sleep;

#[op2(fast)]
fn api_invoke(#[string] data: String) {
    println!("callback in rust hit: {}", data);
}

deno_core::extension!(api_extension, ops = [api_invoke], docs = "API integration");

#[tokio::test]
pub async fn test_scripting() {
    let script = r#"
const eventHandlers = {};

function on(key, callback) {
  if (!eventHandlers[key]) {
    eventHandlers[key] = [];
  }
  eventHandlers[key].push(callback);
}

function _triggerEvent(key, event) {
  if (eventHandlers[key]) {
    eventHandlers[key].forEach((callback) => callback(event));
  }
}

function api_invoke(data) {
    Deno.core.ops.api_invoke(data);
}

on("chat", async (event) => {
    api_invoke("TEST FROM JS" + event.message + JSON.stringify(JSON.parse(\"{}\")));
})
    "#;

    let extensions = vec![api_extension::ext()];
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions,
        ..Default::default()
    });

    let _ = runtime.execute_script("user_script", script);

    println!("executing trigger");
    runtime
        .execute_script(
            "",
            "_triggerEvent(\"chat\", { message: \"Test message recv\"})",
        )
        .unwrap();

    runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await
        .unwrap();
}
