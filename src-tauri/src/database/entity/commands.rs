use super::{
    command_aliases::{CommandAliasActiveModel, CommandAliasColumn, CommandWithAliases},
    command_executions::{CommandExecutionColumn, CommandExecutionModel},
    command_logs::{CommandLogsColumn, CommandLogsModel},
    shared::{DbResult, ExecutionsQuery, LogsQuery, MinimumRequireRole, UpdateOrdering},
};
use anyhow::Context;
use chrono::Utc;
use futures::{future::BoxFuture, stream::FuturesUnordered, TryStreamExt};
use sea_orm::{
    entity::prelude::*, sea_query::Func, ActiveValue::Set, Condition, FromJsonQueryResult,
    IntoActiveModel, QueryOrder, QuerySelect, UpdateResult,
};
use serde::{Deserialize, Serialize};

// Type alias helpers for the database entity types
pub type CommandModel = Model;

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
    /// The outcome of the command
    pub outcome: CommandOutcome,
    /// Cooldown between each trigger of the command
    pub cooldown: CommandCooldown,
    /// Minimum required role to trigger the command
    pub require_role: MinimumRequireRole,
    /// Ordering
    pub order: u32,
    // Date time of creation
    pub created_at: DateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(tag = "type")]
pub enum CommandOutcome {
    Template { message: String },
    Script { script: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(default)]
pub struct CommandCooldown {
    pub enabled: bool,
    pub duration: u32,
    pub per_user: bool,
}

impl Default for CommandCooldown {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: 1000,
            per_user: false,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Command can have many executions
    #[sea_orm(has_many = "super::command_executions::Entity")]
    Executions,
    /// Command can have many logs
    #[sea_orm(has_many = "super::command_logs::Entity")]
    Logs,
    /// Command can have many aliases
    #[sea_orm(has_many = "super::command_aliases::Entity")]
    Aliases,
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

impl Related<super::command_aliases::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Aliases.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateCommand {
    pub enabled: bool,
    pub name: String,
    pub command: String,
    pub outcome: CommandOutcome,
    pub cooldown: CommandCooldown,
    pub require_role: MinimumRequireRole,
    pub aliases: Vec<String>,
}

#[derive(Default, Deserialize)]
pub struct UpdateCommand {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub command: Option<String>,
    pub outcome: Option<CommandOutcome>,
    pub cooldown: Option<CommandCooldown>,
    pub require_role: Option<MinimumRequireRole>,
    pub order: Option<u32>,
    pub aliases: Vec<String>,
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
            outcome: Set(create.outcome),
            cooldown: Set(create.cooldown),
            require_role: Set(create.require_role),
            order: Set(0),
            created_at: Set(Utc::now()),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;

        model.set_aliases(db, create.aliases).await?;

        Ok(model)
    }

    /// Find commands by the actual command trigger word
    /// and only commands that are enabled
    pub async fn get_by_command<C>(db: &C, command: &str) -> DbResult<Vec<Model>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .left_join(super::command_aliases::Entity)
            .filter(
                Condition::any()
                    .add(Expr::expr(Func::lower(Expr::col(Column::Command))).eq(command))
                    .add(Expr::expr(Func::lower(Expr::col(CommandAliasColumn::Alias))).eq(command)),
            )
            .filter(Column::Enabled.eq(true))
            .group_by(Column::Id)
            .all(db)
            .await
    }

    /// Find the most recent execution of this command
    pub async fn last_execution<C>(
        &self,
        db: &C,
        offset: u64,
    ) -> DbResult<Option<CommandExecutionModel>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        self.find_related(super::command_executions::Entity)
            .order_by_desc(CommandExecutionColumn::CreatedAt)
            .offset(offset)
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
    /// Find a specific sound by ID
    pub async fn get_by_id_with_aliases<C>(db: &C, id: Uuid) -> DbResult<Option<CommandWithAliases>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let command = match Entity::find_by_id(id).one(db).await? {
            Some(value) => value,
            None => return Ok(None),
        };

        let aliases = command.get_aliases(db).await?;

        Ok(Some(CommandWithAliases { command, aliases }))
    }

    /// Find all sounds
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find()
            .order_by_asc(Column::Order)
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
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
        this.outcome = data.outcome.map(Set).unwrap_or(this.outcome);
        this.cooldown = data.cooldown.map(Set).unwrap_or(this.cooldown);
        this.require_role = data.require_role.map(Set).unwrap_or(this.require_role);
        this.order = data.order.map(Set).unwrap_or(this.order);

        let this = this.update(db).await?;

        this.set_aliases(db, data.aliases).await?;

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

        select
            .order_by(CommandLogsColumn::CreatedAt, sea_orm::Order::Desc)
            .all(db)
            .await
    }

    pub async fn get_aliases<C>(&self, db: &C) -> DbResult<Vec<String>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        self.find_related(super::command_aliases::Entity)
            .order_by(CommandAliasColumn::Order, sea_orm::Order::Asc)
            .all(db)
            .await
            .map(|aliases| aliases.into_iter().map(|alias| alias.alias).collect())
    }

    pub async fn set_aliases<C>(&self, db: &C, aliases: Vec<String>) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // Delete all command aliases for the command
        super::command_aliases::Entity::delete_many()
            .filter(CommandAliasColumn::CommandId.eq(self.id))
            .exec(db)
            .await?;

        super::command_aliases::Entity::insert_many(aliases.into_iter().enumerate().map(
            |(index, alias)| CommandAliasActiveModel {
                id: Set(Uuid::new_v4()),
                command_id: Set(self.id),
                alias: Set(alias),
                order: Set(index as u32),
            },
        ))
        .on_conflict_do_nothing()
        .exec_without_returning(db)
        .await?;

        Ok(())
    }

    pub async fn get_executions<C>(
        &self,
        db: &C,
        query: ExecutionsQuery,
    ) -> DbResult<Vec<CommandExecutionModel>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut select = self.find_related(super::command_executions::Entity);

        if let Some(start_date) = query.start_date {
            select = select.filter(CommandExecutionColumn::CreatedAt.gt(start_date))
        }

        if let Some(end_date) = query.end_date {
            select = select.filter(CommandExecutionColumn::CreatedAt.lt(end_date))
        }

        if let Some(offset) = query.offset {
            select = select.offset(offset);
        }

        if let Some(limit) = query.limit {
            select = select.limit(limit);
        }

        select
            .order_by(CommandExecutionColumn::CreatedAt, sea_orm::Order::Desc)
            .all(db)
            .await
    }

    pub async fn update_order<C>(db: &C, data: Vec<UpdateOrdering>) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let _results: Result<Vec<UpdateResult>, DbErr> = data
            .into_iter()
            .map(|data| -> BoxFuture<'_, DbResult<UpdateResult>> {
                Box::pin(
                    Entity::update_many()
                        .filter(Column::Id.eq(data.id))
                        .col_expr(Column::Order, data.order.into())
                        .exec(db),
                )
            })
            .collect::<FuturesUnordered<BoxFuture<'_, DbResult<UpdateResult>>>>()
            .try_collect()
            .await;

        Ok(())
    }
}
