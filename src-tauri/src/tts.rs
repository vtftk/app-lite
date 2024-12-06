use anyhow::Context;
use log::debug;
use reqwest::header;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::app_data::AppDataStore;

#[derive(Debug, Deserialize, Serialize)]
pub struct Voice {
    pub voice_id: Uuid,
    pub name: String,
    pub sample: String,
}

#[derive(Debug, Deserialize)]
pub struct VoicesResponse {
    pub voices: Vec<Voice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub voice_id: Uuid,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateResponse {
    pub status: u16,
    pub url: String,
}

/// Request the TTS monster API to generate TTS
pub async fn tts_monster_get_voices(app_data: AppDataStore) -> anyhow::Result<Vec<Voice>> {
    let token = {
        let app_data = app_data.read().await;
        app_data
            .externals_config
            .tts_monster_api_key
            .as_ref()
            .cloned()
            .context("missing tts monster api key")?
    };

    let client = reqwest::Client::new();

    let response = client
        .post("https://api.console.tts.monster/voices")
        .header(header::AUTHORIZATION, token)
        .send()
        .await?;

    let body: VoicesResponse = response.json().await?;
    Ok(body.voices)
}

/// Request the TTS monster API to generate TTS
pub async fn tts_monster_generate(
    app_data: AppDataStore,
    request: GenerateRequest,
) -> anyhow::Result<GenerateResponse> {
    let token = {
        let app_data = app_data.read().await;
        app_data
            .externals_config
            .tts_monster_api_key
            .as_ref()
            .cloned()
            .context("missing tts monster api key")?
    };

    let client = reqwest::Client::new();

    let response = client
        .post("https://api.console.tts.monster/generate")
        .header(header::AUTHORIZATION, token)
        .json(&request)
        .send()
        .await?;

    let body: GenerateResponse = response.json().await?;
    Ok(body)
}

fn parse_message(message: &str) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut chars = message.chars().peekable();

    while let Some(ch) = chars.next() {
        // Ignore leading whitespace
        if ch.is_whitespace() {
            continue;
        }

        let name: String = if ch == '(' {
            let mut name = String::new();

            for ch in chars.by_ref() {
                if ch == ')' {
                    break;
                }

                name.push(ch);
            }

            name
        } else {
            "unknown".to_string()
        };

        let mut message = String::new();

        while let Some(ch) = chars.next_if(|ch| *ch != '(') {
            message.push(ch);
        }

        pairs.push((name, message.trim().to_string()));
    }
    pairs
}

/// Request the TTS monster API to generate TTS
pub async fn tts_monster_generate_parsed(
    app_data: AppDataStore,
    request: String,
) -> anyhow::Result<Vec<String>> {
    let pairs = parse_message(&request);
    let voices = tts_monster_get_voices(app_data.clone()).await?;

    debug!("TTS voice pairs generated: {:?}", pairs);

    let mut generated = Vec::new();

    let default_voice_id = uuid::uuid!("a33aa2c5-47f9-4882-a192-d7aa6a0c0efd");
    let default_voice = voices
        .iter()
        .find(|voice| voice.voice_id.eq(&default_voice_id));

    for (name, message) in pairs {
        let voice = voices
            .iter()
            .find(|voice| name.eq_ignore_ascii_case(&voice.name))
            .or(default_voice);

        debug!("Found voice: {:?}", voice);

        if let Some(voice) = voice {
            let message = tts_monster_generate(
                app_data.clone(),
                GenerateRequest {
                    voice_id: voice.voice_id,
                    message,
                },
            )
            .await?;

            debug!("Generated TTS response: {:?}", message);
            generated.push(message.url);
        }
    }

    Ok(generated)
}

#[cfg(test)]
mod test {
    use super::parse_message;

    #[test]
    fn test_parse_message() {
        let message = "(Wretch) Test message as Wretch (Whisper) Test message as Whisper";
        let pairs = parse_message(message);
        dbg!(pairs);
    }
}
