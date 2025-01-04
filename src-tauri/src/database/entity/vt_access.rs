use super::shared::DbResult;
use anyhow::Context;
use chrono::Utc;
use sea_orm::{entity::prelude::*, sea_query::OnConflict, ActiveValue::Set};
use serde::{Deserialize, Serialize};

// Type alias helpers for the database entity types
pub type VTAccessModel = Model;
pub type VTAccessEntity = Entity;
pub type VTAccessActiveModel = ActiveModel;
pub type VTAccessColumn = Column;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "vt_access")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: i32,
    pub access_token: String,
    // Date time of creation
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct SetVTAccess {
    pub access_token: String,
}

impl Model {
    /// Only one row should ever be created and should have this ID
    const SINGLETON_ID: i32 = 1;

    /// Create a new sound
    pub async fn set<C>(db: &C, create: SetVTAccess) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            id: Set(Self::SINGLETON_ID),
            access_token: Set(create.access_token),
            created_at: Set(Utc::now()),
        };

        Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(Column::Id)
                    .update_columns([Column::AccessToken, Column::CreatedAt])
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
