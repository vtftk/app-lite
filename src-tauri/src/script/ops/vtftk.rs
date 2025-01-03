use crate::{
    database::entity::SoundModel,
    integrations::tts_monster::TTSMonsterVoice,
    script::events::{
        global_script_event, GetSoundByID, GetSoundsByName, PlaySound, PlaySoundSeq, TTSGenerate,
        TTSGenerateParsed, TTSGetVoices,
    },
};
use anyhow::Context;
use chrono::Utc;
use deno_core::op2;
use serde::Deserialize;
use uuid::Uuid;

/// Find sounds by name
#[op2(async)]
#[serde]
pub async fn op_vtftk_get_sounds_by_name(
    #[string] name: String,
    ignore_case: bool,
) -> anyhow::Result<Vec<SoundModel>> {
    global_script_event(GetSoundsByName { name, ignore_case })
        .await
        .context("failed to send event")?
}

/// Find sound by ID
#[op2(async)]
#[serde]
pub async fn op_vtftk_get_sound_by_id(#[serde] id: Uuid) -> anyhow::Result<Option<SoundModel>> {
    global_script_event(GetSoundByID { id })
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
