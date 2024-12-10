use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// Type alias helpers for the database entity types
pub type ScriptEventsModel = Model;
pub type ScriptEventsEntity = Entity;
pub type ScriptEventsActiveModel = ActiveModel;
pub type ScriptEventsColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "script_events")]
pub struct Model {
    /// ID of the script
    #[sea_orm(primary_key)]
    pub script_id: Uuid,
    /// Event itself
    #[sea_orm(primary_key)]
    pub event: ScriptEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "camelCase")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ScriptEvent {
    #[sea_orm(string_value = "redeem")]
    Redeem,
    #[sea_orm(string_value = "cheerBits")]
    CheerBits,
    #[sea_orm(string_value = "follow")]
    Follow,
    #[sea_orm(string_value = "subscription")]
    Subscription,
    #[sea_orm(string_value = "giftSubscription")]
    GiftSubscription,
    #[sea_orm(string_value = "reSubscription")]
    ReSubscription,
    #[sea_orm(string_value = "chat")]
    Chat,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship to the script
    #[sea_orm(
        belongs_to = "super::scripts::Entity",
        from = "Column::ScriptId",
        to = "super::scripts::Column::Id"
    )]
    Script,
}

impl Related<super::scripts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Script.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {}
