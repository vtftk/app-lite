use crate::{
    database::entity::SoundModel,
    events::{ItemWithImpactSoundIds, ItemsWithSounds, ThrowItemConfig},
    integrations::tts_monster::TTSMonsterVoice,
    script::events::{
        global_script_event, GetItemsByIDs, GetItemsByNames, GetSoundsByIDs, GetSoundsByNames,
        PlaySound, PlaySoundSeq, TTSGenerate, TTSGenerateParsed, TTSGetVoices, ThrowItems,
        TriggerHotkey, TriggerHotkeyByName,
    },
};
use anyhow::Context;
use chrono::Utc;
use deno_core::op2;
use serde::Deserialize;
use uuid::Uuid;

#[op2(async)]
#[serde]
pub async fn op_vtftk_trigger_vt_hotkey(#[string] hotkey_id: String) -> anyhow::Result<()> {
    global_script_event(TriggerHotkey { hotkey_id })
        .await
        .context("failed to send event")?
}

#[op2(async)]
#[serde]
pub async fn op_vtftk_trigger_vt_hotkey_by_name(
    #[string] hotkey_name: String,
    ignore_case: bool,
) -> anyhow::Result<()> {
    global_script_event(TriggerHotkeyByName {
        hotkey_name,
        ignore_case,
    })
    .await
    .context("failed to send event")?
}

/// Throw items
#[op2(async)]
#[serde]
pub async fn op_vtftk_throw_items(
    #[serde] items: ItemsWithSounds,
    #[serde] config: ThrowItemConfig,
) -> anyhow::Result<()> {
    global_script_event(ThrowItems { items, config })
        .await
        .context("failed to send event")?
}

/// Find items by name
#[op2(async)]
#[serde]
pub async fn op_vtftk_get_items_by_names(
    #[serde] names: Vec<String>,
    ignore_case: bool,
) -> anyhow::Result<Vec<ItemWithImpactSoundIds>> {
    global_script_event(GetItemsByNames { names, ignore_case })
        .await
        .context("failed to send event")?
}

/// Find items by ids
#[op2(async)]
#[serde]
pub async fn op_vtftk_get_items_by_ids(
    #[serde] ids: Vec<Uuid>,
) -> anyhow::Result<Vec<ItemWithImpactSoundIds>> {
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

#[op2(async)]
#[string]
pub async fn op_vtftk_play_sound(#[string] src: String, volume: f32) -> anyhow::Result<()> {
    let config = SoundModel {
        id: Uuid::new_v4(),
        name: "<internal>".to_string(),
        src,
        volume,
        order: 0,
        created_at: Utc::now(),
    };

    global_script_event(PlaySound { config })
        .await
        .context("failed to send event")?
}

#[derive(Debug, Deserialize)]
pub struct SoundSeq {
    pub src: String,
    pub volume: f32,
}

#[op2(async)]
#[string]
pub async fn op_vtftk_play_sound_seq(#[serde] seq: Vec<SoundSeq>) -> anyhow::Result<()> {
    let configs = seq
        .into_iter()
        .map(|seq| SoundModel {
            id: Uuid::new_v4(),
            name: "<internal>".to_string(),
            src: seq.src,
            volume: seq.volume,
            order: 0,
            created_at: Utc::now(),
        })
        .collect();

    global_script_event(PlaySoundSeq { configs })
        .await
        .context("failed to send event")?
}

#[op2(async)]
#[serde]
pub async fn op_vtftk_tts_generate_parsed(
    #[string] message: String,
) -> anyhow::Result<Vec<String>> {
    global_script_event(TTSGenerateParsed { message })
        .await
        .context("failed to send event")?
}

#[op2(async)]
#[serde]
pub async fn op_vtftk_tts_get_voices() -> anyhow::Result<Vec<TTSMonsterVoice>> {
    global_script_event(TTSGetVoices)
        .await
        .context("failed to send event")?
}

#[op2(async)]
#[string]
pub async fn op_vtftk_tts_generate(
    #[serde] voice_id: Uuid,
    #[string] message: String,
) -> anyhow::Result<String> {
    global_script_event(TTSGenerate { voice_id, message })
        .await
        .context("failed to send event")?
}
