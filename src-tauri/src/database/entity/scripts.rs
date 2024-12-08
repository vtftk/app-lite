use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

// Type alias helpers for the database entity types
pub type ScriptModel = Model;
pub type ScriptEntity = Entity;
pub type ScriptActiveModel = ActiveModel;
pub type ScriptColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "scripts")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Whether the script is enabled and runnable
    pub enabled: bool,
    /// Name of the script
    pub name: String,
    /// The actual script contents
    pub script: String,
    /// Names for events the script is known to be subscribed to
    /// script will be run for these events
    pub events: ScriptEvents,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(transparent)]
pub struct ScriptEvents(pub Vec<String>);

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {}
