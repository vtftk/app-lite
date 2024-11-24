use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, RwLockReadGuard};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppDataStore {
    inner: Arc<AppDataStoreInner>,
}

pub struct AppDataStoreInner {
    /// Current app data
    data: RwLock<AppData>,

    /// File path the app data is stored at
    path: PathBuf,
}

impl AppDataStore {
    pub async fn load(path: PathBuf) -> anyhow::Result<Self> {
        let data = if !path.exists() {
            AppData::default()
        } else {
            load_app_data(&path).await?
        };
        let inner = RwLock::new(data);
        Ok(Self {
            inner: Arc::new(AppDataStoreInner { data: inner, path }),
        })
    }

    /// Obtain a read guard
    pub async fn read(&self) -> RwLockReadGuard<'_, AppData> {
        self.inner.data.read().await
    }

    pub async fn write<F>(&self, action: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut AppData),
    {
        let data = &mut *self.inner.data.write().await;
        action(data);

        debug!("writing app data");
        save_app_data(&self.inner.path, data).await
    }
}

pub async fn load_app_data(path: &Path) -> anyhow::Result<AppData> {
    let data = tokio::fs::read(path).await.context("read file")?;
    let data = serde_json::from_slice(&data).context("parse file")?;
    Ok(data)
}

pub async fn save_app_data(path: &Path, app_data: &AppData) -> anyhow::Result<()> {
    let parent = path.parent().expect("parent should exist");

    if !parent.exists() {
        tokio::fs::create_dir_all(parent).await?
    }

    let data = serde_json::to_string(app_data)?;
    tokio::fs::write(path, &data).await.context("write file")?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AppData {
    pub twitch_config: TwitchConfig,
    pub throwables_config: ThrowablesConfig,
    pub model_config: ModelConfig,
    pub items_config: ItemsConfig,
    pub vtube_studio_config: VTubeStudioConfig,
    pub models: HashMap<ModelId, ModelData>,
    pub items: Vec<ThrowableConfig>,
    pub events: Vec<EventConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VTubeStudioConfig {
    pub host: String,
    pub port: u16,
}

impl Default for VTubeStudioConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8001,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TwitchConfig {
    pub access_token: Option<String>,
}

pub type ModelId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelData {
    /// Min and max X positions of the model
    pub x: MinMax<f64>,
    /// Min and max Y positions of the model
    pub y: MinMax<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThrowablesConfig {
    /// Duration in milliseconds that a thrown object should spend
    /// being thrown
    pub duration: f32,
    /// Range of speed a thrown object can have
    pub spin_speed: MinMax<f32>,
    /// Range of angles an object can be thrown at
    pub throw_angle: MinMax<f32>,
    /// Which direction objects should come from
    pub direction: ThrowDirection,
    /// Delay in milliseconds before impacts show up
    pub impact_delay: f32,
}

impl Default for ThrowablesConfig {
    fn default() -> Self {
        Self {
            duration: 1000.,
            spin_speed: MinMax {
                min: 5000.,
                max: 10_000.,
            },
            throw_angle: MinMax {
                min: -45.,
                max: 45.,
            },
            direction: ThrowDirection::default(),
            impact_delay: 100.,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemsConfig {
    /// Global volume for all sounds
    pub global_volume: f32,
    /// Item scale, range relative to the scale of the model
    pub item_scale: MinMax<f32>,
}

impl Default for ItemsConfig {
    fn default() -> Self {
        Self {
            global_volume: 0.5,
            item_scale: MinMax { min: 0.25, max: 3. },
        }
    }
}

/// Determines how the direction for thrown objects is chosen
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ThrowDirection {
    /// Random direction, left or right
    #[default]
    Random,
    /// Only thrown from left side
    LeftOnly,
    /// Only thrown from right side
    RightOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinMax<T> {
    /// Minimum value
    pub min: T,
    /// Maximum value
    pub max: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ModelConfig {
    /// Time in seconds the model will take to return to its
    /// original position in milliseconds
    pub model_return_time: f32,

    /// How eyes should react when the model is hit by a throwable
    pub eyes_on_hit: EyesMode,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_return_time: 300.,
            eyes_on_hit: EyesMode::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EyesMode {
    /// Eyes should not be changed
    #[default]
    Unchanged,
    /// Eyes should be opened
    Opened,
    /// Eyes should be closed
    Closed,
}

/// Configuration for an individual throwable item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrowableConfig {
    pub id: Uuid,
    /// Name of the throwable item
    pub name: String,
    /// Image to use for the throwable item
    pub image: ThrowableImageConfig,
    /// Optional sound to play upon impact
    pub sound: Option<ImpactSoundConfig>,
}

/// Configuration for a throwable image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrowableImageConfig {
    /// Src URL for the image
    pub src: String,
    /// Weight of impact the image has
    pub weight: u32,
    /// Scale of the image 0-1
    pub scale: f32,
    /// Whether to allow pixelation when rendering at a
    /// different scale
    pub pixelate: bool,
}

/// Configuration for a throwable impact sound
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactSoundConfig {
    /// Src URL for the image
    pub src: String,
    /// Volume of the sound 0-1
    pub volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrowableCollection {
    pub id: Uuid,
    pub name: String,
    pub throwable_ids: Vec<Uuid>,
    pub amount: u32,
    pub throwables_config_override: ThrowablesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfig {
    /// Unique ID of the event
    pub id: Uuid,

    /// Whether the event is enabled
    pub enabled: bool,

    /// Input that should trigger the event
    pub trigger: EventTrigger,

    /// Outcome the event should trigger
    pub outcome: EventOutcome,

    /// Cooldown between each trigger of the even
    pub cooldown: u32,

    /// Minimum required role to trigger the event
    pub require_role: MinimumRequireRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinimumRequireRole {
    None,
    Mod,
    Vip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventTrigger {
    /// Redeem was triggered
    Redeem {
        /// ID of the reward required
        reward_id: String,
    },
    /// Command was sent
    Command {
        /// Command message required
        message: String,
    },
    /// User followed
    Follow,
    /// User subscribed
    Subscription,
    /// User gifted subscription
    GiftedSubscription,
    /// User gifts bits
    Bits {
        /// Minimum bits to trigger the event
        min_bits: u32,
        /// Maximum throws to produce
        max_throws: u32,
    },
    /// Channel has been raided
    Raid {
        /// Minimum raiders required to trigger
        min_raiders: u32,
        /// Minimum and maximum throws to create
        throws: MinMax<u32>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventOutcome {
    /// Throw a random throwable
    Random,
    /// Throw a random barrage of throwables
    RandomBarrage,
    /// Throw a specific throwable
    Throwable { throwable_id: Uuid },
    /// Throw a specific throwable collection
    Collection { collection_id: Uuid },
    /// Trigger a VTube studio hotkey
    TriggerHotkey { hotkey_id: String },
}
