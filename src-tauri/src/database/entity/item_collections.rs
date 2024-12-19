use anyhow::Context;
use futures::{future::BoxFuture, stream::FuturesUnordered, TryStreamExt};
use sea_orm::{entity::prelude::*, ActiveValue::Set, IntoActiveModel, QueryOrder, UpdateResult};
use serde::{Deserialize, Serialize};

use super::{
    item_collection_items::{
        ItemCollectionItemActiveModel, ItemCollectionItemColumn, ItemCollectionItemEntity,
    },
    links::ItemCollectionItems,
    shared::{DbResult, UpdateOrdering},
    ItemModel,
};

// Type alias helpers for the database entity types
pub type ItemCollectionModel = Model;
pub type ItemCollectionEntity = Entity;
pub type ItemCollectionActiveModel = ActiveModel;
pub type ItemCollectionColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "item_collections")]
pub struct Model {
    /// Unique ID for the item
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Name of the throwable item
    pub name: String,
    /// Ordering
    pub order: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Item can have many impact sounds
    #[sea_orm(has_many = "super::item_collection_items::Entity")]
    CollectionItems,
}

impl Related<super::item_collection_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CollectionItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Data for updating an item
#[derive(Default, Deserialize)]
pub struct UpdateItemCollection {
    pub name: Option<String>,
    pub order: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemCollection {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ItemCollectionWithItems {
    pub collection: ItemCollectionModel,
    pub items: Vec<ItemModel>,
}

impl Model {
    /// Create a new item
    pub async fn create<C>(db: &C, create: CreateItemCollection) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            name: Set(create.name),
            order: Set(0),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;

        Ok(model)
    }

    pub async fn all_with_items<C>(db: &C) -> DbResult<Vec<ItemCollectionWithItems>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Ok(Entity::find()
            .find_with_linked(ItemCollectionItems)
            .order_by_asc(Column::Order)
            .all(db)
            .await?
            .into_iter()
            .map(|(collection, mut items)| {
                // Order items
                items.sort_by(|a, b| a.order.cmp(&b.order));
                ItemCollectionWithItems { collection, items }
            })
            .collect())
    }

    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn update<C>(self, db: &C, data: UpdateItemCollection) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        if let Some(name) = data.name {
            this.name = Set(name);
        }

        this.order = data.order.map(Set).unwrap_or(this.order);

        let this = this.update(db).await?;

        Ok(this)
    }

    pub async fn update_order<C>(db: &C, data: Vec<UpdateOrdering>) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let _results: Result<Vec<UpdateResult>, DbErr> = data
            .into_iter()
            .map(|data| -> BoxFuture<'_, DbResult<UpdateResult>> {
                Box::pin(
                    Entity::update_many()
                        .filter(Column::Id.eq(data.id))
                        .col_expr(Column::Order, data.order.into())
                        .exec(db),
                )
            })
            .collect::<FuturesUnordered<BoxFuture<'_, DbResult<UpdateResult>>>>()
            .try_collect()
            .await;

        Ok(())
    }

    pub async fn get_items<C>(&self, db: &C) -> DbResult<Vec<ItemModel>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        self.find_linked(ItemCollectionItems)
            .order_by_asc(Column::Order)
            .all(db)
            .await
            .map(|mut items| {
                // Order items
                items.sort_by(|a, b| a.order.cmp(&b.order));
                items
            })
    }

    pub async fn set_items<C>(&self, db: &C, item_ids: &[Uuid]) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // Delete any impact sounds not in the provided list
        ItemCollectionItemEntity::delete_many()
            .filter(
                ItemCollectionItemColumn::ItemId
                    .eq(self.id)
                    .and(ItemCollectionItemColumn::ItemId.is_not_in(item_ids.iter().copied())),
            )
            .exec(db)
            .await?;

        self.append_items(db, item_ids).await?;

        Ok(())
    }

    pub async fn append_items<C>(&self, db: &C, item_ids: &[Uuid]) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // Insert the new connections
        ItemCollectionItemEntity::insert_many(item_ids.iter().enumerate().map(
            |(index, item_id)| ItemCollectionItemActiveModel {
                item_collection_id: Set(self.id),
                item_id: Set(*item_id),
                order: Set(index as u32),
            },
        ))
        // Ignore already existing connections
        .on_conflict_do_nothing()
        .exec(db)
        .await?;

        Ok(())
    }
}
