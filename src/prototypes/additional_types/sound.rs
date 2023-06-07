use crate::util::defaults::*;
use serde::Deserialize;
use serde_with::serde_as;
use thiserror::Error;

use super::FileName;

/// <https://wiki.factorio.com/Types/LayeredSound>
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "LayeredSoundIntermediate")]
pub struct LayeredSound {
    pub layers: Vec<Sound>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum LayeredSoundIntermediate {
    Many { layers: Vec<Sound> },
    Single(Sound),
}

impl From<LayeredSoundIntermediate> for LayeredSound {
    fn from(value: LayeredSoundIntermediate) -> Self {
        match value {
            LayeredSoundIntermediate::Many { layers } => Self { layers },
            LayeredSoundIntermediate::Single(sound) => Self {
                layers: vec![sound],
            },
        }
    }
}

/// <https://wiki.factorio.com/Types/Sound>
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Sound {
    AsTable(SoundAsTable),
    Variations(Vec<SoundVariation>),
}

/// <https://wiki.factorio.com/Types/Sound>
#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct SoundAsTable {
    pub aggregation: Option<SoundAggregation>,
    #[serde(default = "default_bool::<false>")]
    pub allow_random_repeat: bool,
    #[serde(default = "default_from_i8::<f64, 1>")]
    pub audible_distance_modifier: f64,
}

/// <https://wiki.factorio.com/Types/Sound#aggregation>
#[derive(Debug, Clone, Deserialize)]
pub struct SoundAggregation {
    pub max_count: u32,
    #[serde(default = "default_from_i8::<f32, 1>")]
    pub progress_threshold: f32,
    pub remove: bool,
    #[serde(default = "default_bool::<false>")]
    pub count_already_playing: bool,
}

/// <https://wiki.factorio.com/Types/Sound#variations>
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "SoundVariationIntermediate")]
pub struct SoundVariation {
    pub filename: FileName,
    pub volume: f32,
    pub preload: Option<bool>, // Strange that this doesn't have a default
    pub speed: SoundVariationSpeed,
}

#[derive(Deserialize)]
struct SoundVariationIntermediate {
    pub filename: FileName,
    #[serde(default = "default_from_i8::<f32, 1>")]
    pub volume: f32,
    pub preload: Option<bool>, // Strange that this doesn't have a default
    #[serde(flatten)]
    pub speed: SoundVariationSpeed,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SoundVariationSpeed {
    Single {
        #[serde(default = "default_from_i8::<f32, 1>")]
        speed: f32, // >= 1 / 64
    },
    Range {
        #[serde(default = "default_from_i8::<f32, 1>")]
        min_speed: f32, // >= 1/64
        #[serde(default = "default_from_i8::<f32, 1>")]
        max_speed: f32, // >= min_speed
    },
}

#[derive(Debug, Clone, Error)]
pub enum SoundVariationValidationError {
    #[error("`speed` must be >= 1/64, got {0}")]
    IncorrectSpeed(f32),
    #[error("`min_speed` must be >= 1/64, got {0}")]
    IncorrectMinSpeed(f32),
    #[error("`max_speed` must be >= `min_speed`. `min_speed` is {min_speed}, `max_speed` is {max_speed}")]
    IncorrectMaxSpeed { min_speed: f32, max_speed: f32 },
}

impl TryFrom<SoundVariationIntermediate> for SoundVariation {
    type Error = SoundVariationValidationError;

    fn try_from(value: SoundVariationIntermediate) -> Result<Self, Self::Error> {
        let SoundVariationIntermediate {
            filename,
            volume,
            preload,
            speed,
        } = value;
        match speed {
            SoundVariationSpeed::Single { speed } => {
                if speed < 1.0 / 64.0 {
                    return Err(SoundVariationValidationError::IncorrectSpeed(speed));
                }
            }
            SoundVariationSpeed::Range {
                min_speed,
                max_speed,
            } => {
                if min_speed < 1.0 / 64.0 {
                    return Err(SoundVariationValidationError::IncorrectMinSpeed(min_speed));
                }
                if max_speed < min_speed {
                    return Err(SoundVariationValidationError::IncorrectMaxSpeed {
                        min_speed,
                        max_speed,
                    });
                }
            }
        }
        Ok(SoundVariation {
            filename,
            volume,
            preload,
            speed,
        })
    }
}

/// <https://wiki.factorio.com/Types/WorkingSound>
#[derive(Debug, Clone, Deserialize)]
pub struct WorkingSound {
    #[serde(flatten)]
    pub sound: WorkingSoundSoundDef, // If property not present, Sound is constructed from WorkingSound fields
    #[serde(default = "default_from_i8::<f32, 1>")]
    pub apparent_volume: f32, // Default: 1
    pub max_sounds_per_type: Option<u8>,
    #[serde(default = "default_bool::<false>")]
    pub match_progress_to_activity: bool, // Default: false
    #[serde(default = "default_bool::<false>")]
    pub match_volume_to_activity: bool, // Default: false
    #[serde(default = "default_bool::<false>")]
    pub match_speed_to_activity: bool, // Default: false
    #[serde(default = "default_bool::<false>")]
    pub persistent: bool, // Default: false
    #[serde(default = "default_bool::<true>")]
    pub use_doppler_shift: bool, // Default: true
    #[serde(default = "default_from_i8::<f64, 1>")]
    pub audible_distance_modifier: f64, // Default: 1
    #[serde(default = "default_from_i8::<f64, 1>")]
    pub probability: f64, // Default: 1
    #[serde(default = "default_u32::<0>")]
    pub fade_in_ticks: u32, // Default: 0
    #[serde(default = "default_u32::<0>")]
    pub fade_out_ticks: u32, // Default: 0
    pub idle_sound: Option<Sound>,
    pub activate_sound: Option<Sound>,
    pub deactivate_sound: Option<Sound>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WorkingSoundSoundDef {
    Separate { sound: Sound },
    Combined(Sound),
}

/// <https://wiki.factorio.com/Types/InterruptibleSound>
#[derive(Debug, Clone, Deserialize)]
pub struct InterruptibleSound {
    pub sound: Sound,
    #[serde(default = "default_u32::<0>")]
    pub fade_ticks: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/CyclicSound>
#[derive(Debug, Clone, Deserialize)]
pub struct CyclicSound {
    pub begin_sound: Option<Sound>,
    pub middle_sound: Option<Sound>,
    pub end_sound: Option<Sound>,
}

/// <https://wiki.factorio.com/Prototype/Tile#build_sound>
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TileBuildSound {
    Single(Sound),
    Many {
        small: Option<Sound>,
        medium: Option<Sound>,
        large: Option<Sound>,
    },
}
