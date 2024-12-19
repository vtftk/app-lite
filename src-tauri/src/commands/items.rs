//! # Items
//!
//! Commands for interacting with items from the frontend

use crate::database::entity::{
    item_collection_items::ItemCollectionItemModel,
    item_collections::{
        CreateItemCollection, ItemCollectionModel, ItemCollectionWithItemIds,
        ItemCollectionWithItems,
    },
    items::{CreateItem, ItemWithImpactSounds, UpdateItem, UpdateItemOrdering},
    shared::UpdateOrdering,
    ItemModel, SoundModel,
};
use anyhow::Context;
use sea_orm::{DatabaseConnection, ModelTrait};
use tauri::{AppHandle, State};
use uuid::Uuid;

use super::{data::delete_src_file, CmdResult};

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
) -> CmdResult<ItemWithImpactSounds> {
    let db = db.inner();
    let item = ItemModel::get_by_id(db, item_id)
        .await?
        .context("item not found")?;
    let item = item.update(db, update).await?;
    let impact_sounds = item.get_impact_sounds(db).await?;

    Ok(ItemWithImpactSounds {
        item,
        impact_sounds,
    })
}

#[tauri::command]
pub async fn update_item_orderings(
    update: Vec<UpdateItemOrdering>,
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
    app_handle: AppHandle,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();
    let item = ItemModel::get_by_id(db, item_id)
        .await?
        .context("item not found")?;

    let item_url = item.image.src.clone();

    item.delete(db).await?;

    delete_src_file(item_url, app_handle).await?;

    Ok(())
}

/// Create a new item collection
#[tauri::command]
pub async fn create_item_collection(
    create: CreateItemCollection,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<ItemCollectionWithItemIds> {
    let db = db.inner();
    let collection = ItemCollectionModel::create(db, create).await?;

    Ok(ItemCollectionWithItemIds {
        collection,
        items: vec![],
    })
}

/// Get all item collections
#[tauri::command]
pub async fn get_item_collections(
    db: State<'_, DatabaseConnection>,
) -> CmdResult<Vec<ItemCollectionWithItemIds>> {
    let db = db.inner();
    let collections = ItemCollectionModel::all_with_items(db).await?;
    Ok(collections)
}

/// Get a specific item collection and its items
#[tauri::command]
pub async fn get_item_collection(
    id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<ItemCollectionWithItems> {
    let db = db.inner();
    let collection = ItemCollectionModel::get_by_id(db, id)
        .await?
        .context("unknown collection")?;

    let items = collection.get_items(db).await?;

    Ok(ItemCollectionWithItems { collection, items })
}

/// Update ordering of item collections
#[tauri::command]
pub async fn update_item_collection_orderings(
    update: Vec<UpdateOrdering>,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();
    ItemCollectionModel::update_order(db, update).await?;

    Ok(())
}

/// Update ordering of items within an item collection
#[tauri::command]
pub async fn update_item_collection_item_orderings(
    id: Uuid,
    update: Vec<UpdateOrdering>,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();
    ItemCollectionItemModel::update_order(db, id, update).await?;
    Ok(())
}

/// Set the items within an item collection
#[tauri::command]
pub async fn set_item_collection_items(
    id: Uuid,
    items: Vec<Uuid>,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();
    let collection = ItemCollectionModel::get_by_id(db, id)
        .await?
        .context("item not found")?;
    collection.set_items(db, &items).await?;
    Ok(())
}

/// Adds items to an item collection
#[tauri::command]
pub async fn append_item_collection_items(
    id: Uuid,
    items: Vec<Uuid>,
    db: State<'_, DatabaseConnection>,
) -> CmdResult<()> {
    let db = db.inner();
    let collection = ItemCollectionModel::get_by_id(db, id)
        .await?
        .context("item not found")?;
    collection.append_items(db, &items).await?;
    Ok(())
}

/// Delete an item collection
#[tauri::command]
pub async fn delete_item_collection(id: Uuid, db: State<'_, DatabaseConnection>) -> CmdResult<()> {
    let db = db.inner();
    let collection = ItemCollectionModel::get_by_id(db, id)
        .await?
        .context("item not found")?;

    collection.delete(db).await?;

    Ok(())
}
