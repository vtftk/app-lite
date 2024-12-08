use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

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

impl Model {}
