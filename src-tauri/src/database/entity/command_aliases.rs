use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::commands::CommandModel;

// Type alias helpers for the database entity types
pub type CommandAliasActiveModel = ActiveModel;
pub type CommandAliasColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "command_alias")]
pub struct Model {
    /// Unique ID of the log
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// ID of the command
    pub command_id: Uuid,
    /// The alias
    pub alias: String,
    /// Order within the command aliases list
    pub order: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandWithAliases {
    #[serde(flatten)]
    pub command: CommandModel,
    pub aliases: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship to the command
    #[sea_orm(
        belongs_to = "super::commands::Entity",
        from = "Column::CommandId",
        to = "super::commands::Column::Id"
    )]
    Command,
}

impl Related<super::commands::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Command.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {}
