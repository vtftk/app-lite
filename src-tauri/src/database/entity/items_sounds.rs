use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// Type alias helpers for the database entity types
pub type ItemsSoundsEntity = Entity;
pub type ItemsSoundsModel = Model;
pub type ItemsSoundsActiveModel = ActiveModel;
pub type ItemsSoundsColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "items_sounds")]
pub struct Model {
    /// ID of the item
    #[sea_orm(primary_key)]
    pub item_id: Uuid,
    /// ID of the sound
    #[sea_orm(primary_key)]
    pub sound_id: Uuid,
    /// Type of sound
    #[sea_orm(primary_key)]
    pub sound_type: SoundType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum SoundType {
    #[sea_orm(string_value = "Impact")]
    Impact,
    #[sea_orm(string_value = "Windup")]
    Windup,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Item half of the relationship
    #[sea_orm(
        belongs_to = "super::items::Entity",
        from = "Column::ItemId",
        to = "super::items::Column::Id"
    )]
    Item,
    /// Sound half of the relationship
    #[sea_orm(
        belongs_to = "super::sounds::Entity",
        from = "Column::SoundId",
        to = "super::sounds::Column::Id"
    )]
    Sound,
}

impl Related<super::items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl Related<super::sounds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sound.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {}
