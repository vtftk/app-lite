use tokio::sync::broadcast;

use crate::{
    events::EventMessage,
    state::app_data::{SoundConfig, ThrowableConfig},
};

/// Plays a test throw item event
#[tauri::command]
pub fn test_throw(
    config: ThrowableConfig,
    amount: Option<u32>,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> Result<bool, ()> {
    if let Some(amount) = amount {
        event_sender
            .send(EventMessage::ThrowMany { config, amount })
            .map_err(|_| ())?;
    } else {
        event_sender
            .send(EventMessage::Throw { config })
            .map_err(|_| ())?;
    }

    Ok(true)
}

/// Plays a test sound event
#[tauri::command]
pub fn test_sound(
    config: SoundConfig,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> Result<bool, ()> {
    event_sender
        .send(EventMessage::PlaySound { config })
        .map_err(|_| ())?;

    Ok(true)
}
