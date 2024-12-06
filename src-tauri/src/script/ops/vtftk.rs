use crate::{
    script::events::{JsEventMessage, SCRIPT_EVENT_PRODUCER},
    state::app_data::SoundConfig,
    tts::{GenerateRequest, GenerateResponse, Voice},
};
use anyhow::{anyhow, Context};
use deno_core::op2;
use serde::Deserialize;
use tokio::sync::oneshot;
use uuid::Uuid;

#[op2(async)]
#[string]
pub async fn op_vtftk_play_sound(#[string] src: String, volume: f32) -> anyhow::Result<()> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        sender
            .send(JsEventMessage::PlaySound {
                config: SoundConfig {
                    id: Uuid::new_v4(),
                    name: "<internal>".to_string(),
                    src,
                    volume,
                },
            })
            .await
            .context("failed to send event")?;

        Ok(())
    } else {
        Err(anyhow!("no event producer is available"))
    }
}

#[derive(Debug, Deserialize)]
pub struct SoundSeq {
    pub src: String,
    pub volume: f32,
}

#[op2(async)]
#[string]
pub async fn op_vtftk_play_sound_seq(#[serde] seq: Vec<SoundSeq>) -> anyhow::Result<()> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        sender
            .send(JsEventMessage::PlaySoundSeq {
                configs: seq
                    .into_iter()
                    .map(|seq| SoundConfig {
                        id: Uuid::new_v4(),
                        name: "<internal>".to_string(),
                        src: seq.src,
                        volume: seq.volume,
                    })
                    .collect(),
            })
            .await
            .context("failed to send event")?;

        Ok(())
    } else {
        Err(anyhow!("no event producer is available"))
    }
}

#[op2(async)]
#[serde]
pub async fn op_vtftk_tts_generate_parsed(
    #[string] message: String,
) -> anyhow::Result<Vec<String>> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::TtsGenerateParsed {
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
#[serde]
pub async fn op_vtftk_tts_get_voices() -> anyhow::Result<Vec<Voice>> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::TtsGetVoices { return_tx: tx })
            .await
            .context("failed to send event")?;

        rx.await.context("event producer is closed")?
    } else {
        Err(anyhow!("no event producer is available"))
    }
}

#[op2(async)]
#[serde]
pub async fn op_vtftk_tts_generate(
    #[serde] request: GenerateRequest,
) -> anyhow::Result<GenerateResponse> {
    if let Some(sender) = &mut *SCRIPT_EVENT_PRODUCER.lock().await {
        let (tx, rx) = oneshot::channel();
        sender
            .send(JsEventMessage::TtsGenerate {
                request,
                return_tx: tx,
            })
            .await
            .context("failed to send event")?;

        rx.await.context("event producer is closed")?
    } else {
        Err(anyhow!("no event producer is available"))
    }
}
