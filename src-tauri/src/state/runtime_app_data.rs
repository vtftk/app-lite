use serde::{Deserialize, Serialize};
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
    pub model_id: Option<String>,
    pub vtube_studio_connected: bool,
}
