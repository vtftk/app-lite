use crate::state::app_data::{default_http_port, AppData, ExternalsConfig, MainConfig};
use anyhow::Context;
use chrono::Utc;
use sea_orm::{
    entity::prelude::*, sea_query::OnConflict, ActiveValue::Set, FromJsonQueryResult,
    FromQueryResult, QuerySelect,
};
use serde::{Deserialize, Serialize};

// Type alias helpers for the database entity types
pub type AppDataModel = Model;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_data")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: i32,

    pub data: DbAppData,

    pub created_at: DateTimeUtc,
    pub last_modified_at: DateTimeUtc,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(transparent)]
pub struct DbAppData(pub AppData);

impl PartialEq for DbAppData {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Only one row should ever be created and should have this ID
    const SINGLETON_ID: i32 = 1;

    /// Create a new sound
    pub async fn set<C>(db: &C, app_data: AppData) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            id: Set(Self::SINGLETON_ID),
            data: Set(DbAppData(app_data)),
            created_at: Set(Utc::now()),
            last_modified_at: Set(Utc::now()),
        };

        Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(Column::Id)
                    .update_columns([Column::Data, Column::LastModifiedAt])
                    .to_owned(),
            )
            .exec_without_returning(db)
            .await?;

        let model = Entity::find_by_id(Self::SINGLETON_ID)
            .one(db)
            .await?
            .context("model not inserted")?;

        Ok(model)
    }

    pub async fn get_or_default<C>(db: &C) -> anyhow::Result<AppData>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let model = Entity::find_by_id(Self::SINGLETON_ID).one(db).await?;

        let model = match model {
            Some(value) => value,
            None => Self::set(db, Default::default()).await?,
        };

        Ok(model.data.0)
    }

    /// HTTP port is loaded pretty frequently
    pub async fn get_http_port<C>(db: &C) -> anyhow::Result<u16>
    where
        C: ConnectionTrait + Send + 'static,
    {
        #[derive(Default, FromQueryResult)]
        struct PartialModel {
            http_port: Option<u16>,
        }

        // HTTP port is loaded frequently so save on loading the entire main_config every time
        let http_port = Entity::find_by_id(Self::SINGLETON_ID)
            .select_only()
            // Select just the HTTP port from the data
            .expr_as(
                Expr::cust("json_extract(data, '$.main_config.http_port')"),
                "main_config",
            )
            .into_model::<PartialModel>()
            .one(db)
            .await?
            .and_then(|value| value.http_port)
            .unwrap_or_else(default_http_port);

        // Debug fixed port override
        #[cfg(debug_assertions)]
        {
            _ = http_port;
            Ok(58372)
        }

        #[cfg(not(debug_assertions))]
        Ok(http_port)
    }

    pub async fn is_auto_updating<C>(db: &C) -> anyhow::Result<bool>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let main_config = Self::get_main_config(db).await?;
        Ok(main_config.auto_updating)
    }

    pub async fn get_main_config<C>(db: &C) -> anyhow::Result<MainConfig>
    where
        C: ConnectionTrait + Send + 'static,
    {
        #[derive(Default, FromQueryResult)]
        struct PartialModel {
            main_config: MainConfig,
        }

        Ok(Entity::find_by_id(Self::SINGLETON_ID)
            .select_only()
            // Select
            .expr_as(
                Expr::cust("json_extract(data, '$.main_config')"),
                "main_config",
            )
            .into_model::<PartialModel>()
            .one(db)
            .await?
            .map(|value| value.main_config)
            .unwrap_or_default())
    }

    pub async fn get_externals_config<C>(db: &C) -> anyhow::Result<ExternalsConfig>
    where
        C: ConnectionTrait + Send + 'static,
    {
        #[derive(Default, FromQueryResult)]
        struct PartialModel {
            externals_config: ExternalsConfig,
        }

        Ok(Entity::find_by_id(Self::SINGLETON_ID)
            .select_only()
            // Select
            .expr_as(
                Expr::cust("json_extract(data, '$.externals_config')"),
                "externals_config",
            )
            .into_model::<PartialModel>()
            .one(db)
            .await?
            .map(|value| value.externals_config)
            .unwrap_or_default())
    }
}
