use anyhow::Context;
use sea_orm::{entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel};
use serde::{Deserialize, Serialize};

use super::shared::{DbResult, MinimumRequireRole};

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
    /// The command to trigger when entered
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

#[derive(Debug, Deserialize)]
pub struct CreateCommand {
    pub enabled: bool,
    pub name: String,
    pub command: String,
    pub aliases: CommandAliases,
    pub outcome: CommandOutcome,
    pub cooldown: u32,
    pub require_role: MinimumRequireRole,
}

#[derive(Default, Deserialize)]
pub struct UpdateCommand {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub command: Option<String>,
    pub aliases: Option<CommandAliases>,
    pub outcome: Option<CommandOutcome>,
    pub cooldown: Option<u32>,
    pub require_role: Option<MinimumRequireRole>,
}

impl Model {
    /// Create a new sound
    pub async fn create<C>(db: &C, create: CreateCommand) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            enabled: Set(create.enabled),
            name: Set(create.name),
            command: Set(create.command),
            aliases: Set(create.aliases),
            outcome: Set(create.outcome),
            cooldown: Set(create.cooldown),
            require_role: Set(create.require_role),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;

        Ok(model)
    }

    /// Find commands by the actual command trigger word
    /// and only commands that are enabled
    pub async fn get_by_command<C>(db: &C, command: &str) -> DbResult<Vec<Model>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // TODO: Join against future aliases table
        Entity::find()
            .filter(Column::Command.eq(command).and(Column::Enabled.eq(true)))
            .all(db)
            .await
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
    pub async fn update<C>(self, db: &C, data: UpdateCommand) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        this.enabled = data.enabled.map(Set).unwrap_or(this.enabled);
        this.name = data.name.map(Set).unwrap_or(this.name);
        this.command = data.command.map(Set).unwrap_or(this.command);
        this.aliases = data.aliases.map(Set).unwrap_or(this.aliases);
        this.outcome = data.outcome.map(Set).unwrap_or(this.outcome);
        this.cooldown = data.cooldown.map(Set).unwrap_or(this.cooldown);
        this.require_role = data.require_role.map(Set).unwrap_or(this.require_role);

        let this = this.update(db).await?;
        Ok(this)
    }
}
