use crate::script::events::{JsEventMessage, SCRIPT_EVENT_PRODUCER};
use anyhow::{anyhow, Context};
use deno_core::*;
use log::debug;
use tokio::sync::oneshot;

/// Operation for sending a chat message from JS
#[op2(async)]
pub async fn op_twitch_send_chat(#[string] message: String) -> anyhow::Result<()> {
    debug!("requested sending twitch chat message: {}", message);

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
