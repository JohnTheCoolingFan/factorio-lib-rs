use mlua::{prelude::*, Lua};
use crate::prototypes::{GetPrototype, DataTable};
use factorio_lib_rs_derive::PrototypeFromLua;

use super::FileName;

/// <https://wiki.factorio.com/Types/LayeredSound>
pub type LayeredSound = Vec<Sound>; // `layers`

/// <https://wiki.factorio.com/Types/Sound>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct Sound {
    pub aggregation: Option<SoundAggregation>,
    #[default(false)]
    pub allow_random_repeat: bool,
    #[default(1.0)]
    pub audible_distance_modifier: f64,
    #[use_self_vec]
    pub variations: Vec<SoundVariation> // If variations table not present, use the same table, but construct single variation.
}

/// <https://wiki.factorio.com/Types/Sound#aggregation>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SoundAggregation {
    pub max_count: u32,
    #[default(1.0)]
    pub progress_threshold: f32,
    pub remove: bool,
    #[default(false)]
    pub count_already_playing: bool
}

/// <https://wiki.factorio.com/Types/Sound#variations>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct SoundVariation {
    #[resource]
    pub filename: FileName,
    #[default(1.0)]
    pub volume: f32,
    pub preload: Option<bool>, // Strange that this doesn't have a default
    pub speed: Option<f32>,
    pub min_speed: Option<f32>, // >= 1/64, Ignored if speed is present
    pub max_speed: Option<f32>  // Mandatory if min_speed is present, >= min_speed
}

impl SoundVariation {
    fn post_extr_fn(&mut self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if let Some(speed) = self.speed {
            if speed < (1.0 / 64.0) {
                return Err(mlua::Error::FromLuaConversionError { from: "table", to: "Sound.variations", message: Some("`speed` must be >= 1/64".into()) })
            };
            self.min_speed = self.speed;
            self.max_speed = self.speed;
        } else {
            self.speed = Some(1.0);
            if let Some(min_speed) = self.min_speed {
                if min_speed < (1.0 / 64.0) {
                    return Err(mlua::Error::FromLuaConversionError { from: "table", to: "Sound.variations", message: Some("`min_speed` must be >= 1/64".into()) })
                }
            } else {
                self.min_speed = Some(1.0)
            }
            if let Some(max_speed) = self.max_speed {
                if max_speed < self.min_speed.unwrap() {
                    return Err(mlua::Error::FromLuaConversionError { from: "table", to: "Sound.variations", message: Some("`max_speed` must be >= `min_speed`".into()) })
                }
            } else {
                self.max_speed = self.min_speed
            }
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/WorkingSound>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct WorkingSound {
    #[use_self]
    pub sound: Sound, // If property not present, Sound is constructed from WorkingSound fields
    #[default(1.0)]
    pub apparent_volume: f32, // Default: 1
    pub max_sounds_per_type: Option<u8>,
    #[default(false)]
    pub match_progress_to_activity: bool, // Default: false
    #[default(false)]
    pub match_volume_to_activity: bool, // Default: false
    #[default(false)]
    pub match_speed_to_activity: bool, // Default: false
    #[default(false)]
    pub persistent: bool, // Default: false
    #[default(true)]
    pub use_doppler_shift: bool, // Default: true
    #[default(1.0)]
    pub audible_distance_modifier: f64, // Default: 1
    #[default(1.0)]
    pub probability: f64, // Default: 1
    #[default(0_u32)]
    pub fade_in_ticks: u32, // Default: 0
    #[default(0_u32)]
    pub fade_out_ticks: u32, // Default: 0
    pub idle_sound: Option<Sound>,
    pub activate_sound: Option<Sound>,
    pub deactivate_sound: Option<Sound>,
}

/// <https://wiki.factorio.com/Types/InterruptibleSound>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct InterruptibleSound {
    pub sound: Sound,
    #[default(0_u32)]
    pub fade_ticks: u32 // Default: 0
}

/// <https://wiki.factorio.com/Types/CyclicSound>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct CyclicSound {
    pub begin_sound: Option<Sound>,
    pub middle_sound: Option<Sound>,
    pub end_sound: Option<Sound>
}

/// <https://wiki.factorio.com/Prototype/Tile#build_sound>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct TileBuildSound {
    #[use_self]
    pub small: Sound,
    pub medium: Option<Sound>,
    pub large: Option<Sound>
}
