use crate::types::FileName;

/// <https://wiki.factorio.com/Types/LayeredSound>
pub type LayeredSound = Vec<Sound>; // `layers`

/// <https://wiki.factorio.com/Types/Sound>
#[derive(Debug)]
pub struct Sound {
    aggregation: Option<SoundAggregation>,
    allow_random_repeat: Option<bool>,
    audible_distance_modifier: Option<f64>,
    variations: Vec<SoundVariation> // If variations table not present, use the same table, but construct single variation.
}

/// <https://wiki.factorio.com/Types/Sound#aggregation>
#[derive(Debug)]
pub struct SoundAggregation {
    max_count: u32,
    progress_threshold: Option<f32>,
    remove: bool,
    count_already_playing: Option<bool>
}

/// <https://wiki.factorio.com/Types/Sound#variations>
#[derive(Debug)]
pub struct SoundVariation {
    filename: FileName,
    volume: Option<f32>,
    preload: Option<bool>,
    speed: Option<f32>,
    min_speed: Option<f32>, // >= 1/64, Ignored if speed is present
    max_speed: Option<f32>  // Mandatory if min_speed is present, >= min_speed
}

/// <https://wiki.factorio.com/Types/WorkingSound>
#[derive(Debug)]
pub struct WorkingSound {
    sound: Sound, // If property not present, Sound is constructed from WorkingSound fields
    apparent_volume: f32, // Default: 1
    max_sounds_per_type: Option<u8>,
    match_progress_to_activity: bool, // Default: false
    match_volume_to_activity: bool, // Default: false
    match_speed_to_activity: bool, // Default: false
    persistent: bool, // Default: false
    use_doppler_shift: bool, // Default: true
    audible_distance_modifier: bool, // Default: 1
    probability: f64, // Default: 1
    fade_in_ticks: u32, // Default: 0
    fade_out_ticks: u32, // Default: 0
    idle_sound: Option<Sound>,
    activate_sound: Option<Sound>,
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
