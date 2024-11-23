use crate::{events::EventMessage, http::models::calibration::CalibrationStep};
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
