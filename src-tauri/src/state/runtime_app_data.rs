use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt::Debug, sync::Arc};
use tauri::{AppHandle, Emitter};
use tokio::sync::{RwLock, RwLockReadGuard};

/// Store for [RuntimeAppData] when the state changes the client frontend
/// receives an event containing the new data
#[derive(Clone)]
pub struct RuntimeAppDataStore {
    inner: Arc<RuntimeAppDataStoreInner>,
}

pub struct RuntimeAppDataStoreInner {
    /// Actual current runtime app data
    data: RwLock<RuntimeAppData>,
    /// App handle to report changes to
    app_handle: AppHandle,
}

impl RuntimeAppDataStore {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            inner: Arc::new(RuntimeAppDataStoreInner {
                data: Default::default(),
                app_handle,
            }),
        }
    }

    /// Obtain a read guard
    pub async fn read(&self) -> RwLockReadGuard<'_, RuntimeAppData> {
        self.inner.data.read().await
    }

    pub async fn write<F>(&self, action: F)
    where
        F: FnOnce(&mut RuntimeAppData),
    {
        let data = &mut *self.inner.data.write().await;
        action(data);

        // Let the frontend know the runtime data has changed
        _ = self
            .inner
            .app_handle
            .emit("runtime_app_data_changed", &data);
    }
}

/// App data used at runtime, used by the overlay for informing the client
/// the current state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RuntimeAppData {
    /// ID of current model
    pub model_id: Option<String>,

    /// vtube studio connection state
    pub vtube_studio_connected: bool,

    /// VTube studio authentication state
    pub vtube_studio_auth: bool,

    /// Current hotkey list from vtube studio
    pub hotkeys: Vec<VTubeStudioHotkey>,

    /// Current number of active connected overlays
    pub active_overlay_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VTubeStudioHotkey {
    pub hotkey_id: String,
    pub name: String,
}

/// Partial update to the runtime app data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UpdateRuntimeAppData {
    #[serde(default, deserialize_with = "deserialize_some")]
    pub model_id: Option<Option<String>>,
    pub vtube_studio_connected: Option<bool>,
    pub vtube_studio_auth: Option<bool>,
    pub hotkeys: Option<Vec<VTubeStudioHotkey>>,
}

// Any value that is present is considered Some value, including null.
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}
