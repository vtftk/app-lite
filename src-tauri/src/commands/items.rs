//! # Items
//!
//! Commands for interacting with items from the frontend

use crate::database::entity::{
    items::{CreateItem, ItemWithImpactSounds, UpdateItem},
    ItemModel, SoundModel,
};
use anyhow::Context;
use sea_orm::{DatabaseConnection, ModelTrait};
use tauri::State;
use uuid::Uuid;

use super::CmdResult;

/// Get all items
#[tauri::command]
pub async fn get_items(db: State<'_, DatabaseConnection>) -> CmdResult<Vec<ItemModel>> {
    let db = db.inner();
    let items = ItemModel::all(db).await?;
    Ok(items)
}

/// Get a specific item by ID, provides both the item itself
/// and any associated impact sounds
#[tauri::command]
pub async fn get_item_by_id(
    item_id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<Option<ItemWithImpactSounds>> {
    let db = db.inner();
    let item = match ItemModel::get_by_id(db, item_id).await? {
        Some(value) => value,
        None => return Ok(None),
    };

    let impact_sounds = item.get_impact_sounds(db).await?;

    Ok(Some(ItemWithImpactSounds {
        item,
        impact_sounds,
    }))
}
/// Get a specific item by ID, provides both the item itself
/// and any associated impact sounds
#[tauri::command]
pub async fn get_item_sounds(
    item_id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<Vec<SoundModel>> {
    let db = db.inner();
    let item = ItemModel::get_by_id(db, item_id)
        .await?
        .context("item not found")?;
    let impact_sounds = item.get_impact_sounds(db).await?;

    Ok(impact_sounds)
}

/// Create a new item
#[tauri::command]
pub async fn create_item(
    create: CreateItem,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<ItemModel> {
    let db = db.inner();
    let item = ItemModel::create(db, create).await?;
    Ok(item)
}

/// Update an existing item
#[tauri::command]
pub async fn update_item(
    item_id: Uuid,
    update: UpdateItem,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<ItemModel> {
    let db = db.inner();
    let item = ItemModel::get_by_id(db, item_id)
        .await?
        .context("item not found")?;
    let item = item.update(db, update).await?;
    Ok(item)
}

/// Add impact sounds to an item
#[tauri::command]
pub async fn append_item_impact_sounds(
    item_id: Uuid,
    sounds: Vec<Uuid>,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();
    let item = ItemModel::get_by_id(db, item_id)
        .await?
        .context("item not found")?;
    item.append_impact_sounds(db, &sounds).await?;
    Ok(())
}

/// Delete an item
#[tauri::command]
pub async fn delete_item(item_id: Uuid, db: State<'_, DatabaseConnection>) -> CmdResult<()> {
    let db = db.inner();
    let item = ItemModel::get_by_id(db, item_id)
        .await?
        .context("item not found")?;
    item.delete(db).await?;
    Ok(())
}
