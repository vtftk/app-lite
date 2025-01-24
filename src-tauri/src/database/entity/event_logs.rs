use sea_orm::{
    entity::prelude::*, sea_query::Func, ActiveValue::Set, FromQueryResult, QuerySelect,
};
use serde::{Deserialize, Serialize};

use super::shared::{DbResult, LoggingLevelDb};

// Type alias helpers for the database entity types
pub type EventLogsModel = Model;
pub type EventLogsColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "event_logs")]
pub struct Model {
    /// Unique ID of the log
    #[sea_orm(primary_key)]
    pub id: Uuid,
    /// ID of the event
    pub event_id: Uuid,
    /// Level of the log
    pub level: LoggingLevelDb,
    /// Logging message
    pub message: String,
    /// Creation time of the event
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship to the event
    #[sea_orm(
        belongs_to = "super::events::Entity",
        from = "Column::EventId",
        to = "super::events::Column::Id"
    )]
    Event,
}

impl Related<super::events::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug)]
pub struct CreateEventLog {
    pub event_id: Uuid,
    pub level: LoggingLevelDb,
    pub message: String,
    pub created_at: DateTimeUtc,
}

impl Model {
    /// Create a new script
    pub async fn create<C>(db: &C, create: CreateEventLog) -> DbResult<()>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            event_id: Set(create.event_id),
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

    pub async fn estimate_size<C>(db: &C) -> DbResult<u32>
    where
        C: ConnectionTrait + Send + 'static,
    {
        #[derive(Default, FromQueryResult)]
        struct PartialModel {
            total_message_length: u32,
        }

        let result = Entity::find()
            .expr_as(
                Func::sum(Func::char_length(Expr::col(Column::Message))),
                "total_message_length",
            )
            .into_model::<PartialModel>()
            .one(db)
            .await?;

        let result = match result {
            Some(result) => result,
            None => return Ok(0),
        };

        Ok(result.total_message_length)
    }
}
