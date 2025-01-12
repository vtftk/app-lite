use serde::{Deserialize, Serialize};

use crate::database::entity::model_data::{ModelDataModel, ModelId};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "step")]
pub struct CalibrationPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "step")]
pub enum CalibrationStepData {
    NotStarted,
    Smallest,
    Largest,
    Complete {
        model_id: ModelId,
        model_name: String,
        smallest_point: CalibrationPoint,
        largest_point: CalibrationPoint,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CalibrationStep {
    NotStarted,
    Smallest,
    Largest,
    Complete,
}

#[derive(Debug, Serialize)]
pub struct CalibrationProgressRes {
    /// Updated model data when a model calibration is complete
    pub model_data: Option<ModelDataModel>,
}
