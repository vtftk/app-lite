use crate::{
    commands::CmdResult, database::entity::ModelDataModel, events::EventMessage,
    http::models::calibration::CalibrationStep,
};
use sea_orm::DatabaseConnection;
use tauri::State;
use tokio::sync::broadcast;

/// Set the current calibration step
#[tauri::command]
pub fn set_calibration_step(
    step: CalibrationStep,
    event_sender: State<'_, broadcast::Sender<EventMessage>>,
) -> Result<(), ()> {
    event_sender
        .send(EventMessage::SetCalibrationStep { step })
        .map_err(|_| ())?;
    Ok(())
}

/// Move the model for calibration
#[tauri::command]
pub fn calibration_move_model(
    x: f32,
    y: f32,
    event_sender: State<'_, broadcast::Sender<EventMessage>>,
) -> Result<(), ()> {
    event_sender
        .send(EventMessage::MoveModel { x, y })
        .map_err(|_| ())?;
    Ok(())
}

/// Get all model data
#[tauri::command]
pub async fn get_calibration_data(
    db: State<'_, DatabaseConnection>,
) -> CmdResult<Vec<ModelDataModel>> {
    let db = db.inner();
    let model_data = ModelDataModel::all(db).await?;
    Ok(model_data)
}
