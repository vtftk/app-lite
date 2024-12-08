use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

use super::shared::MinimumRequireRole;

// Type alias helpers for the database entity types
pub type CommandModel = Model;
pub type CommandEntity = Entity;
pub type CommandActiveModel = ActiveModel;
pub type CommandColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "commands")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// Whether the command is enabled and runnable
    pub enabled: bool,
    /// Name of the command
    pub name: String,
    /// The command to run
    pub command: String,
    /// Aliases that also trigger the command
    pub aliases: CommandAliases,
    /// The outcome of the command
    pub outcome: CommandOutcome,
    /// Cooldown between each trigger of the command
    pub cooldown: u32,
    /// Minimum required role to trigger the command
    pub require_role: MinimumRequireRole,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(tag = "type")]
pub enum CommandOutcome {
    Template { message: String },
    Script { script: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(transparent)]
pub struct CommandAliases(pub Vec<String>);

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {}
