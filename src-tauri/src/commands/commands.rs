//! # Commands
//!
//! Commands for interacting with commands from the frontend

use crate::database::entity::{
    commands::{CreateCommand, UpdateCommand},
    CommandModel,
};
use anyhow::Context;
use sea_orm::{DatabaseConnection, ModelTrait};
use tauri::State;
use uuid::Uuid;

use super::CmdResult;

/// Get all commands
#[tauri::command]
pub async fn get_commands(db: State<'_, DatabaseConnection>) -> CmdResult<Vec<CommandModel>> {
    let db = db.inner();
    let commands = CommandModel::all(db).await?;
    Ok(commands)
}

/// Get a specific command by ID
#[tauri::command]
pub async fn get_command_by_id(
    command_id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<Option<CommandModel>> {
    let db = db.inner();
    let command = CommandModel::get_by_id(db, command_id).await?;
    Ok(command)
}

/// Create a new command
#[tauri::command]
pub async fn create_command(
    create: CreateCommand,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<CommandModel> {
    let db = db.inner();
    let command = CommandModel::create(db, create).await?;
    Ok(command)
}

/// Update an existing command
#[tauri::command]
pub async fn update_command(
    command_id: Uuid,
    update: UpdateCommand,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<CommandModel> {
    let db = db.inner();
    let command = CommandModel::get_by_id(db, command_id)
        .await?
        .context("command not found")?;
    let command = command.update(db, update).await?;
    Ok(command)
}

/// Delete a command
#[tauri::command]
pub async fn delete_command(command_id: Uuid, db: State<'_, DatabaseConnection>) -> CmdResult<()> {
    let db = db.inner();
    let command = CommandModel::get_by_id(db, command_id)
        .await?
        .context("command not found")?;
    command.delete(db).await?;
    Ok(())
}
