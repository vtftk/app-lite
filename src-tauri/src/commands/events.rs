//! # Events
//!
//! Commands for interacting with events from the frontend

use crate::database::entity::{
    events::{CreateEvent, UpdateEvent},
    EventModel,
};
use anyhow::Context;
use sea_orm::{DatabaseConnection, ModelTrait};
use tauri::State;
use uuid::Uuid;

use super::CmdResult;

/// Get all events
#[tauri::command]
pub async fn get_events(db: State<'_, DatabaseConnection>) -> CmdResult<Vec<EventModel>> {
    let db = db.inner();
    let events = EventModel::all(db).await?;
    Ok(events)
}

/// Get a specific event by ID
#[tauri::command]
pub async fn get_event_by_id(
    event_id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<Option<EventModel>> {
    let db = db.inner();
    let event = EventModel::get_by_id(db, event_id).await?;
    Ok(event)
}

/// Create a new event
#[tauri::command]
pub async fn create_event(
    create: CreateEvent,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<EventModel> {
    let db = db.inner();
    let event = EventModel::create(db, create).await?;
    Ok(event)
}

/// Update an existing event
#[tauri::command]
pub async fn update_event(
    event_id: Uuid,
    update: UpdateEvent,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<EventModel> {
    let db = db.inner();
    let event = EventModel::get_by_id(db, event_id)
        .await?
        .context("event not found")?;
    let event = event.update(db, update).await?;
    Ok(event)
}

/// Delete a event
#[tauri::command]
pub async fn delete_event(event_id: Uuid, db: State<'_, DatabaseConnection>) -> CmdResult<()> {
    let db = db.inner();
    let event = EventModel::get_by_id(db, event_id)
        .await?
        .context("event not found")?;
    event.delete(db).await?;
    Ok(())
}
