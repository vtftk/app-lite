//! # TTS Monster Integration
//!
//! Integration with https://tts.monster/ for AI text to speech

use std::{iter::Peekable, str::Chars};

use anyhow::Context;
use log::debug;
use reqwest::header;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Voice from the TTS (https://docs.tts.monster/endpoint/get-voices#param-voices)
#[derive(Debug, Deserialize, Serialize)]
pub struct TTSMonsterVoice {
    pub voice_id: Uuid,
    pub name: String,
    pub sample: String,
}

/// Response containing list of TTS voices (https://docs.tts.monster/endpoint/get-voices)
#[derive(Debug, Deserialize)]
struct GetVoicesResponse {
    voices: Vec<TTSMonsterVoice>,
}

/// Request structure for generating TTS voice messages (https://docs.tts.monster/endpoint/generate-tts)
#[derive(Debug, Serialize, Deserialize)]
struct GenerateRequest {
    voice_id: Uuid,
    message: String,
}

/// Response containing a generated TTS voice message (https://docs.tts.monster/endpoint/generate-tts)
#[derive(Debug, Deserialize, Serialize)]
struct GenerateResponse {
    url: String,
}

/// Error response from the API
#[derive(Debug, Deserialize, Serialize)]
struct ErrorResponse {
    error: String,
}

pub struct TTSMonsterService {}

const VOICES_API_ENDPOINT: &str = "https://api.console.tts.monster/voices";
const GENERATE_API_ENDPOINT: &str = "https://api.console.tts.monster/generate";

impl TTSMonsterService {
    /// Request the list of available voices from the TTS monster API
    ///
    /// `token` is the user authentication token
    pub async fn get_voices(token: &str) -> anyhow::Result<Vec<TTSMonsterVoice>> {
        let client = reqwest::Client::new();

        let response = client
            .post(VOICES_API_ENDPOINT)
            .header(header::AUTHORIZATION, token)
            .send()
            .await?;

        let status = response.status();

        // Handle error response
        if status.is_server_error() || status.is_client_error() {
            let message: ErrorResponse = response
                .json()
                .await
                .context("failed to parse error response")?;

            return Err(anyhow::Error::msg(message.error));
        }

        let body: GetVoicesResponse = response.json().await?;

        Ok(body.voices)
    }

    /// Request the TTS monster API to generate TTS
    ///
    /// Provides the URL for the generated TTS message file
    pub async fn generate(token: &str, voice_id: Uuid, message: String) -> anyhow::Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .post(GENERATE_API_ENDPOINT)
            .header(header::AUTHORIZATION, token)
            .json(&GenerateRequest { voice_id, message })
            .send()
            .await?;

        let status = response.status();

        // Handle error response
        if status.is_server_error() || status.is_client_error() {
            let message: ErrorResponse = response
                .json()
                .await
                .context("failed to parse error response")?;

            return Err(anyhow::Error::msg(message.error));
        }

        let body: GenerateResponse = response.json().await?;
        Ok(body.url)
    }

    /// Generates a TTS voices uses the names and messages parsed from the
    /// provided message "(Name1) This is the message for Name1 (Name2) This is the message for Name2"
    pub async fn generate_parsed(token: &str, message: String) -> anyhow::Result<Vec<String>> {
        let pairs = parse_tts_message(&message);

        // No message was provided
        if pairs.is_empty() {
            return Ok(vec![]);
        }

        debug!("determine tts voice message names: {pairs:?}");

        // Load available voices
        let voices = Self::get_voices(token)
            .await
            .context("failed to obtain available voices")?;

        let mut generated = Vec::new();

        let default_voice_id = uuid::uuid!("a33aa2c5-47f9-4882-a192-d7aa6a0c0efd");
        let default_voice = voices
            .iter()
            .find(|voice| voice.voice_id.eq(&default_voice_id));

        for (name, message) in pairs {
            // Find the requested voice
            let voice = voices
                .iter()
                .find(|voice| name.eq_ignore_ascii_case(&voice.name))
                .or(default_voice);

            let voice = match voice {
                Some(value) => value,
                // Ignore unknown voice
                None => continue,
            };

            debug!("found requested tts voice: ({name}) {voice:?}");

            let url = Self::generate(token, voice.voice_id, message).await?;

            debug!("generated tts response: {:?}", url);

            generated.push(url);
        }

        Ok(generated)
    }
}

/// Parses a user provided TTS message
///
/// Message is a collection of (Name) and messages where (Name)
/// indicates the speaker.
///
/// i.e (Whisper) This is an example message by whisper
pub fn parse_tts_message(message: &str) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut chars = message.chars().peekable();

    loop {
        let name = match next_message_name(&mut chars) {
            // Nothing more to read
            MessageNameResult::EndOfInput => break,

            // Name is unknown
            MessageNameResult::UnknownSpeaker => "Unknown".to_string(),

            // Name is found
            MessageNameResult::Name(name) => name,
        };

        // Collect the remaining message until the next (Name)
        let mut message = String::new();
        while let Some(ch) = chars.next_if(|ch| *ch != '(') {
            message.push(ch);
        }

        // Strip leading and trailing whitespace
        let message = message.trim();

        // Skip empty messages
        if message.is_empty() {
            continue;
        }

        pairs.push((name, message.to_string()));
    }

    pairs
}

/// Result from parsing a message name
enum MessageNameResult {
    /// Reached end of input
    EndOfInput,
    /// Reached message before finding a (Name)
    UnknownSpeaker,
    /// Found a (Name)
    Name(String),
}

/// Attempts to consume the next [MessageNameResult] from the chars iterator
///
/// Used to get the (Name) portion before messages
fn next_message_name(chars: &mut Peekable<Chars>) -> MessageNameResult {
    while let Some(ch) = chars.peek() {
        // Ignore leading whitespace
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        // We hit text, no message name specific
        if *ch != '(' {
            return MessageNameResult::UnknownSpeaker;
        }

        // Consume opening brace
        chars.next();

        // Create the name from all characters until the end of the string
        let mut name: String = String::new();

        for ch in chars.by_ref() {
            if ch == ')' {
                break;
            }

            name.push(ch);
        }

        let name = name.trim();

        if name.is_empty() {
            continue;
        }

        return MessageNameResult::Name(name.to_string());
    }

    MessageNameResult::EndOfInput
}

#[cfg(test)]
mod test {
    use super::parse_tts_message;

    #[test]
    fn test_parse_tts_message() {
        let message = "(Wretch) Test message as Wretch (Whisper) Test message as Whisper";
        let pairs = parse_tts_message(message);
        let expected = vec![
            ("Wretch".to_string(), "Test message as Wretch".to_string()),
            ("Whisper".to_string(), "Test message as Whisper".to_string()),
        ];

        assert_eq!(pairs, expected);
    }

    #[test]
    fn test_parse_tts_message_empty() {
        let message = "";
        let pairs = parse_tts_message(message);
        let expected = vec![];

        assert_eq!(pairs, expected);
    }

    #[test]
    fn test_parse_tts_message_empty_name() {
        let message = "()";
        let pairs = parse_tts_message(message);
        let expected = vec![];

        assert_eq!(pairs, expected);
    }

    #[test]
    fn test_parse_tts_message_with_unknown() {
        let message = "Test (Wretch) Test message as Wretch (Whisper) Test message as Whisper";
        let pairs = parse_tts_message(message);
        let expected = vec![
            ("Unknown".to_string(), "Test".to_string()),
            ("Wretch".to_string(), "Test message as Wretch".to_string()),
            ("Whisper".to_string(), "Test message as Whisper".to_string()),
        ];

        assert_eq!(pairs, expected);
    }
}
