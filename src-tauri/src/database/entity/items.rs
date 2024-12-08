use sea_orm::{entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel};
use serde::{Deserialize, Serialize};

use super::{
    links::ItemImpactSounds, shared::DbResult, ItemImpactSoundsActiveModel, ItemImpactSoundsColumn,
    ItemImpactSoundsEntity, SoundModel,
};

// Type alias helpers for the database entity types
pub type ItemModel = Model;
pub type ItemEntity = Entity;
pub type ItemActiveModel = ActiveModel;
pub type ItemColumn = Column;

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
}

/// Configuration for a throwable image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ThrowableImageConfig {
    /// Src URL for the image
    pub src: String,
    /// Weight of impact the image has
    pub weight: u32,
    /// Scale of the image 0-1
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
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub image: ThrowableImageConfig,
    pub impact_sounds: Vec<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ItemWithImpactSounds {
    #[serde(flatten)]
    pub item: ItemModel,
    pub impact_sounds: Vec<SoundModel>,
}

impl Model {
    /// Create a new item
    pub async fn create<C>(db: &C, create: CreateItem) -> DbResult<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(create.name),
            image: Set(create.image),
        };

        let model = active_model.insert(db).await?;

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
    pub async fn get_by_ids<C>(db: &C, id: &[Uuid]) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .filter(Column::Id.is_in(id.iter().copied()))
            .all(db)
            .await
    }

    /// Find all items
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find().all(db).await
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

        let this = this.update(db).await?;

        if let Some(impact_sounds) = data.impact_sounds {
            this.set_impact_sounds(db, &impact_sounds).await?;
        }

        Ok(this)
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
