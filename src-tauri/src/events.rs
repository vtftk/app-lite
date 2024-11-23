use serde::Serialize;
use tokio::sync::broadcast;

use crate::{http::models::calibration::CalibrationStep, state::app_data::ThrowableConfig};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum EventMessage {
    // Tells any connected browser apps to refresh
    Refresh,
    // Sets the current calibration step
    SetCalibrationStep {
        step: CalibrationStep,
    },
    // Throw an item
    Throw {
        config: ThrowableConfig,
    },
    // Throw many items
    ThrowMany {
        config: ThrowableConfig,
        amount: u32,
    },
}

pub struct EventRecvHandle(pub broadcast::Receiver<EventMessage>);

impl Clone for EventRecvHandle {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}
