use sea_orm::{entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel};
use serde::{Deserialize, Serialize};

use super::shared::DbResult;

// Type alias helpers for the database entity types
pub type SoundModel = Model;
pub type SoundEntity = Entity;
pub type SoundActiveModel = ActiveModel;
pub type SoundColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sounds")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Name of the sound
    pub name: String,
    /// Src URL for the image
    pub src: String,
    /// Volume of the sound 0-1
    pub volume: f32,
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

#[derive(Debug, Deserialize)]
pub struct CreateSound {
    pub name: String,
    pub src: String,
    pub volume: f32,
}

#[derive(Default, Deserialize)]
pub struct UpdateSound {
    pub name: Option<String>,
    pub src: Option<String>,
    pub volume: Option<f32>,
}

impl Model {
    /// Create a new sound
    pub async fn create<C>(db: &C, create: CreateSound) -> DbResult<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(create.name),
            src: Set(create.src),
            volume: Set(create.volume),
        };

        let model = active_model.insert(db).await?;

        Ok(model)
    }

    /// Find a specific sound by ID
    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// Find all sounds
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find().all(db).await
    }

    /// Update the current sound
    pub async fn update<C>(self, db: &C, data: UpdateSound) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        if let Some(name) = data.name {
            this.name = Set(name);
        }

        if let Some(src) = data.src {
            this.src = Set(src);
        }

        if let Some(volume) = data.volume {
            this.volume = Set(volume);
        }

        let this = this.update(db).await?;
        Ok(this)
    }
}
