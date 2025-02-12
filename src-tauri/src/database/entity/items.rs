use super::{
    items_sounds::{
        ItemsSoundsActiveModel, ItemsSoundsColumn, ItemsSoundsEntity, ItemsSoundsModel, SoundType,
    },
    shared::{DbResult, UpdateOrdering},
};
use anyhow::Context;
use chrono::Utc;
use futures::{future::BoxFuture, stream::FuturesUnordered, TryStreamExt};
use sea_orm::{
    entity::prelude::*, sea_query::CaseStatement, ActiveValue::Set, FromJsonQueryResult,
    IntoActiveModel, QueryOrder, UpdateResult,
};
use serde::{Deserialize, Serialize};

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
    pub config: ItemConfig,
    /// Ordering
    pub order: u32,
    // Date time of creation
    pub created_at: DateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ItemConfig {
    pub image: ItemImageConfig,
    #[serde(default)]
    pub windup: ItemWindupConfig,
}

/// Configuration for a throwable image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ItemImageConfig {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(default)]
pub struct ItemWindupConfig {
    /// Whether a windup is enabled
    pub enabled: bool,
    /// Duration of the windup
    pub duration: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Item can have many impact sounds
    #[sea_orm(has_many = "super::items_sounds::Entity")]
    ImpactSounds,
}

impl Related<super::items_sounds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImpactSounds.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Data for updating an item
#[derive(Default, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub config: Option<ItemConfig>,
    pub impact_sounds: Option<Vec<Uuid>>,
    pub windup_sounds: Option<Vec<Uuid>>,
    pub order: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub config: ItemConfig,
    pub impact_sounds: Vec<Uuid>,
    pub windup_sounds: Vec<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ItemWithSounds {
    #[serde(flatten)]
    pub item: ItemModel,
    pub impact_sounds: Vec<Uuid>,
    pub windup_sounds: Vec<Uuid>,
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
            config: Set(create.config),
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
            .append_sounds(db, &create.impact_sounds, SoundType::Impact)
            .await?;

        model
            .append_sounds(db, &create.windup_sounds, SoundType::Windup)
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
    pub async fn get_by_ids_with_sounds<C>(
        db: &C,
        ids: &[Uuid],
    ) -> DbResult<Vec<(Self, Vec<ItemsSoundsModel>)>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .find_with_related(super::items_sounds::Entity)
            .filter(Column::Id.is_in(ids.iter().copied()))
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

        this.name = data.name.map(Set).unwrap_or(this.name);
        this.config = data.config.map(Set).unwrap_or(this.config);
        this.order = data.order.map(Set).unwrap_or(this.order);

        let this = this.update(db).await?;

        if let Some(impact_sounds) = data.impact_sounds {
            this.set_sounds(db, &impact_sounds, SoundType::Impact)
                .await?;
        }

        if let Some(windup_sounds) = data.windup_sounds {
            this.set_sounds(db, &windup_sounds, SoundType::Windup)
                .await?;
        }

        Ok(this)
    }

    pub async fn update_order<C>(db: &C, data: Vec<UpdateOrdering>) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        data.chunks(1000)
            .map(|order_chunk| -> BoxFuture<'_, DbResult<UpdateResult>> {
                let mut case = CaseStatement::new()
                    // Use the current column value when not specified
                    .finally(Expr::col(Column::Order));

                // Add case for all updated values
                for order in order_chunk {
                    case = case.case(Expr::col(Column::Id).eq(order.id), Expr::value(order.order));
                }

                Box::pin(
                    Entity::update_many()
                        .col_expr(Column::Order, case.into())
                        .exec(db),
                )
            })
            .collect::<FuturesUnordered<BoxFuture<'_, DbResult<UpdateResult>>>>()
            .try_collect::<Vec<UpdateResult>>()
            .await?;

        Ok(())
    }

    /// Sets the impact sounds for thsis item
    pub async fn set_sounds<C>(
        &self,
        db: &C,
        sound_ids: &[Uuid],
        sound_type: SoundType,
    ) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // Delete any impact sounds not in the provided list
        ItemsSoundsEntity::delete_many()
            .filter(
                ItemsSoundsColumn::ItemId
                    .eq(self.id)
                    .and(ItemsSoundsColumn::SoundId.is_not_in(sound_ids.iter().copied()))
                    .and(ItemsSoundsColumn::SoundType.eq(sound_type)),
            )
            .exec(db)
            .await?;

        self.append_sounds(db, sound_ids, sound_type).await?;

        Ok(())
    }

    /// Append impact sounds to the item
    pub async fn append_sounds<C>(
        &self,
        db: &C,
        sound_ids: &[Uuid],
        sound_type: SoundType,
    ) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // Insert the new connections
        ItemsSoundsEntity::insert_many(sound_ids.iter().map(|sound_id| ItemsSoundsActiveModel {
            item_id: Set(self.id),
            sound_id: Set(*sound_id),
            sound_type: Set(sound_type),
        }))
        // Ignore already existing connections
        .on_conflict_do_nothing()
        .exec(db)
        .await?;

        Ok(())
    }

    /// Finds all sounds connected to this item
    pub async fn with_sounds<C>(self, db: &C) -> DbResult<ItemWithSounds>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let sounds = self
            .find_related(super::items_sounds::Entity)
            .all(db)
            .await?;

        let mut impact_sounds = Vec::new();
        let mut windup_sounds = Vec::new();

        for sound in sounds {
            match sound.sound_type {
                SoundType::Impact => impact_sounds.push(sound.sound_id),
                SoundType::Windup => windup_sounds.push(sound.sound_id),
            }
        }

        Ok(ItemWithSounds {
            item: self,
            impact_sounds,
            windup_sounds,
        })
    }
}
