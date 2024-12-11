use crate::{
    database::entity::{script_events::ScriptEvent, SoundModel},
    events::{outcome::create_throwable_config, EventMessage},
    script::runtime::ScriptExecutorHandle,
};
use sea_orm::DatabaseConnection;
use tauri::State;
use tokio::sync::broadcast;
use uuid::Uuid;

use super::CmdResult;

/// Plays a test throw item event
#[tauri::command]
pub async fn test_throw(
    item_ids: Vec<Uuid>,
    amount: Option<u32>,
    db: State<'_, DatabaseConnection>,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> CmdResult<()> {
    let db = db.inner();
    let config = create_throwable_config(db, &item_ids).await?;

    event_sender.send(EventMessage::ThrowItem {
        config,
        amount: amount.unwrap_or_default(),
    })?;

    Ok(())
}

/// Plays a test throw item event
#[tauri::command]
pub async fn test_throw_barrage(
    item_ids: Vec<Uuid>,
    amount_per_throw: u32,
    amount: u32,
    frequency: u32,
    db: State<'_, DatabaseConnection>,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> CmdResult<()> {
    let db = db.inner();
    let config = create_throwable_config(db, &item_ids).await?;

    event_sender.send(EventMessage::ThrowItemBarrage {
        config,
        amount_per_throw,
        amount,
        frequency,
    })?;

    Ok(())
}

/// Plays a test sound event
#[tauri::command]
pub fn test_sound(
    config: SoundModel,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> Result<bool, ()> {
    event_sender
        .send(EventMessage::PlaySound { config })
        .map_err(|_| ())?;

    Ok(true)
}

/// Test execution of a script to obtain the script list of subscribed events
#[tauri::command]
pub async fn test_get_script_events(
    script: String,
    script_handle: tauri::State<'_, ScriptExecutorHandle>,
) -> CmdResult<Vec<ScriptEvent>> {
    let events = script_handle.get_events(script).await?;
    Ok(events)
}
