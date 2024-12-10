use anyhow::Context;
use sea_orm::{entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel};
use serde::{Deserialize, Serialize};

use super::shared::DbResult;

// Type alias helpers for the database entity types
pub type EventExecutionModel = Model;
pub type EventExecutionEntity = Entity;
pub type EventExecutionActiveModel = ActiveModel;
pub type EventExecutionColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "event_executions")]
pub struct Model {
    /// Unique ID for the event
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub event_id: Uuid,
    pub metadata: EventExecutionMetadata,
    pub created_at: DateTimeUtc,
}

#[derive(Clone, Debug, PartialEq, FromJsonQueryResult, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventExecutionMetadata(pub serde_json::Value);

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship to the script
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
pub struct CreateEventExecution {
    pub event_id: Uuid,
    pub metadata: serde_json::Value,
    pub created_at: DateTimeUtc,
}

impl Model {
    /// Create a new script
    pub async fn create<C>(db: &C, create: CreateEventExecution) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            event_id: Set(create.event_id),
            metadata: Set(EventExecutionMetadata(create.metadata)),
            created_at: Set(create.created_at),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;

        Ok(model)
    }

    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find().all(db).await
    }
}
