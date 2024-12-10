use super::shared::{DbResult, MinMax};
use anyhow::Context;
use sea_orm::{entity::prelude::*, sea_query::OnConflict, ActiveValue::Set, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

// Type alias helpers for the database entity types
pub type ModelDataModel = Model;
pub type ModelDataEntity = Entity;
pub type ModelDataActiveModel = ActiveModel;
pub type ModelDataColumn = Column;

pub type ModelId = String;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_data")]
pub struct Model {
    /// Unique ID for the sound
    #[sea_orm(primary_key)]
    pub id: ModelId,
    /// Name of the model in VT studio
    pub name: String,
    /// Calibration data for the model
    pub calibration: ModelCalibration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ModelCalibration {
    /// Min and max X positions of the model
    pub x: MinMax<f64>,
    /// Min and max Y positions of the model
    pub y: MinMax<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateModelData {
    pub id: String,
    pub name: String,
    pub calibration: ModelCalibration,
}

impl Model {
    /// Create a new script
    pub async fn create<C>(db: &C, create: CreateModelData) -> anyhow::Result<Model>
    where
        C: ConnectionTrait + Send + 'static,
    {
        let active_model = ActiveModel {
            id: Set(create.id.clone()),
            name: Set(create.name),
            calibration: Set(create.calibration),
        };

        Entity::insert(active_model)
            .on_conflict(
                OnConflict::new()
                    .update_column(Column::Name)
                    .update_column(Column::Calibration)
                    .to_owned(),
            )
            .exec_without_returning(db)
            .await?;

        let model = Self::get_by_id(db, &create.id)
            .await?
            .context("model was not inserted")?;

        Ok(model)
    }

    /// Find a specific model data by ID
    pub async fn get_by_id<C>(db: &C, id: &str) -> DbResult<Option<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// Find all model data
    pub async fn all<C>(db: &C) -> DbResult<Vec<Self>>
    where
        C: ConnectionTrait + Send + 'static,
    {
        Entity::find().all(db).await
    }
}
