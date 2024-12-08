use super::shared::MinMax;
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
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

impl Model {}
