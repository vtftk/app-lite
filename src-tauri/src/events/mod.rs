pub mod event_processing;
pub mod matching;
pub mod outcome;
pub mod processing;

use serde::Serialize;
use tokio::sync::broadcast;

use crate::{
    database::entity::SoundModel, http::models::calibration::CalibrationStep,
    state::app_data::ThrowableConfig,
};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum EventMessage {
    // Sets the current calibration step
    SetCalibrationStep {
        step: CalibrationStep,
    },

    /// Throw a specific item
    ThrowItem {
        config: ThrowableConfig,
        amount: u32,
    },

    /// Throw a barrage of many items
    ThrowItemBarrage {
        config: ThrowableConfig,
        amount_per_throw: u32,
        amount: u32,
        frequency: u32,
    },

    // Request the latest set of vtube studio hotkeys
    UpdateHotkeys,

    // Trigger a vtube studio hotkey
    TriggerHotkey {
        hotkey_id: String,
    },

    // Play a sound
    PlaySound {
        config: SoundModel,
    },

    // Play a sequence of sounds one after the other
    PlaySoundSeq {
        configs: Vec<SoundModel>,
    },
}

pub struct EventRecvHandle(pub broadcast::Receiver<EventMessage>);

impl Clone for EventRecvHandle {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

pub type EventSendHandle = broadcast::Sender<EventMessage>;

pub fn create_event_channel() -> (EventSendHandle, EventRecvHandle) {
    let (event_tx, rx) = broadcast::channel(10);
    let event_rx = EventRecvHandle(rx);

    (event_tx, event_rx)
}
