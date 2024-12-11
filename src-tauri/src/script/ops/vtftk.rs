use crate::{
    database::entity::SoundModel,
    script::events::{
        global_script_event, PlaySound, PlaySoundSeq, TTSGenerate, TTSGenerateParsed, TTSGetVoices,
    },
    tts::{GenerateRequest, GenerateResponse, Voice},
};
use anyhow::Context;
use deno_core::op2;
use serde::Deserialize;
use uuid::Uuid;

#[op2(async)]
#[string]
pub async fn op_vtftk_play_sound(#[string] src: String, volume: f32) -> anyhow::Result<()> {
    let config = SoundModel {
        id: Uuid::new_v4(),
        name: "<internal>".to_string(),
        src,
        volume,
        order: 0,
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
pub async fn op_vtftk_tts_get_voices() -> anyhow::Result<Vec<Voice>> {
    global_script_event(TTSGetVoices)
        .await
        .context("failed to send event")?
}

#[op2(async)]
#[serde]
pub async fn op_vtftk_tts_generate(
    #[serde] request: GenerateRequest,
) -> anyhow::Result<GenerateResponse> {
    global_script_event(TTSGenerate { request })
        .await
        .context("failed to send event")?
}
