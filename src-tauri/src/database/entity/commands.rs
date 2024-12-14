use anyhow::Context;
use sea_orm::{
    entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel, QueryOrder,
    QuerySelect,
};
use serde::{Deserialize, Serialize};

use super::{
    command_executions::{CommandExecutionColumn, CommandExecutionModel},
    command_logs::{CommandLogsColumn, CommandLogsModel},
    shared::{DbResult, LogsQuery, MinimumRequireRole},
};

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
    /// Ordering
    pub order: u32,
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
pub enum Relation {
    /// Command can have many executions
    #[sea_orm(has_many = "super::command_executions::Entity")]
    Executions,
    /// Command can have many logs
    #[sea_orm(has_many = "super::command_logs::Entity")]
    Logs,
}

impl Related<super::command_executions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Executions.def()
    }
}

impl Related<super::command_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Logs.def()
    }
}

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
    pub order: Option<u32>,
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
            order: Set(0),
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

    /// Find the most recent execution of this command
    pub async fn last_execution<C>(&self, db: &C) -> DbResult<Option<CommandExecutionModel>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        self.find_related(super::command_executions::Entity)
            .order_by_desc(CommandExecutionColumn::CreatedAt)
            .one(db)
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
        Entity::find().order_by_asc(Column::Order).all(db).await
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
        this.order = data.order.map(Set).unwrap_or(this.order);

        let this = this.update(db).await?;
        Ok(this)
    }

    pub async fn get_logs<C>(&self, db: &C, query: LogsQuery) -> DbResult<Vec<CommandLogsModel>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut select = self.find_related(super::command_logs::Entity);

        if let Some(level) = query.level {
            select = select.filter(CommandLogsColumn::Level.eq(level))
        }

        if let Some(start_date) = query.start_date {
            select = select.filter(CommandLogsColumn::CreatedAt.gt(start_date))
        }

        if let Some(end_date) = query.end_date {
            select = select.filter(CommandLogsColumn::CreatedAt.lt(end_date))
        }

        if let Some(offset) = query.offset {
            select = select.offset(offset);
        }

        if let Some(limit) = query.limit {
            select = select.limit(limit);
        }

        select.all(db).await
    }
}
