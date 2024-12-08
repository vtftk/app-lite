//! # Scripts
//!
//! Commands for interacting with scripts from the frontend

use crate::database::entity::{
    scripts::{CreateScript, UpdateScript},
    ScriptModel,
};
use anyhow::Context;
use sea_orm::{DatabaseConnection, ModelTrait};
use tauri::State;
use uuid::Uuid;

use super::CmdResult;

/// Get all scripts
#[tauri::command]
pub async fn get_scripts(db: State<'_, DatabaseConnection>) -> CmdResult<Vec<ScriptModel>> {
    let db = db.inner();
    let scripts = ScriptModel::all(db).await?;
    Ok(scripts)
}

/// Get a specific script by ID
#[tauri::command]
pub async fn get_script_by_id(
    script_id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<Option<ScriptModel>> {
    let db = db.inner();
    let script = ScriptModel::get_by_id(db, script_id).await?;
    Ok(script)
}

/// Create a new script
#[tauri::command]
pub async fn create_script(
    create: CreateScript,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<ScriptModel> {
    let db = db.inner();
    let script = ScriptModel::create(db, create).await?;
    Ok(script)
}

/// Update an existing script
#[tauri::command]
pub async fn update_script(
    script_id: Uuid,
    update: UpdateScript,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<ScriptModel> {
    let db = db.inner();
    let script = ScriptModel::get_by_id(db, script_id)
        .await?
        .context("script not found")?;
    let script = script.update(db, update).await?;
    Ok(script)
}

/// Delete a script
#[tauri::command]
pub async fn delete_script(script_id: Uuid, db: State<'_, DatabaseConnection>) -> CmdResult<()> {
    let db = db.inner();
    let script = ScriptModel::get_by_id(db, script_id)
        .await?
        .context("script not found")?;
    script.delete(db).await?;
    Ok(())
}
