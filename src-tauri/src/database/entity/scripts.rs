use anyhow::Context;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "camelCase")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ScriptEvent {
    #[sea_orm(string_value = "redeem")]
    Redeem,
    #[sea_orm(string_value = "cheerBits")]
    CheerBits,
    #[sea_orm(string_value = "follow")]
    Follow,
    #[sea_orm(string_value = "subscription")]
    Subscription,
    #[sea_orm(string_value = "giftSubscription")]
    GiftSubscription,
    #[sea_orm(string_value = "reSubscription")]
    ReSubscription,
    #[sea_orm(string_value = "chat")]
    Chat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(transparent)]
pub struct ScriptEvents(pub Vec<ScriptEvent>);

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
    pub async fn create<C>(db: &C, create: CreateScript) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let id = Uuid::new_v4();
        let active_model = ActiveModel {
            id: Set(id),
            enabled: Set(create.enabled),
            name: Set(create.name),
            script: Set(create.script),
            events: Set(create.events),
        };

        Entity::insert(active_model)
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, id)
            .await?
            .context("model was not inserted")?;

        Ok(model)
    }

    /// Find a specific script by ID
    pub async fn get_by_id<C>(db: &C, id: Uuid) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// Find a script by the event its subscribed to filters to only enabled
    pub async fn get_by_event<C>(db: &C, script_event: ScriptEvent) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        // TODO: DATABASE LEVEL FILTERING, USE SEPARATE COLUMN TO STORE SUBSCRIBED EVENTS
        let scripts = Self::all(db).await?;

        Ok(scripts
            .into_iter()
            .filter(|script| {
                script.enabled && script.events.0.iter().any(|event| script_event.eq(event))
            })
            .collect())
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
