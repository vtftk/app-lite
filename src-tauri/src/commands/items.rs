//! # Items
//!
//! Commands for interacting with items from the frontend

use super::CmdResult;
use crate::{
    database::entity::{
        items::{CreateItem, ItemModel, ItemWithImpactSounds, UpdateItem},
        shared::UpdateOrdering,
        sounds::SoundModel,
    },
    storage::Storage,
};
use anyhow::Context;
use sea_orm::{DatabaseConnection, ModelTrait};
use tauri::State;
use uuid::Uuid;

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
) -> CmdResult<ItemWithImpactSounds> {
    let db = db.inner();
    let item = ItemModel::create(db, create).await?;
    let impact_sounds = item.get_impact_sounds(db).await?;

    Ok(ItemWithImpactSounds {
        item,
        impact_sounds,
    })
}

/// Update an existing item
#[tauri::command]
pub async fn update_item(
    item_id: Uuid,
    update: UpdateItem,
    db: State<'_, DatabaseConnection>,
    storage: State<'_, Storage>,
) -> CmdResult<ItemWithImpactSounds> {
    let db = db.inner();
    let item = ItemModel::get_by_id(db, item_id)
        .await?
        .context("item not found")?;

    let original_item_url = item.image.src.clone();

    let item = item.update(db, update).await?;
    let impact_sounds = item.get_impact_sounds(db).await?;

    // Delete previous image file when changed
    if item.image.src != original_item_url {
        storage.try_delete_file(original_item_url).await?;
    }

    Ok(ItemWithImpactSounds {
        item,
        impact_sounds,
    })
}

/// Updates the list orderings of items using the provided orderings
#[tauri::command]
pub async fn update_item_orderings(
    update: Vec<UpdateOrdering>,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();
    ItemModel::update_order(db, update).await?;

    Ok(())
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
pub async fn delete_item(
    item_id: Uuid,
    db: State<'_, DatabaseConnection>,
    storage: State<'_, Storage>,
) -> CmdResult<()> {
    let db = db.inner();
    let item = ItemModel::get_by_id(db, item_id)
        .await?
        .context("item not found")?;

    let item_url = item.image.src.clone();

    item.delete(db).await?;

    storage.try_delete_file(item_url).await?;

    Ok(())
}
