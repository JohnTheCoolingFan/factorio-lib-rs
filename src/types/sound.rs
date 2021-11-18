use crate::PrototypeFromLua;
use factorio_lib_rs_derive::PrototypeFromLua;
use mlua::{prelude::LuaResult, Lua, Value};
use crate::{DataTable, ResourceRecord, ResourceType};
use mlua::FromLua;

use crate::types::FileName;

/// <https://wiki.factorio.com/Types/LayeredSound>
pub type LayeredSound = Vec<Sound>; // `layers`

/// <https://wiki.factorio.com/Types/Sound>
#[derive(Debug, PrototypeFromLua)]
pub struct Sound {
    aggregation: Option<SoundAggregation>,
    #[default(false)]
    allow_random_repeat: bool,
    #[default(1.0)]
    audible_distance_modifier: f64,
    #[use_self_if_not_found]
    #[prototype]
    variations: Vec<SoundVariation> // If variations table not present, use the same table, but construct single variation.
}

/// <https://wiki.factorio.com/Types/Sound#aggregation>
#[derive(Debug, PrototypeFromLua)]
pub struct SoundAggregation {
    max_count: u32,
    #[default(1.0)]
    progress_threshold: f32,
    remove: bool,
    #[default(false)]
    count_already_playing: bool
}

/// <https://wiki.factorio.com/Types/Sound#variations>
#[derive(Debug, PrototypeFromLua)]
pub struct SoundVariation {
    #[resource]
    filename: FileName,
    #[default(1.0)]
    volume: f32,
    preload: Option<bool>,
    #[default(1.0)]
    speed: f32,
    min_speed: Option<f32>, // >= 1/64, Ignored if speed is present
    max_speed: Option<f32>  // Mandatory if min_speed is present, >= min_speed
}

/// <https://wiki.factorio.com/Types/WorkingSound>
#[derive(Debug, PrototypeFromLua)]
pub struct WorkingSound {
    #[use_self_if_not_found]
    #[prototype]
    sound: Sound, // If property not present, Sound is constructed from WorkingSound fields
    #[default(1.0)]
    apparent_volume: f32, // Default: 1
    max_sounds_per_type: Option<u8>,
    #[default(false)]
    match_progress_to_activity: bool, // Default: false
    #[default(false)]
    match_volume_to_activity: bool, // Default: false
    #[default(false)]
    match_speed_to_activity: bool, // Default: false
    #[default(false)]
    persistent: bool, // Default: false
    #[default(true)]
    use_doppler_shift: bool, // Default: true
    #[default(1.0)]
    audible_distance_modifier: f64, // Default: 1
    #[default(1.0)]
    probability: f64, // Default: 1
    #[default(0)]
    fade_in_ticks: u32, // Default: 0
    #[default(0)]
    fade_out_ticks: u32, // Default: 0
    #[prototype]
    idle_sound: Option<Sound>,
    #[prototype]
    activate_sound: Option<Sound>,
    #[prototype]
    deactivate_sound: Option<Sound>,
}

/// <https://wiki.factorio.com/Types/InterruptibleSound>
#[derive(Debug)]
pub struct InterruptibleSound {
    sound: Sound,
    fade_ticks: u32 // Default: 0
}

/// <https://wiki.factorio.com/Types/CyclicSound>
#[derive(Debug)]
pub struct CyclicSound {
    begin_sound: Option<Sound>,
    middle_sound: Option<Sound>,
    end_sound: Option<Sound>
}

/// <https://wiki.factorio.com/Prototype/Tile#build_sound>
#[derive(Debug)]
pub struct TileBuildSound {
    small: Sound,
    medium: Option<Sound>,
    large: Option<Sound>
}
