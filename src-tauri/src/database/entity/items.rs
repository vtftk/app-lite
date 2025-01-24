use anyhow::Context;
use chrono::Utc;
use futures::{future::BoxFuture, stream::FuturesUnordered, TryStreamExt};
use sea_orm::{
    entity::prelude::*, sea_query::Func, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel,
    QueryOrder, UpdateResult,
};
use serde::{Deserialize, Serialize};

use super::{
    items_impact_sounds::{
        ItemImpactSoundsActiveModel, ItemImpactSoundsColumn, ItemImpactSoundsEntity,
    },
    links::ItemImpactSounds,
    shared::{DbResult, UpdateOrdering},
    sounds::SoundModel,
};

// Type alias helpers for the database entity types
pub type ItemModel = Model;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    /// Unique ID for the item
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Name of the throwable item
    pub name: String,
    /// Image to use for the throwable item
    pub image: ThrowableImageConfig,
    /// Ordering
    pub order: u32,
    // Date time of creation
    pub created_at: DateTimeUtc,
}

/// Configuration for a throwable image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ThrowableImageConfig {
    /// Src URL for the image
    pub src: String,
    /// Weight of impact the image has
    pub weight: f32,
    /// Scale of the image
    pub scale: f32,
    /// Whether to allow pixelation when rendering at a
    /// different scale
    pub pixelate: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Item can have many impact sounds
    #[sea_orm(has_many = "super::items_impact_sounds::Entity")]
    ImpactSounds,
}

impl Related<super::items_impact_sounds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImpactSounds.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Data for updating an item
#[derive(Default, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub image: Option<ThrowableImageConfig>,
    pub impact_sounds: Option<Vec<Uuid>>,
    pub order: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub image: ThrowableImageConfig,
    pub impact_sounds: Vec<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ItemWithSounds {
    #[serde(flatten)]
    pub item: ItemModel,
    pub impact_sounds: Vec<SoundModel>,
}

impl Model {
    /// Create a new item
    pub async fn create<C>(db: &C, create: CreateItem) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            name: Set(create.name),
            image: Set(create.image),
            order: Set(0),
            created_at: Set(Utc::now()),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;

        model
            .append_impact_sounds(db, &create.impact_sounds)
            .await?;

        Ok(model)
    }

    /// Find a specific item by ID
    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// Find items with IDs present in the provided list
    pub async fn get_by_ids_with_impact_sounds<C>(
        db: &C,
        id: &[Uuid],
    ) -> DbResult<Vec<(Self, Vec<super::items_impact_sounds::Model>)>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .filter(Column::Id.is_in(id.iter().copied()))
            .find_with_related(super::items_impact_sounds::Entity)
            .all(db)
            .await
    }

    /// Find items with names present in the provided list
    pub async fn get_by_names_with_impact_sounds<C>(
        db: &C,
        names: &[String],
        ignore_case: bool,
    ) -> DbResult<Vec<(Self, Vec<super::items_impact_sounds::Model>)>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut select = Entity::find();

        if ignore_case {
            select = select.filter(
                // Convert stored name to lower case
                Expr::expr(Func::lower(Expr::col(Column::Name)))
                    // Compare with lowercase value
                    .is_in(names.iter().map(|value| value.to_lowercase())),
            )
        } else {
            select = select.filter(Column::Name.is_in(names))
        }

        select
            .find_with_related(super::items_impact_sounds::Entity)
            .all(db)
            .await
    }

    /// Find all items
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .order_by_asc(Column::Order)
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// Update the current item
    pub async fn update<C>(self, db: &C, data: UpdateItem) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        if let Some(name) = data.name {
            this.name = Set(name);
        }

        if let Some(image) = data.image {
            this.image = Set(image);
        }

        this.order = data.order.map(Set).unwrap_or(this.order);

        let this = this.update(db).await?;

        if let Some(impact_sounds) = data.impact_sounds {
            this.set_impact_sounds(db, &impact_sounds).await?;
        }

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

    /// Sets the impact sounds for this item
    pub async fn set_impact_sounds<C>(&self, db: &C, impact_sound_ids: &[Uuid]) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // Delete any impact sounds not in the provided list
        ItemImpactSoundsEntity::delete_many()
            .filter(
                ItemImpactSoundsColumn::ItemId.eq(self.id).and(
                    ItemImpactSoundsColumn::SoundId.is_not_in(impact_sound_ids.iter().copied()),
                ),
            )
            .exec(db)
            .await?;

        self.append_impact_sounds(db, impact_sound_ids).await?;

        Ok(())
    }

    /// Append impact sounds to the item
    pub async fn append_impact_sounds<C>(&self, db: &C, impact_sound_ids: &[Uuid]) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // Insert the new connections
        ItemImpactSoundsEntity::insert_many(impact_sound_ids.iter().map(|sound_id| {
            ItemImpactSoundsActiveModel {
                item_id: Set(self.id),
                sound_id: Set(*sound_id),
            }
        }))
        // Ignore already existing connections
        .on_conflict_do_nothing()
        .exec(db)
        .await?;

        Ok(())
    }

    /// Finds all sounds connected to this item
    pub async fn get_impact_sounds<C>(&self, db: &C) -> DbResult<Vec<super::sounds::SoundModel>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let impact_sounds = self.find_linked(ItemImpactSounds).all(db).await?;
        Ok(impact_sounds)
    }
}
