use crate::http::server::{CalibrationStep, EventMessage};
use serde::Deserialize;
use tokio::sync::broadcast;

/// Set the current calibration step
#[tauri::command]
pub fn set_calibration_step(
    step: CalibrationStep,
    event_sender: tauri::State<'_, broadcast::Sender<EventMessage>>,
) -> Result<(), ()> {
    event_sender
        .send(EventMessage::SetCalibrationStep { step })
        .map_err(|_| ())?;
    Ok(())
}
