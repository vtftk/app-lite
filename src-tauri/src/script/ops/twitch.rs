use crate::script::events::{JsEventMessage, SCRIPT_EVENT_PRODUCER};
use anyhow::{anyhow, Context};
use deno_core::*;
use log::debug;
use tokio::sync::oneshot;
use twitch_api::types::UserId;

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

#[op2(async)]
pub async fn op_twitch_is_mod(#[string] user_id: String) -> anyhow::Result<bool> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::TwitchIsMod {
                user_id: UserId::new(user_id),
                return_tx: tx,
            })
            .await
            .context("failed to send event")?;

        rx.await.context("event producer is closed")?
    } else {
        Err(anyhow!("no event producer is available"))
    }
}

#[op2(async)]
pub async fn op_twitch_is_vip(#[string] user_id: String) -> anyhow::Result<bool> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::TwitchIsVip {
                user_id: UserId::new(user_id),
                return_tx: tx,
            })
            .await
            .context("failed to send event")?;

        rx.await.context("event producer is closed")?
    } else {
        Err(anyhow!("no event producer is available"))
    }
}
