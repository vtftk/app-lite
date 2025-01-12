use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};

use super::shared::{DbResult, LoggingLevelDb};

// Type alias helpers for the database entity types
pub type CommandLogsModel = Model;
pub type CommandLogsColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "command_logs")]
pub struct Model {
    /// Unique ID of the log
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// ID of the command
    pub command_id: Uuid,
    /// Level of the log
    pub level: LoggingLevelDb,
    /// Logging message
    pub message: String,
    /// Creation time of the event
    pub created_at: DateTimeUtc,
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

#[derive(Debug)]
pub struct CreateCommandLog {
    pub command_id: Uuid,
    pub level: LoggingLevelDb,
    pub message: String,
    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Create a new script
    pub async fn create<C>(db: &C, create: CreateCommandLog) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            command_id: Set(create.command_id),
            level: Set(create.level),
            message: Set(create.message),
            created_at: Set(create.created_at),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;
        Ok(())
    }

    pub async fn delete_many<C>(db: &C, ids: &[Uuid]) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::delete_many()
            .filter(Column::Id.is_in(ids.iter().copied()))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn delete_before<C>(db: &C, start_date: DateTimeUtc) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::delete_many()
            .filter(Column::CreatedAt.lt(start_date))
            .exec(db)
            .await?;
        Ok(())
    }
}
