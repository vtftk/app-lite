use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};

use super::shared::{DbResult, LoggingLevelDb};

// Type alias helpers for the database entity types
pub type ScriptLogsModel = Model;
pub type ScriptLogsEntity = Entity;
pub type ScriptLogsActiveModel = ActiveModel;
pub type ScriptLogsColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "script_logs")]
pub struct Model {
    /// Unique ID of the log
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// ID of the script
    pub script_id: Uuid,
    /// Level of the log
    pub level: LoggingLevelDb,
    /// Logging message
    pub message: String,
    /// Creation time of the event
    pub created_at: DateTimeUtc,
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

#[derive(Debug)]
pub struct CreateScriptLog {
    pub script_id: Uuid,
    pub level: LoggingLevelDb,
    pub message: String,
    pub created_at: DateTimeUtc,
}

impl Model {
    /// Create a new script
    pub async fn create<C>(db: &C, create: CreateScriptLog) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            script_id: Set(create.script_id),
            level: Set(create.level),
            message: Set(create.message),
            created_at: Set(create.created_at),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        Ok(())
    }
}
