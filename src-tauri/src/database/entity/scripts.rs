use sea_orm::{entity::prelude::*, ActiveValue::Set, FromJsonQueryResult, IntoActiveModel};
use serde::{Deserialize, Serialize};

use super::shared::DbResult;

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

#[derive(Debug, Deserialize)]
pub struct CreateScript {
    pub enabled: bool,
    pub name: String,
    pub script: String,
    pub events: ScriptEvents,
}

#[derive(Default, Deserialize)]
pub struct UpdateScript {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub script: Option<String>,
    pub events: Option<ScriptEvents>,
}

impl Model {
    /// Create a new script
    pub async fn create<C>(db: &C, create: CreateScript) -> DbResult<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            id: Set(Uuid::new_v4()),
            enabled: Set(create.enabled),
            name: Set(create.name),
            script: Set(create.script),
            events: Set(create.events),
        };

        let model = active_model.insert(db).await?;

        Ok(model)
    }

    /// Find a specific script by ID
    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// Find all script
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find().all(db).await
    }

    /// Update the current script
    pub async fn update<C>(self, db: &C, data: UpdateScript) -> DbResult<Self>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let mut this = self.into_active_model();

        if let Some(enabled) = data.enabled {
            this.enabled = Set(enabled);
        }

        if let Some(name) = data.name {
            this.name = Set(name);
        }

        if let Some(script) = data.script {
            this.script = Set(script);
        }

        if let Some(events) = data.events {
            this.events = Set(events);
        }

        let this = this.update(db).await?;
        Ok(this)
    }
}
