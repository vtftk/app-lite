use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use tokio::sync::{RwLock, RwLockReadGuard};

#[derive(Default, Clone)]
pub struct RuntimeAppDataStore {
    inner: Arc<RwLock<RuntimeAppData>>,
}

impl RuntimeAppDataStore {
    /// Obtain a read guard
    pub async fn read(&self) -> RwLockReadGuard<'_, RuntimeAppData> {
        self.inner.read().await
    }

    pub async fn write<F>(&self, action: F)
    where
        F: FnOnce(&mut RuntimeAppData),
    {
        let data = &mut *self.inner.write().await;
        action(data);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RuntimeAppData {
    pub model_id: Option<String>,
}
