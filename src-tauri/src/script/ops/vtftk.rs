use crate::{
    database::entity::sounds::SoundModel,
    events::{EventMessage, ItemWithSoundIds},
    script::events::{
        global_script_event, EmitEventMessage, GetItemsByIDs, GetItemsByNames, GetSoundsByIDs,
        GetSoundsByNames,
    },
};
use anyhow::Context;
use deno_core::op2;
use uuid::Uuid;

/// Emit event messages to the websocket
#[op2(async)]
#[serde]
pub async fn op_vtftk_emit_event_message(#[serde] message: EventMessage) -> anyhow::Result<()> {
    global_script_event(EmitEventMessage { message })
        .await
        .context("failed to send event")?
}

/// Find items by name
#[op2(async)]
#[serde]
pub async fn op_vtftk_get_items_by_names(
    #[serde] names: Vec<String>,
    ignore_case: bool,
) -> anyhow::Result<Vec<ItemWithSoundIds>> {
    global_script_event(GetItemsByNames { names, ignore_case })
        .await
        .context("failed to send event")?
}

/// Find items by ids
#[op2(async)]
#[serde]
pub async fn op_vtftk_get_items_by_ids(
    #[serde] ids: Vec<Uuid>,
) -> anyhow::Result<Vec<ItemWithSoundIds>> {
    global_script_event(GetItemsByIDs { ids })
        .await
        .context("failed to send event")?
}

/// Find sounds by name
#[op2(async)]
#[serde]
pub async fn op_vtftk_get_sounds_by_names(
    #[serde] names: Vec<String>,
    ignore_case: bool,
) -> anyhow::Result<Vec<SoundModel>> {
    global_script_event(GetSoundsByNames { names, ignore_case })
        .await
        .context("failed to send event")?
}

/// Find sound by ID
#[op2(async)]
#[serde]
pub async fn op_vtftk_get_sounds_by_ids(
    #[serde] ids: Vec<Uuid>,
) -> anyhow::Result<Vec<SoundModel>> {
    global_script_event(GetSoundsByIDs { ids })
        .await
        .context("failed to send event")?
}
