//! # Scripts
//!
//! Commands for interacting with scripts from the frontend

use crate::database::entity::{
    script_logs::ScriptLogsModel,
    scripts::{CreateScript, UpdateScript, UpdateScriptOrdering},
    shared::LogsQuery,
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

/// Get logs of a script
#[tauri::command]
pub async fn get_script_logs(
    script_id: Uuid,
    query: LogsQuery,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<Vec<ScriptLogsModel>> {
    let db = db.inner();
    let script = ScriptModel::get_by_id(db, script_id)
        .await?
        .context("script not found")?;
    let logs = script.get_logs(db, query).await?;

    Ok(logs)
}

#[tauri::command]
pub async fn delete_script_logs(
    log_ids: Vec<Uuid>,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();

    ScriptLogsModel::delete_many(db, &log_ids).await?;

    Ok(())
}

#[tauri::command]
pub async fn update_script_orderings(
    update: Vec<UpdateScriptOrdering>,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();
    ScriptModel::update_order(db, update).await?;

    Ok(())
}
