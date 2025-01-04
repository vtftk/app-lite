use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use twitch_api::{helix::Scope, twitch_oauth2::AccessToken};
use uuid::Uuid;

use crate::database::entity::{ItemModel, SoundModel};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AppConfig {
    pub main_config: MainConfig,
    pub externals_config: ExternalsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OverlayConfig {
    pub throwables_config: ThrowablesConfig,
    pub model_config: ModelConfig,
    pub sounds_config: SoundsConfig,
    pub vtube_studio_config: VTubeStudioConfig,
    pub physics_config: PhysicsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AppData {
    #[serde(flatten)]
    pub app: AppConfig,

    #[serde(flatten)]
    pub overlay: OverlayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(default)]
pub struct MainConfig {
    /// Minimize to try instead of closing
    pub minimize_to_tray: bool,
    /// Clean old log data on startup
    pub clean_logs: bool,
    /// Number of days of logs to retain when cleaning logs
    pub clean_logs_days: u64,
    /// Clean old execution data on start
    pub clean_executions: bool,
    /// Number of days of execution data to retain when cleaning executions
    pub clean_executions_days: u64,
    /// Allow automatic updates
    pub auto_updating: bool,
    /// Port for the HTTP server
    http_port: u16,
}

impl MainConfig {
    pub fn get_http_port(&self) -> u16 {
        #[cfg(not(debug_assertions))]
        return self.http_port;

        // Debug fixed port override
        #[cfg(debug_assertions)]
        return 58372;
    }
}

pub fn default_http_port() -> u16 {
    58371
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            minimize_to_tray: true,
            clean_logs: true,
            clean_logs_days: 30,
            clean_executions: true,
            clean_executions_days: 30,
            auto_updating: true,
            http_port: default_http_port(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromJsonQueryResult)]
#[serde(default)]
pub struct ExternalsConfig {
    pub tts_monster_api_key: Option<String>,
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
    pub access_token: Option<AccessToken>,
    pub scopes: Option<Vec<Scope>>,
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
    /// Item scale, range relative to the scale of the model
    pub item_scale: MinMax<f32>,
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
            item_scale: MinMax { min: 0.25, max: 3. },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SoundsConfig {
    /// Global volume for all sounds
    pub global_volume: f32,
}

impl Default for SoundsConfig {
    fn default() -> Self {
        Self { global_volume: 0.5 }
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

/// Collection of items along with the resolved impact
/// sounds for the items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsWithSounds {
    /// All the referenced items
    pub items: Vec<ItemWithImpactSoundIds>,
    /// All the referenced sounds
    pub impact_sounds: Vec<SoundModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemWithImpactSoundIds {
    #[serde(flatten)]
    pub item: ItemModel,
    pub impact_sound_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PhysicsConfig {
    pub enabled: bool,
    pub fps: u16,
    pub gravity_multiplier: f32,
    pub horizontal_multiplier: f32,
    pub vertical_multiplier: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            fps: 30,
            gravity_multiplier: 1.,
            horizontal_multiplier: 1.,
            vertical_multiplier: 1.,
        }
    }
}
