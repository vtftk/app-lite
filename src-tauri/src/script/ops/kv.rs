use crate::script::events::{JsEventMessage, SCRIPT_EVENT_PRODUCER};
use anyhow::{anyhow, Context};
use deno_core::*;
use tokio::sync::oneshot;

#[op2(async)]
#[string]
pub async fn op_kv_get(#[string] key: String) -> anyhow::Result<Option<String>> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::KvGet { key, return_tx: tx })
            .await
            .context("failed to send event")?;

        rx.await.context("event producer is closed")?
    } else {
        Err(anyhow!("no event producer is available"))
    }
}

#[op2(async)]
#[string]
pub async fn op_kv_remove(#[string] key: String) -> anyhow::Result<()> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::KvRemove { key, return_tx: tx })
            .await
            .context("failed to send event")?;

        rx.await.context("event producer is closed")?
    } else {
        Err(anyhow!("no event producer is available"))
    }
}

#[op2(async)]
pub async fn op_kv_set(#[string] key: String, #[string] value: String) -> anyhow::Result<()> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::KvSet {
                key,
                value,
                return_tx: tx,
            })
            .await
            .context("failed to send event")?;

        rx.await.context("event producer is closed")?
    } else {
        Err(anyhow!("no event producer is available"))
    }
}
