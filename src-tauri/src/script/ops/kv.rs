use crate::{
    database::entity::key_value::KeyValueType,
    script::events::{global_script_event, KvGet, KvRemove, KvSet},
};
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
pub async fn op_kv_set(
    #[string] ty: String,
    #[string] key: String,
    #[string] value: String,
) -> anyhow::Result<()> {
    let ty = serde_json::from_str::<KeyValueType>(&format!("\"{ty}\""))?;

    global_script_event(KvSet { key, value, ty })
        .await
        .context("failed to send event")?
}
