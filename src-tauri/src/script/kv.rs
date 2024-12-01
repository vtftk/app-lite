use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use log::debug;
use tokio::sync::{RwLock, RwLockReadGuard};

#[derive(Clone)]
pub struct KVStore {
    inner: Arc<KVStoreInner>,
}

pub struct KVStoreInner {
    /// Current kv data
    data: RwLock<HashMap<String, String>>,

    /// File path the KV data is stored at
    path: PathBuf,
}

impl KVStore {
    pub async fn load(path: PathBuf) -> anyhow::Result<Self> {
        let data = if !path.exists() {
            HashMap::default()
        } else {
            load_kv_data(&path).await?
        };
        let inner = RwLock::new(data);
        Ok(Self {
            inner: Arc::new(KVStoreInner { data: inner, path }),
        })
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let data = &*self.read().await;
        data.get(key).cloned()
    }

    pub async fn set(&self, key: &str, value: String) -> anyhow::Result<()> {
        self.write(|data| {
            data.insert(key.to_string(), value);
        })
        .await?;

        Ok(())
    }

    pub async fn remove(&self, key: &str) -> anyhow::Result<()> {
        self.write(|data| {
            data.remove(key);
        })
        .await?;

        Ok(())
    }

    /// Obtain a read guard
    pub async fn read(&self) -> RwLockReadGuard<'_, HashMap<String, String>> {
        self.inner.data.read().await
    }

    pub async fn write<F>(&self, action: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut HashMap<String, String>),
    {
        let data = &mut *self.inner.data.write().await;
        action(data);

        debug!("writing kv data");
        save_kv_data(&self.inner.path, data).await
    }
}

pub async fn load_kv_data(path: &Path) -> anyhow::Result<HashMap<String, String>> {
    let data = tokio::fs::read(path).await.context("read file")?;
    let data = serde_json::from_slice(&data).context("parse file")?;
    Ok(data)
}

pub async fn save_kv_data(path: &Path, app_data: &HashMap<String, String>) -> anyhow::Result<()> {
    let parent = path.parent().expect("parent should exist");

    if !parent.exists() {
        tokio::fs::create_dir_all(parent).await?
    }

    let data = serde_json::to_string(app_data)?;
    tokio::fs::write(path, &data).await.context("write file")?;
    Ok(())
}
