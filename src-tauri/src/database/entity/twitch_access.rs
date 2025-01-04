use super::shared::DbResult;
use anyhow::Context;
use chrono::Utc;
use sea_orm::{entity::prelude::*, sea_query::OnConflict, ActiveValue::Set, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use twitch_api::{helix::Scope, twitch_oauth2::AccessToken};

// Type alias helpers for the database entity types
pub type TwitchAccessModel = Model;
pub type TwitchAccessEntity = Entity;
pub type TwitchAccessActiveModel = ActiveModel;
pub type TwitchAccessColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "twitch_access")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: i32,

    pub access_token: DbAccessToken,
    pub scopes: DbScopes,

    // Date time of creation
    pub created_at: DateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(transparent)]
pub struct DbAccessToken(pub AccessToken);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(transparent)]
pub struct DbScopes(pub Vec<Scope>);

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct SetTwitchAccess {
    pub access_token: AccessToken,
    pub scopes: Vec<Scope>,
}

impl Model {
    /// Only one row should ever be created and should have this ID
    const SINGLETON_ID: i32 = 1;

    /// Create a new sound
    pub async fn set<C>(db: &C, create: SetTwitchAccess) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            id: Set(Self::SINGLETON_ID),
            access_token: Set(DbAccessToken(create.access_token)),
            scopes: Set(DbScopes(create.scopes)),
            created_at: Set(Utc::now()),
        };

        Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(Column::Id)
                    .update_columns([Column::AccessToken, Column::Scopes, Column::CreatedAt])
                    .to_owned(),
            )
            .exec_without_returning(db)
            .await?;

        let model = Self::get(db).await?.context("model was not inserted")?;

        Ok(model)
    }

    pub async fn get<C>(db: &C) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(Self::SINGLETON_ID).one(db).await
    }
}
