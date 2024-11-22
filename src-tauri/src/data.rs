use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    pub throwables: ThrowablesConfig,
    pub model: ModelConfig,
    pub items: ItemsConfig,
    pub models: HashMap<ModelId, ModelData>,
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
