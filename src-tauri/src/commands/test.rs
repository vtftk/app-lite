use tokio::sync::broadcast;

use crate::{
    events::EventMessage,
    script::runtime::get_script_events,
    state::app_data::{SoundConfig, ThrowableConfig},
};

use super::CmdResult;

/// Plays a test throw item event
#[tauri::command]
pub fn test_throw(
    config: ThrowableConfig,
    amount: Option<u32>,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> Result<bool, ()> {
    event_sender
        .send(EventMessage::ThrowItem {
            config,
            amount: amount.unwrap_or_default(),
        })
        .map_err(|_| ())?;

    Ok(true)
}

/// Plays a test throw item event
#[tauri::command]
pub fn test_throw_barrage(
    config: ThrowableConfig,
    amount_per_throw: u32,
    amount: u32,
    frequency: u32,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> Result<bool, ()> {
    event_sender
        .send(EventMessage::ThrowItemBarrage {
            config,
            amount_per_throw,
            amount,
            frequency,
        })
        .map_err(|_| ())?;

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

/// Test execution of a script to obtain the script list of subscribed events
#[tauri::command]
pub fn test_get_script_events(script: String) -> CmdResult<Vec<String>> {
    let events = get_script_events(script)?;
    Ok(events)
}
