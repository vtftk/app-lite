use anyhow::Context;
use futures::{future::BoxFuture, stream::FuturesUnordered, TryStreamExt};
use sea_orm::{entity::prelude::*, ActiveValue::Set, UpdateResult};
use serde::{Deserialize, Serialize};

use super::shared::{DbResult, UpdateOrdering};

// Type alias helpers for the database entity types
pub type ItemCollectionItemModel = Model;
pub type ItemCollectionItemEntity = Entity;
pub type ItemCollectionItemActiveModel = ActiveModel;
pub type ItemCollectionItemColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "item_collection_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub item_id: Uuid,
    #[sea_orm(primary_key)]
    pub item_collection_id: Uuid,

    /// Ordering
    pub order: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::items::Entity",
        from = "Column::ItemId",
        to = "super::items::Column::Id"
    )]
    Item,
    #[sea_orm(
        belongs_to = "super::item_collections::Entity",
        from = "Column::ItemCollectionId",
        to = "super::item_collections::Column::Id"
    )]
    Collection,
}

impl Related<super::item_collections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collection.def()
    }
}

impl Related<super::items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateItemCollectionItem {
    pub item_id: Uuid,
    pub item_collection_id: Uuid,
}

impl Model {
    pub async fn create<C>(db: &C, create: CreateItemCollectionItem) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            item_collection_id: Set(create.item_collection_id),
            item_id: Set(create.item_id),
            order: Set(0),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, create.item_id, create.item_collection_id)
            .await?
            .context("model was not inserted")?;

        Ok(model)
    }

    pub async fn get_by_id<C>(
        db: &C,
        item_id: Uuid,
        item_collection_id: Uuid,
    ) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id((item_id, item_collection_id))
            .one(db)
            .await
    }

    pub async fn update_order<C>(
        db: &C,
        item_collection_id: Uuid,
        data: Vec<UpdateOrdering>,
    ) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let _results: Result<Vec<UpdateResult>, DbErr> = data
            .into_iter()
            .map(|data| -> BoxFuture<'_, DbResult<UpdateResult>> {
                Box::pin(
                    Entity::update_many()
                        .filter(
                            Column::ItemId
                                .eq(data.id)
                                .and(Column::ItemCollectionId.eq(item_collection_id)),
                        )
                        .col_expr(Column::Order, data.order.into())
                        .exec(db),
                )
            })
            .collect::<FuturesUnordered<BoxFuture<'_, DbResult<UpdateResult>>>>()
            .try_collect()
            .await;

        Ok(())
    }
}
