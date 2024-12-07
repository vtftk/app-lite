use crate::script::events::{global_script_event, KvGet, KvRemove, KvSet};
use anyhow::Context;
use deno_core::*;

#[op2(async)]
#[string]
pub async fn op_kv_get(#[string] key: String) -> anyhow::Result<Option<String>> {
    global_script_event(KvGet { key })
        .await
        .context("failed to send event")?
}

#[op2(async)]
#[string]
pub async fn op_kv_remove(#[string] key: String) -> anyhow::Result<()> {
    global_script_event(KvRemove { key })
        .await
        .context("failed to send event")?
}

#[op2(async)]
pub async fn op_kv_set(#[string] key: String, #[string] value: String) -> anyhow::Result<()> {
    global_script_event(KvSet { key, value })
        .await
        .context("failed to send event")?
}
