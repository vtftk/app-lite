use anyhow::Context;
use axum::{Extension, Json};
use log::info;
use tauri::{AppHandle, Emitter};

use crate::{
    http::{
        error::HttpResult,
        models::calibration::{CalibrationProgressRes, CalibrationStepData},
    },
    state::app_data::{AppDataStore, MinMax, ModelData},
};

pub async fn handle_calibration_progress(
    Extension(app_data): Extension<AppDataStore>,
    Extension(app_handle): Extension<AppHandle>,
    Json(req): Json<CalibrationStepData>,
) -> HttpResult<CalibrationProgressRes> {
    match &req {
        CalibrationStepData::NotStarted => {}
        CalibrationStepData::Smallest => {}
        CalibrationStepData::Largest => {}
        CalibrationStepData::Complete {
            model_id,
            smallest_point,
            largest_point,
        } => {
            info!(
                "COMPLETED CALIBRATION: {:?} {:?}",
                smallest_point, largest_point
            );

            app_data
                .write(move |app_data| {
                    app_data.models.insert(
                        model_id.to_string(),
                        ModelData {
                            x: MinMax {
                                min: smallest_point.x,
                                max: largest_point.x,
                            },
                            y: MinMax {
                                min: smallest_point.y,
                                max: largest_point.y,
                            },
                        },
                    );
                })
                .await
                .context("saving app data")?;
        }
    }

    app_handle
        .emit("calibration_state", req)
        .context("failed to inform app")?;

    Ok(Json(CalibrationProgressRes {}))
}
