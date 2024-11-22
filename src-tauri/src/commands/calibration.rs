use crate::http::server::{CalibrationStep, EventMessage};
use tokio::sync::broadcast;

/// Obtain a URL for use logging into twitch using OAuth2
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
