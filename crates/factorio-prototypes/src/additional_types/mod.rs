mod attack_parameters;
mod autoplace;
mod capsule_action;
mod graphics;
mod sound;
mod style_specification;
mod tile_transitions;
mod tip_trigger;
mod trigger;

pub use attack_parameters::*;
pub use autoplace::*;
pub use capsule_action::*;
pub use graphics::*;
use serde::de::{self, Visitor};
use serde_with::serde_as;
pub use sound::*;
pub use style_specification::*;
use thiserror::Error;
pub use tile_transitions::*;
pub use tip_trigger::*;
pub use trigger::*;

use super::{Base, DataTable, LocalisedString, PrototypeFromLua};
use crate::prototypes::{GetPrototype, PrototypesErr};
use crate::util::fixed_serde;
use factorio_lib_rs_derive::prot_from_str;
use fixed::types::I24F8;
use mlua::{prelude::*, FromLua, Lua, ToLua, Value};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::convert::From;
use std::ffi::OsStr;
use std::fmt::{self, Display};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use std::path::Path;
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumDiscriminants, EnumString};

/// <https://wiki.factorio.com/Types/FileName>
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct FileName<FT: FileType = ImageFileType> {
    pub name: String,
    phantom: PhantomData<FT>,
}

pub trait FileType: Copy {
    fn validate_extension(ext: &str) -> bool;
    fn validate_extension_osstr(ext: &OsStr) -> bool {
        ext.to_str()
            .map(|extstr| Self::validate_extension(extstr))
            .unwrap_or(false)
    }
    fn validate_path(path: &Path) -> Option<bool> {
        Some(Self::validate_extension_osstr(path.extension()?))
    }
    fn validate_filename_osstr(filename: &OsStr) -> Option<bool> {
        Self::validate_filename(filename.to_str()?)
    }
    fn validate_filename(filename: &str) -> Option<bool> {
        Some(Self::validate_extension(filename.rsplit_once('.')?.1))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SoundFileType;

impl FileType for SoundFileType {
    fn validate_extension(ext: &str) -> bool {
        matches!(ext, "ogg" | "wav" | "voc")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageFileType;

impl FileType for ImageFileType {
    fn validate_extension(ext: &str) -> bool {
        matches!(ext, "png")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SaveFileType;

impl FileType for SaveFileType {
    fn validate_extension(ext: &str) -> bool {
        matches!(ext, "zip")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScriptFileType;

impl FileType for ScriptFileType {
    fn validate_extension(ext: &str) -> bool {
        matches!(ext, "lua")
    }
}

/// <https://wiki.factorio.com/Types/ItemStackIndex>
pub type ItemStackIndex = u16;
/// <https://wiki.factorio.com/Types/ItemCountType>
pub type ItemCountType = u32;
// Type derived from Factorio3DVector definition (https://wiki.factorio.com/Types/Vector3D)
/// 2D Vector defined by Factorio <https://wiki.factorio.com/Types/vector>
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Default)]
pub struct Factorio2DVector(pub f32, pub f32);

/// 3D Vector defined by Factorio <https://wiki.factorio.com/Types/Vector3D>
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Default)]
pub struct Factorio3DVector(pub f32, pub f32, pub f32);

// Parser and checker maybe?
/// Keyboard keys sequence <https://wiki.factorio.com/Prototype/CustomInput#key_sequence>
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct KeySequence(pub String);

/// <https://wiki.factorio.com/Types/BoundingBox>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
pub struct BoundingBox(pub Position, pub Position);

impl From<((f32, f32), (f32, f32))> for BoundingBox {
    fn from(bb: ((f32, f32), (f32, f32))) -> Self {
        Self(bb.0.into(), bb.1.into())
    }
}

impl From<BoundingBox> for ((f32, f32), (f32, f32)) {
    fn from(bb: BoundingBox) -> Self {
        (bb.0.into(), bb.1.into())
    }
}
impl From<&((f32, f32), (f32, f32))> for BoundingBox {
    fn from(bb: &((f32, f32), (f32, f32))) -> Self {
        Self(bb.0.into(), bb.1.into())
    }
}

impl From<&BoundingBox> for ((f32, f32), (f32, f32)) {
    fn from(bb: &BoundingBox) -> Self {
        (bb.0.into(), bb.1.into())
    }
}

impl BoundingBox {
    pub fn larger_than(&self, rhs: &Self) -> bool {
        let lhs: ((f32, f32), (f32, f32)) = self.into();
        let rhs: ((f32, f32), (f32, f32)) = rhs.into();
        let lhs_width = (lhs.0 .0 - lhs.1 .0).abs();
        let lhs_height = (lhs.0 .1 - lhs.1 .1).abs();
        let rhs_width = (rhs.0 .0 - rhs.1 .0).abs();
        let rhs_height = (rhs.0 .1 - rhs.1 .1).abs();
        lhs_width >= rhs_width && lhs_height >= rhs_height
    }
}

#[test]
fn boundingbox_comparison() {
    let zero_boundingbox = BoundingBox::from(((0.0, 0.0), (0.0, 0.0)));
    let min_boundingbox = BoundingBox::from(((-0.0, -0.2), (0.0, 0.2)));
    let larger_boundingbox = BoundingBox::from(((-0.2, -0.4), (0.2, 0.4)));
    assert!(min_boundingbox.larger_than(&zero_boundingbox));
    assert!(!min_boundingbox.larger_than(&larger_boundingbox));
}

/// Value range: [0.0; 1.0) <https://wiki.factorio.com/Types/RealOrientation>
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Default)]
#[serde(try_from = "f32")]
pub struct RealOrientation(pub f32);

#[derive(Debug, Clone, Copy, Error)]
pub struct RealOrientationCheckError(f32);

impl Display for RealOrientationCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value must be in a range [0.0; 1.0), got {}", self.0)
    }
}

impl TryFrom<f32> for RealOrientation {
    type Error = RealOrientationCheckError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if (0.0..1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(RealOrientationCheckError(value))
        }
    }
}

/// Can be constructed from an array or table with x and y values <https://wiki.factorio.com/Types/Position>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(from = "PositionVariants")]
pub struct Position(I24F8, I24F8);

#[derive(Deserialize)]
#[serde(untagged)]
enum PositionVariants {
    Sequence(
        #[serde(with = "fixed_serde::as_f32")] I24F8,
        #[serde(with = "fixed_serde::as_f32")] I24F8,
    ),
    Table {
        #[serde(with = "fixed_serde::as_f32")]
        x: I24F8,
        #[serde(with = "fixed_serde::as_f32")]
        y: I24F8,
    },
}

impl From<PositionVariants> for Position {
    fn from(value: PositionVariants) -> Self {
        match value {
            PositionVariants::Sequence(x, y) | PositionVariants::Table { x, y } => Self(x, y),
        }
    }
}

impl From<(f32, f32)> for Position {
    fn from(v: (f32, f32)) -> Self {
        let (x, y) = v;
        let (x, y) = (I24F8::from_num(x), I24F8::from_num(y));
        Self(x, y)
    }
}

impl From<Position> for (f32, f32) {
    fn from(p: Position) -> Self {
        (p.0.to_num(), p.1.to_num())
    }
}

/// Any of the color components are optional <https://wiki.factorio.com/Types/Color>
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "ColorVariants")]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ColorVariants {
    Sequence(f32, f32, f32, #[serde(default = "default_alpha")] f32),
    Table {
        r: Option<f32>,
        g: Option<f32>,
        b: Option<f32>,
        a: Option<f32>,
    },
}

impl From<ColorVariants> for Color {
    fn from(value: ColorVariants) -> Self {
        match value {
            ColorVariants::Sequence(r, g, b, a) => Self::new_rgba(r, g, b, a),
            ColorVariants::Table { r, g, b, a } => Self::new_rgba_opt(r, g, b, a),
        }
    }
}

fn default_alpha() -> f32 {
    1.0
}

impl Color {
    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn new_rgba_opt(r: Option<f32>, g: Option<f32>, b: Option<f32>, a: Option<f32>) -> Self {
        let r = r.unwrap_or(0.0_f32);
        let g = g.unwrap_or(0.0_f32);
        let b = b.unwrap_or(0.0_f32);
        let a = a.unwrap_or(1.0_f32);
        Self { r, g, b, a }
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        // r, g, b default is 0
        Self { r, g, b, a: 1.0 }
    }
}

/// <https://lua-api.factorio.com/latest/defines.html#defines.difficulty_settings>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum DifficultySetting {
    Normal,
    Expensive,
}

/// <https://wiki.factorio.com/Prototype/MapSettings#difficulty_settings>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ResearchQueueSetting {
    AfterVictory,
    Always,
    Never,
}

/// <https://wiki.factorio.com/Tutorial:Mod_settings#The_setting_type_property>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ModSettingType {
    Startup,
    RuntimeGlobal,
    RuntimePerUser,
}

/// <https://wiki.factorio.com/Types/MapGenPreset>
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "MapGenPresetIntermediate")]
pub enum MapGenPreset {
    // Decided by `default` field
    Default(MapGenPresetDefault),
    NonDefault(Box<MapGenPresetNonDefault>),
}

#[derive(Deserialize)]
struct MapGenPresetIntermediate {
    #[serde(default = "default_true")]
    default: bool,
    order: String,
    basic_settings: Option<MapGenPresetBasicSettings>,
    advanced_settings: Option<MapGenPresetAdvancedSettings>,
}

impl From<MapGenPresetIntermediate> for MapGenPreset {
    fn from(value: MapGenPresetIntermediate) -> Self {
        match value.default {
            true => Self::Default(MapGenPresetDefault { order: value.order }),
            false => Self::NonDefault(Box::new(MapGenPresetNonDefault {
                order: value.order,
                basic_settings: value.basic_settings,
                advanced_settings: value.advanced_settings,
            })),
        }
    }
}

/// <https://wiki.factorio.com/Types/MapGenPreset#default>
#[derive(Debug, Clone)]
pub struct MapGenPresetDefault {
    order: String,
}

/// <https://wiki.factorio.com/Types/MapGenPreset#default>
#[derive(Debug, Clone)]
pub struct MapGenPresetNonDefault {
    order: String,
    // Should these be optional or just have defaults? TODO
    basic_settings: Option<MapGenPresetBasicSettings>,
    advanced_settings: Option<MapGenPresetAdvancedSettings>,
}

/// <https://wiki.factorio.com/Types/MapGenSize>
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize)]
#[serde(from = "MapGenSizeVariants")]
pub struct MapGenSize(pub f64); // Exact type is unknown, so slap a f64

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MapGenSizeVariants {
    None,
    VeryLow,
    VerySmall,
    VeryPoor,
    Low,
    Small,
    Poor,
    Normal,
    Medium,
    Regular,
    High,
    Big,
    Good,
    VeryHigh,
    VeryBig,
    VeryGood,
}

impl From<MapGenSizeVariants> for MapGenSize {
    fn from(value: MapGenSizeVariants) -> Self {
        match value {
            MapGenSizeVariants::None => Self(0.0),
            MapGenSizeVariants::VeryLow
            | MapGenSizeVariants::VerySmall
            | MapGenSizeVariants::VeryPoor => Self(0.5),
            MapGenSizeVariants::Low | MapGenSizeVariants::Small | MapGenSizeVariants::Poor => {
                Self(1.0 / (2.0_f64).sqrt())
            }
            MapGenSizeVariants::Normal
            | MapGenSizeVariants::Medium
            | MapGenSizeVariants::Regular => Self(1.0),
            MapGenSizeVariants::High | MapGenSizeVariants::Big | MapGenSizeVariants::Good => {
                Self((2.0_f64).sqrt())
            }
            MapGenSizeVariants::VeryHigh
            | MapGenSizeVariants::VeryBig
            | MapGenSizeVariants::VeryGood => Self(2.0),
        }
    }
}

/// <https://lua-api.factorio.com/latest/Concepts.html#CliffPlacementSettings>
#[derive(Debug, Clone, Deserialize)]
pub struct CliffPlacementSettings {
    pub name: String, // Name of the cliff prototype
    #[serde(default = "default_10_0_f32")]
    pub cliff_elevation_0: f32, // Default 10.0
    pub cliff_elevation_interval: f32,
    pub richness: MapGenSize,
}

fn default_10_0_f32() -> f32 {
    10.0
}

// TODO: defaults
// Quote: «All key/value pairs are optional. If not set they will just use the default values.»
/// <https://wiki.factorio.com/Types/MapGenPreset#basic_settings>
#[derive(Debug, Clone, Deserialize)]
pub struct MapGenPresetBasicSettings {
    // Defaults are not documented for some f'ing reason
    pub terain_segmentation: MapGenSize, // Default is... Unknown
    pub water: MapGenSize,               // Same here
    #[serde(default = "default_true")]
    pub default_enable_all_autoplace_controls: bool, // Default: true
    pub autoplace_controls: HashMap<String, AutoplaceSetting>, // key is AutoplaceControl name
    // Quote: «Each setting in this table maps the string type to the settings for that type. Valid types are "entity", "tile" and "decorative".»
    pub autoplace_settings: Vec<AutoplaceSettings>,
    pub property_expression_names: HashMap<String, String>, // Map property name to noise expression name
    pub starting_points: Vec<Position>,
    pub seed: u32,
    pub width: u32,
    pub height: u32,
    pub starting_area: MapGenSize,
    pub peaceful_mode: bool,
    pub cliff_settings: CliffPlacementSettings,
}

fn default_true() -> bool {
    true
}

/// <https://wiki.factorio.com/Types/MapGenPreset#basic_settings>
/// <https://lua-api.factorio.com/latest/Concepts.html#AutoplaceSettings>
#[derive(Debug, Clone, Deserialize)]
pub struct AutoplaceSettings {
    pub treat_missing_as_default: bool, // Doesn't look like it's optional or has a default...
    pub settings: HashMap<String, AutoplaceSetting>,
}

/// <https://lua-api.factorio.com/latest/Concepts.html#AutoplaceSetting>
#[derive(Debug, Clone, Deserialize)]
pub struct AutoplaceSetting {
    pub frequency: Option<MapGenSize>,
    pub size: Option<MapGenSize>,
    pub richness: Option<MapGenSize>,
}

// About defaults, quote: «All key/value pairs are optional, if not set they will just use the
// existing values.»
/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug, Clone, Deserialize)]
pub struct MapGenPresetAdvancedSettings {
    // Defaults are not documented too
    pub pollution: MapGenPollution,
    pub enemy_evolution: MapGenEnemyEvolution,
    pub enemy_expansion: MapGenEnemyExpansion,
    pub difficulty_settings: MapGenDifficultySettings,
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "MapGenPollutionIntermediate")]
pub struct MapGenPollution {
    pub enabled: bool,
    pub diffusion_ratio: f64, // Must be <= 0.25
    pub ageing: f64,          // Must be >= 0.5
    pub enemy_attack_pollution_consumption_modifier: f64,
    pub min_pollution_to_damage_trees: f64,
    pub pollution_restored_per_tree_damage: f64,
}

#[derive(Deserialize)]
struct MapGenPollutionIntermediate {
    pub enabled: bool,
    pub diffusion_ratio: f64, // Must be <= 0.25
    pub ageing: f64,          // Must be >= 0.5
    pub enemy_attack_pollution_consumption_modifier: f64,
    pub min_pollution_to_damage_trees: f64,
    pub pollution_restored_per_tree_damage: f64,
}

#[derive(Debug, Clone, Copy, Error)]
enum MapGenPollutionCheckError {
    #[error("diffusion_ratio must be <= 0.25, got {0}")]
    DiffusionRatio(f64),
    #[error("ageing must be >= 0.25, got {0}")]
    Ageing(f64),
}

impl TryFrom<MapGenPollutionIntermediate> for MapGenPollution {
    type Error = MapGenPollutionCheckError;

    fn try_from(value: MapGenPollutionIntermediate) -> Result<Self, Self::Error> {
        let MapGenPollutionIntermediate {
            enabled,
            diffusion_ratio,
            ageing,
            enemy_attack_pollution_consumption_modifier,
            min_pollution_to_damage_trees,
            pollution_restored_per_tree_damage,
        } = value;

        if diffusion_ratio > 0.25 {
            return Err(MapGenPollutionCheckError::DiffusionRatio(diffusion_ratio));
        }
        if ageing < 0.25 {
            return Err(MapGenPollutionCheckError::Ageing(ageing));
        }
        Ok(Self {
            enabled,
            diffusion_ratio,
            ageing,
            enemy_attack_pollution_consumption_modifier,
            min_pollution_to_damage_trees,
            pollution_restored_per_tree_damage,
        })
    }
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug, Clone, Deserialize)]
pub struct MapGenEnemyEvolution {
    pub enabled: bool,
    pub time_factor: f64,
    pub destroy_factor: f64,
    pub pollution_factor: f64,
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug, Clone, Deserialize)]
pub struct MapGenEnemyExpansion {
    pub enabled: bool,
    // Oddly satisfying how field names line up
    pub max_expansion_distance: f64,
    pub settler_group_min_size: f64,
    pub settler_group_max_size: f64,
    pub max_expansion_cooldown: f64,
    pub min_expansion_cooldown: f64,
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug, Clone, Deserialize)]
pub struct MapGenDifficultySettings {
    pub recipe_difficulty: DifficultySetting,
    pub technology_difficulty: DifficultySetting,
    pub technology_price_multiplier: f64,
    pub research_queue_setting: ResearchQueueSetting,
}

/// <https://wiki.factorio.com/Prototype/MapSettings#pollution>
#[derive(Debug, Clone, Deserialize)]
pub struct MapPollutionSettings {
    pub enabled: bool,
    pub diffusion_ratio: f64,
    pub min_to_diffuse: f64,
    pub ageing: f64,
    pub expected_max_per_chunk: f64,
    pub min_to_show_per_chunk: f64,
    pub min_pollution_to_damage_trees: f64,
    pub pollution_with_max_forest_damage: f64,
    pub pollution_restored_per_tree_damage: f64,
    pub pollution_per_tree_damage: f64,
    pub max_pollution_to_restore_trees: f64,
    pub enemy_attack_pollution_consumption_modifier: f64,
}

/// <https://wiki.factorio.com/Prototype/MapSettings#steering>
#[derive(Debug, Clone, Deserialize)]
pub struct MapSteering {
    pub default: MapSteeringSettings,
    pub moving: MapSteeringSettings,
}

/// <https://wiki.factorio.com/Prototype/MapSettings#steering>
#[derive(Debug, Clone, Deserialize)]
pub struct MapSteeringSettings {
    pub radius: f64,
    pub separation_factor: f64,
    pub separation_force: f64,
    pub force_unit_fuzzy_goto_behavior: bool,
}

/// <https://wiki.factorio.com/Prototype/MapSettings#enemy_evolution>
#[derive(Debug, Clone, Deserialize)]
pub struct MapEnemyEvolution {
    pub enabled: bool,
    pub time_factor: f64,
    pub destroy_factor: f64,
    pub pollution_factor: f64,
}

/// <https://wiki.factorio.com/Prototype/MapSettings#enemy_expansion>
#[derive(Debug, Clone, Deserialize)]
pub struct MapEnemyExpansion {
    pub enabled: bool,
    pub max_expansion_distance: u32,
    pub friendly_base_influence_radius: u32,
    pub enemy_building_influence_radius: u32,
    pub building_coefficient: f64,
    pub other_base_coefficient: f64,
    pub neighbouring_chunk_coefficient: f64,
    pub neighbouring_base_chunk_coefficient: f64,
    pub max_colliding_tiles_coefficient: f64,
    pub settler_group_min_size: u32,
    pub settler_group_max_size: u32,
    pub min_expansion_cooldown: u32,
    pub max_expansion_cooldown: u32,
}

/// <https://wiki.factorio.com/Prototype/MapSettings#unit_group>
#[derive(Debug, Clone, Deserialize)]
pub struct MapUnitGroup {
    pub min_group_gathering_time: u32,
    pub max_group_gathering_time: u32,
    pub max_wait_time_for_late_members: u32,
    pub max_group_radius: f64,
    pub min_group_radius: f64,
    pub max_member_speedup_when_behind: f64,
    pub max_member_slowdown_when_ahead: f64,
    pub max_group_slowdown_facor: f64,
    pub max_group_member_fallback_factor: f64,
    pub member_disown_distance: f64,
    pub tick_tolerance_when_member_arrives: u32,
    pub max_gathering_unit_groups: u32,
    pub max_unit_group_size: u32,
}

/// <https://wiki.factorio.com/Prototype/MapSettings#path_finder>
#[derive(Debug, Clone, Deserialize)]
pub struct MapPathFinder {
    #[serde(deserialize_with = "fwd2bwd_ratio_deser_check")]
    pub fwd2bwd_ratio: u32,
    pub goal_pressure_ratio: f64,
    pub use_path_cache: bool,
    pub max_steps_worked_per_tick: f64,
    pub max_work_done_per_tick: u32,
    pub short_cache_size: u32,
    pub short_cache_min_cacheable_distance: f64,
    pub short_cache_min_algo_steps_to_cache: u32,
    pub long_cache_min_cacheable_distance: f64,
    pub cache_max_connect_to_cache_steps_multiplier: u32,
    pub cache_accept_path_start_distance_ratio: f64,
    pub cache_accept_path_end_distance_ratio: f64,
    pub negative_cache_accept_path_start_distance_ratio: f64,
    pub negative_cache_accept_path_end_distance_ratio: f64,
    pub cache_path_start_distance_rating_multiplier: f64,
    pub cache_path_end_distance_rating_multiplier: f64,
    pub stale_enemy_with_same_destination_collision_penalty: f64,
    pub ignore_moving_enemy_collision_distance: f64,
    pub enemy_with_different_destination_collision_penalty: f64,
    pub general_entity_collision_penalty: f64,
    pub general_entity_subsequent_collision_penalty: f64,
    pub extended_collision_penalty: f64,
    pub max_clients_to_accept_any_new_request: u32,
    pub max_clients_to_accept_short_new_request: u32,
    pub direct_distance_to_consider_short_request: u32,
    pub short_request_max_steps: u32,
    pub short_request_ratio: f64,
    pub min_steps_to_check_path_find_termination: u32,
    pub start_to_goal_cost_multiplier_to_terminate_path_find: f64,
    pub overload_levels: Vec<u32>,
    pub overload_multipliers: Vec<f64>,
}

fn fwd2bwd_ratio_deser_check<'de, D>(deser: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let result = u32::deserialize(deser)?;
    if result < 2 {
        Err(de::Error::custom(format!(
            "`fwd2bwd_ratio` must not be less than 2, got {result}"
        )))
    } else {
        Ok(result)
    }
}

/// <https://wiki.factorio.com/Prototype/MapSettings#difficulty_settings>
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "MapDifficultySettingsIntermediate")]
pub struct MapDifficultySettings {
    pub recipe_difficulty: DifficultySetting,
    pub technology_difficulty: DifficultySetting,
    pub technology_price_multiplier: f64, // Default: 1.0 // Must be >= 0.001 and <= 1000.0
    pub research_queue_setting: Option<ResearchQueueSetting>,
}

#[derive(Deserialize)]
struct MapDifficultySettingsIntermediate {
    pub recipe_difficulty: DifficultySetting,
    pub technology_difficulty: DifficultySetting,
    #[serde(default = "default_1_0_f64")]
    pub technology_price_multiplier: f64, // Default: 1.0 // Must be >= 0.001 and <= 1000.0
    pub research_queue_setting: Option<ResearchQueueSetting>,
}

fn default_1_0_f64() -> f64 {
    1.0
}

#[derive(Debug, Clone, Copy, Error)]
enum MapDifficultySettingsCheckError {
    #[error("`technology_price_multiplier` should be in a range [0.001, 1000.0], got {0}")]
    TechnologyPriceMultiplier(f64),
}

impl TryFrom<MapDifficultySettingsIntermediate> for MapDifficultySettings {
    type Error = MapDifficultySettingsCheckError;

    fn try_from(value: MapDifficultySettingsIntermediate) -> Result<Self, Self::Error> {
        let MapDifficultySettingsIntermediate {
            recipe_difficulty,
            technology_difficulty,
            technology_price_multiplier,
            research_queue_setting,
        } = value;

        if technology_price_multiplier < 0.001 || technology_price_multiplier > 1000.0 {
            return Err(MapDifficultySettingsCheckError::TechnologyPriceMultiplier(
                technology_price_multiplier,
            ));
        }

        Ok(Self {
            recipe_difficulty,
            technology_difficulty,
            technology_price_multiplier,
            research_queue_setting,
        })
    }
}

/// <https://wiki.factorio.com/Prototype/MouseCursor>
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MouseCursorType {
    SystemCursor { system_cursor: SystemCursor },
    CustomCursor(CustomCursor),
}

/// <https://wiki.factorio.com/Prototype/MouseCursor#system_cursor>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum SystemCursor {
    Arrow,
    IBeam,
    Crosshair,
    WaitArrow,
    SizeAll,
    No,
    Hand,
}

/// <https://wiki.factorio.com/Prototype/MouseCursor>
#[derive(Debug, Clone, Deserialize)]
pub struct CustomCursor {
    pub filename: FileName,
    pub hot_pixel_x: i16,
    pub hot_pixel_y: i16,
}

// Make different constructors for variants with different field names, like `icon_tintable` in https://wiki.factorio.com/Prototype/ItemWithEntityData
/// <https://wiki.factorio.com/Types/IconSpecification>
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum IconSpecification {
    Icons(IconsSpec),
    Icon(IconSpec),
}

/// <https://wiki.factorio.com/Types/IconSpecification#Prototype_properties:_Option_2>
#[derive(Debug, Clone, Deserialize)]
pub struct IconSpec {
    pub icon: FileName,
    pub icon_size: i16,
    #[serde(default)]
    pub icon_mipmaps: u8, // Default: 0
}

/// <https://wiki.factorio.com/Types/IconSpecification#Prototype_properties:_Option_1>
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "IconsSpecIntermediate")]
pub struct IconsSpec {
    pub icons: Vec<IconData>,
    // icon_size omitted here, it will be copied to each IconData
    pub icon_mipmaps: u8, // Default: 0
}

#[derive(Deserialize)]
struct IconsSpecIntermediate {
    icons: Vec<IconDataIntermediate>,
    icon_size: Option<i16>,
    #[serde(default)]
    icon_mipmaps: u8,
}

#[derive(Debug, Clone, Copy, Error)]
enum IconsSpecCheckError {
    #[error("`icon_size` must be set if not all items of `icons` contain it, `icon_size` not found at index {index}")]
    MissingIconSize { index: usize },
}

impl TryFrom<IconsSpecIntermediate> for IconsSpec {
    type Error = IconsSpecCheckError;

    fn try_from(value: IconsSpecIntermediate) -> Result<Self, Self::Error> {
        let IconsSpecIntermediate {
            icons,
            icon_size,
            icon_mipmaps,
        } = value;

        Ok(Self {
            icons: icons
                .into_iter()
                .enumerate()
                .map(|(i, idi)| {
                    let IconDataIntermediate {
                        icon,
                        icon_size: int_icon_size,
                        tint,
                        shift,
                        scale,
                        icon_mipmaps,
                    } = idi;
                    Result::<_, Self::Error>::Ok(IconData {
                        icon,
                        icon_size: int_icon_size.map(|v| Ok(v)).unwrap_or_else(|| {
                            int_icon_size
                                .ok_or_else(|| IconsSpecCheckError::MissingIconSize { index: i })
                        })?,
                        tint,
                        shift,
                        scale,
                        icon_mipmaps,
                    })
                })
                .collect::<Result<_, _>>()?,
            icon_mipmaps,
        })
    }
}

/// <https://wiki.factorio.com/Types/IconData>
#[derive(Deserialize)]
struct IconDataIntermediate {
    pub icon: FileName,
    pub icon_size: Option<i16>, // Copied from `icon_size` from prototype
    #[serde(default)]
    pub tint: Color, // Default: (0, 0, 0 , 1)
    #[serde(default)]
    pub shift: Factorio2DVector, // Default: (0, 0)
    #[serde(default = "default_1_0_f64")]
    pub scale: f64, // Default: 1
    #[serde(default)]
    pub icon_mipmaps: u8, // Default: 0
}

#[derive(Debug, Clone)]
pub struct IconData {
    pub icon: FileName,
    pub icon_size: i16,          // Copied from `icon_size` from prototype
    pub tint: Color,             // Default: (0, 0, 0 , 1)
    pub shift: Factorio2DVector, // Default: (0, 0)
    pub scale: f64,              // Default: 1
    pub icon_mipmaps: u8,        // Default: 0
}

// TODO: fmt::Display
/// Input data is converted to J/tick or Joule
/// J/s (Joule/second) is not supported, as I can't find any uses and it's equvalent to W (Watt)
/// <https://wiki.factorio.com/Types/Energy>
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Deserialize)]
#[serde(try_from = "String")]
pub struct Energy(pub f64); // I don't know which type factorio uses internally, so I will use this

impl TryFrom<String> for Energy {
    type Error = <Energy as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Energy {
    fn get_multiplier(multiplier_char: char) -> Option<f64> {
        match multiplier_char {
            'k' | 'K' => Some(1e3),
            'M' => Some(1e6),
            'G' => Some(1e9),
            'T' => Some(1e12),
            'P' => Some(1e15),
            'E' => Some(1e18),
            'Z' => Some(1e21),
            'Y' => Some(1e24),
            _ => None,
        }
    }

    fn err_fn(s: &str) -> PrototypesErr {
        PrototypesErr::InvalidTypeStr("Energy".into(), s.into())
    }

    fn split_num_and_suffix(s: &str) -> Option<(&str, &str)> {
        let mut chars = s.chars();
        // None returned if string is too short
        chars.next_back()?;
        let second_last_char = chars.next_back()?;
        // Panics if split is on UTF-8 boundary
        Some(s.split_at(s.len() - 2 + (second_last_char.is_ascii_digit() as usize)))
    }
}

impl FromStr for Energy {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('W') || s.ends_with('J') {
            let (num, suffix) = Self::split_num_and_suffix(s).ok_or_else(|| Self::err_fn(s))?;
            let multiplier = suffix
                .chars()
                .next()
                .and_then(Self::get_multiplier)
                .ok_or_else(|| Self::err_fn(s))?;
            let mut value = num.parse::<f64>().map_err(|_| Self::err_fn(s))?;
            if s.ends_with('W') {
                value /= 60.0
            };
            Ok(Self(value * multiplier))
        } else {
            Err(Self::err_fn(s))
        }
    }
}

#[test]
fn energy_parse() {
    assert_eq!(Energy(1e3), Energy::from_str("1kJ").unwrap());
    assert_eq!(Energy(1e9), Energy::from_str("1000.0MJ").unwrap());
    assert_eq!(Energy(1e3 / 60.0), Energy::from_str("1kW").unwrap());
    assert_eq!(Energy(1e3 / 60.0), Energy::from_str("1KW").unwrap());
    assert_eq!(Energy(1246890.0), Energy::from_str("1246.89kJ").unwrap());
    assert!(Energy::from_str("1k").is_err());
    assert!(Energy::from_str("1000.0").is_err());
    assert!(Energy::from_str("").is_err())
}

/// <https://wiki.factorio.com/Prototype/ProduceAchievement#item_product>
/// <https://wiki.factorio.com/Prototype/ProducePerHourAchievement#item_product>
#[derive(Debug, Clone, Deserialize)]
pub enum ProductType {
    #[serde(rename = "item_product")]
    Item(String),
    #[serde(rename = "fluid_product")]
    Fluid(String),
}

/// <https://wiki.factorio.com/Prototype/ResearchAchievement>
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "ResearchTargetIntermediate")]
pub enum ResearchTarget {
    All,
    Technology(String),
}

#[derive(Deserialize)]
struct ResearchTargetIntermediate {
    #[serde(default)]
    technology: String,
    #[serde(default)]
    research_all: bool,
}

impl From<ResearchTargetIntermediate> for ResearchTarget {
    fn from(value: ResearchTargetIntermediate) -> Self {
        if value.research_all {
            Self::All
        } else {
            Self::Technology(value.technology)
        }
    }
}

/// <https://wiki.factorio.com/Prototype/AutoplaceControl#category>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum AutoplaceControlCategory {
    Resource,
    Terrain,
    Enemy,
}

/// <https://wiki.factorio.com/Prototype/CustomInput#consuming>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ConsumingType {
    None,
    GameOnly,
}

/// <https://wiki.factorio.com/Prototype/CustomInput#action>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum CustomInputAction {
    Lua,
    SpawnItem,
    TogglePersonalRoboport,
    TogglePersonalLogisticRequests,
    ToggleEquipmentMovementBonus,
}

/// <https://wiki.factorio.com/Types/CollisionMask>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, Default)]
pub struct CollisionMask(pub(crate) u64);

impl CollisionMask {
    pub const GROUND_TILE: CollisionMask = CollisionMask(1);
    pub const WATER_TILE: CollisionMask = CollisionMask(1 << 1);
    pub const RESOURCE_LAYER: CollisionMask = CollisionMask(1 << 2);
    pub const DOODAD_LAYER: CollisionMask = CollisionMask(1 << 3);
    pub const FLOOR_LAYER: CollisionMask = CollisionMask(1 << 4);
    pub const ITEM_LAYER: CollisionMask = CollisionMask(1 << 5);
    pub const GHOST_LAYER: CollisionMask = CollisionMask(1 << 6);
    pub const OBJECT_LAYER: CollisionMask = CollisionMask(1 << 7);
    pub const PLAYER_LAYER: CollisionMask = CollisionMask(1 << 8);
    pub const TRAIN_LAYER: CollisionMask = CollisionMask(1 << 9);
    pub const RAIL_LAYER: CollisionMask = CollisionMask(1 << 10);
    pub const TRANSPORT_BELT_LAYER: CollisionMask = CollisionMask(1 << 11);
    pub const LAYER_13: CollisionMask = CollisionMask(1 << 12);
    pub const LAYER_14: CollisionMask = CollisionMask(1 << 13);
    pub const LAYER_15: CollisionMask = CollisionMask(1 << 14);
    pub const LAYER_16: CollisionMask = CollisionMask(1 << 15);
    pub const LAYER_17: CollisionMask = CollisionMask(1 << 16);
    pub const LAYER_18: CollisionMask = CollisionMask(1 << 17);
    pub const LAYER_19: CollisionMask = CollisionMask(1 << 18);
    pub const LAYER_20: CollisionMask = CollisionMask(1 << 19);
    pub const LAYER_21: CollisionMask = CollisionMask(1 << 20);
    pub const LAYER_22: CollisionMask = CollisionMask(1 << 21);
    pub const LAYER_23: CollisionMask = CollisionMask(1 << 22);
    pub const LAYER_24: CollisionMask = CollisionMask(1 << 23);
    pub const LAYER_25: CollisionMask = CollisionMask(1 << 24);
    pub const LAYER_26: CollisionMask = CollisionMask(1 << 25);
    pub const LAYER_27: CollisionMask = CollisionMask(1 << 26);
    pub const LAYER_28: CollisionMask = CollisionMask(1 << 27);
    pub const LAYER_29: CollisionMask = CollisionMask(1 << 28);
    pub const LAYER_30: CollisionMask = CollisionMask(1 << 29);
    pub const LAYER_31: CollisionMask = CollisionMask(1 << 30);
    pub const LAYER_32: CollisionMask = CollisionMask(1 << 31);
    pub const LAYER_33: CollisionMask = CollisionMask(1 << 32);
    pub const LAYER_34: CollisionMask = CollisionMask(1 << 33);
    pub const LAYER_35: CollisionMask = CollisionMask(1 << 34);
    pub const LAYER_36: CollisionMask = CollisionMask(1 << 35);
    pub const LAYER_37: CollisionMask = CollisionMask(1 << 36);
    pub const LAYER_38: CollisionMask = CollisionMask(1 << 37);
    pub const LAYER_39: CollisionMask = CollisionMask(1 << 38);
    pub const LAYER_40: CollisionMask = CollisionMask(1 << 39);
    pub const LAYER_41: CollisionMask = CollisionMask(1 << 40);
    pub const LAYER_42: CollisionMask = CollisionMask(1 << 41);
    pub const LAYER_43: CollisionMask = CollisionMask(1 << 42);
    pub const LAYER_44: CollisionMask = CollisionMask(1 << 43);
    pub const LAYER_45: CollisionMask = CollisionMask(1 << 44);
    pub const LAYER_46: CollisionMask = CollisionMask(1 << 45);
    pub const LAYER_47: CollisionMask = CollisionMask(1 << 46);
    pub const LAYER_48: CollisionMask = CollisionMask(1 << 47);
    pub const LAYER_49: CollisionMask = CollisionMask(1 << 48);
    pub const LAYER_50: CollisionMask = CollisionMask(1 << 49);
    pub const LAYER_51: CollisionMask = CollisionMask(1 << 50);
    pub const LAYER_52: CollisionMask = CollisionMask(1 << 51);
    pub const LAYER_53: CollisionMask = CollisionMask(1 << 52);
    pub const LAYER_54: CollisionMask = CollisionMask(1 << 53);
    pub const LAYER_55: CollisionMask = CollisionMask(1 << 54);
    pub const NOT_COLLIDING_WITH_ITSELF: CollisionMask = CollisionMask(1 << 55);
    pub const CONSIDER_TILE_TRANSITIONS: CollisionMask = CollisionMask(1 << 56);
    pub const COLLIDING_WITH_TILES_ONLY: CollisionMask = CollisionMask(1 << 57);
    pub const ALL: CollisionMask = CollisionMask((1 << 58) - 1);
    pub const NONE: CollisionMask = CollisionMask(0);
}

impl<T: AsRef<str>> FromIterator<T> for CollisionMask {
    fn from_iter<I: IntoIterator<Item = T>>(layers: I) -> Self {
        let mut result = Self(0);
        for layer in layers {
            if let Ok(layer_parsed) = layer.as_ref().parse() {
                result |= layer_parsed;
            }
        }
        result
    }
}

impl CollisionMask {
    pub fn without_flags(&self) -> Self {
        Self(self.0 & (Self::NOT_COLLIDING_WITH_ITSELF.0 - 1))
    }
}

#[derive(Debug, Clone, Error)]
enum CollisionMaskParseError {
    #[error("Invalid layer name: \"{0}\"")]
    InvalidLayerName(String),
}

impl FromStr for CollisionMask {
    type Err = CollisionMaskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ground-tile" => Self::GROUND_TILE,
            "water-tile" => Self::WATER_TILE,
            "resource-layer" => Self::RESOURCE_LAYER,
            "doodad-layer" => Self::DOODAD_LAYER,
            "floor-layer" => Self::FLOOR_LAYER,
            "item-layer" => Self::ITEM_LAYER,
            "ghost-layer" => Self::GHOST_LAYER,
            "object-layer" => Self::OBJECT_LAYER,
            "player-layer" => Self::PLAYER_LAYER,
            "train-layer" => Self::TRAIN_LAYER,
            "rail-layer" => Self::RAIL_LAYER,
            "transport-belt-layer" => Self::TRANSPORT_BELT_LAYER,
            // These 3 are flags
            "not-colliding-with-itself" => Self::NOT_COLLIDING_WITH_ITSELF,
            "consider-tile-transitions" => Self::CONSIDER_TILE_TRANSITIONS,
            "colliding-with-tiles-only" => Self::COLLIDING_WITH_TILES_ONLY,
            // I love vim
            // https://vim.fandom.com/wiki/Increasing_or_decreasing_numbers
            // https://vim.fandom.com/wiki/Macros
            "layer-13" => Self::LAYER_13,
            "layer-14" => Self::LAYER_14,
            "layer-15" => Self::LAYER_15,
            "layer-16" => Self::LAYER_16,
            "layer-17" => Self::LAYER_17,
            "layer-18" => Self::LAYER_18,
            "layer-19" => Self::LAYER_19,
            "layer-20" => Self::LAYER_20,
            "layer-21" => Self::LAYER_21,
            "layer-22" => Self::LAYER_22,
            "layer-23" => Self::LAYER_23,
            "layer-24" => Self::LAYER_24,
            "layer-25" => Self::LAYER_25,
            "layer-26" => Self::LAYER_26,
            "layer-27" => Self::LAYER_27,
            "layer-28" => Self::LAYER_28,
            "layer-29" => Self::LAYER_29,
            "layer-30" => Self::LAYER_30,
            "layer-31" => Self::LAYER_31,
            "layer-32" => Self::LAYER_32,
            "layer-33" => Self::LAYER_33,
            "layer-34" => Self::LAYER_34,
            "layer-35" => Self::LAYER_35,
            "layer-36" => Self::LAYER_36,
            "layer-37" => Self::LAYER_37,
            "layer-38" => Self::LAYER_38,
            "layer-39" => Self::LAYER_39,
            "layer-40" => Self::LAYER_40,
            "layer-41" => Self::LAYER_41,
            "layer-42" => Self::LAYER_42,
            "layer-43" => Self::LAYER_43,
            "layer-44" => Self::LAYER_44,
            "layer-45" => Self::LAYER_45,
            "layer-46" => Self::LAYER_46,
            "layer-47" => Self::LAYER_47,
            "layer-48" => Self::LAYER_48,
            "layer-49" => Self::LAYER_49,
            "layer-50" => Self::LAYER_50,
            "layer-51" => Self::LAYER_51,
            "layer-52" => Self::LAYER_52,
            "layer-53" => Self::LAYER_53,
            "layer-54" => Self::LAYER_54,
            "layer-55" => Self::LAYER_55,
            _ => return Err(CollisionMaskParseError::InvalidLayerName(s.to_string())),
        })
    }
}

impl<'lua> ToLua<'lua> for CollisionMask {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<Value<'lua>> {
        let mut result: HashMap<String, bool> = HashMap::new();
        if (self & Self::GROUND_TILE).0 > 0 {
            result.insert("ground-tile".into(), true);
        };
        if (self & Self::WATER_TILE).0 > 0 {
            result.insert("water-tile".into(), true);
        };
        if (self & Self::RESOURCE_LAYER).0 > 0 {
            result.insert("resource-layer".into(), true);
        };
        if (self & Self::DOODAD_LAYER).0 > 0 {
            result.insert("doodad-layer".into(), true);
        };
        if (self & Self::FLOOR_LAYER).0 > 0 {
            result.insert("floor-layer".into(), true);
        };
        if (self & Self::ITEM_LAYER).0 > 0 {
            result.insert("item-layer".into(), true);
        };
        if (self & Self::GHOST_LAYER).0 > 0 {
            result.insert("ghost-layer".into(), true);
        };
        if (self & Self::OBJECT_LAYER).0 > 0 {
            result.insert("object-layer".into(), true);
        };
        if (self & Self::PLAYER_LAYER).0 > 0 {
            result.insert("player-layer".into(), true);
        };
        if (self & Self::TRAIN_LAYER).0 > 0 {
            result.insert("train-layer".into(), true);
        };
        if (self & Self::RAIL_LAYER).0 > 0 {
            result.insert("rail-layer".into(), true);
        };
        if (self & Self::TRANSPORT_BELT_LAYER).0 > 0 {
            result.insert("transport-belt-layer".into(), true);
        };
        if (self & Self::NOT_COLLIDING_WITH_ITSELF).0 > 0 {
            result.insert("not-colliding-with-itself".into(), true);
        };
        if (self & Self::CONSIDER_TILE_TRANSITIONS).0 > 0 {
            result.insert("consider-tile-transitions".into(), true);
        };
        if (self & Self::COLLIDING_WITH_TILES_ONLY).0 > 0 {
            result.insert("colliding-with-tiles-only".into(), true);
        };
        if (self & Self::LAYER_13).0 > 0 {
            result.insert("layer-13".into(), true);
        };
        if (self & Self::LAYER_14).0 > 0 {
            result.insert("layer-14".into(), true);
        };
        if (self & Self::LAYER_15).0 > 0 {
            result.insert("layer-15".into(), true);
        };
        if (self & Self::LAYER_16).0 > 0 {
            result.insert("layer-16".into(), true);
        };
        if (self & Self::LAYER_17).0 > 0 {
            result.insert("layer-17".into(), true);
        };
        if (self & Self::LAYER_18).0 > 0 {
            result.insert("layer-18".into(), true);
        };
        if (self & Self::LAYER_19).0 > 0 {
            result.insert("layer-19".into(), true);
        };
        if (self & Self::LAYER_20).0 > 0 {
            result.insert("layer-20".into(), true);
        };
        if (self & Self::LAYER_21).0 > 0 {
            result.insert("layer-21".into(), true);
        };
        if (self & Self::LAYER_22).0 > 0 {
            result.insert("layer-22".into(), true);
        };
        if (self & Self::LAYER_23).0 > 0 {
            result.insert("layer-23".into(), true);
        };
        if (self & Self::LAYER_24).0 > 0 {
            result.insert("layer-24".into(), true);
        };
        if (self & Self::LAYER_25).0 > 0 {
            result.insert("layer-25".into(), true);
        };
        if (self & Self::LAYER_26).0 > 0 {
            result.insert("layer-26".into(), true);
        };
        if (self & Self::LAYER_27).0 > 0 {
            result.insert("layer-27".into(), true);
        };
        if (self & Self::LAYER_28).0 > 0 {
            result.insert("layer-28".into(), true);
        };
        if (self & Self::LAYER_29).0 > 0 {
            result.insert("layer-29".into(), true);
        };
        if (self & Self::LAYER_30).0 > 0 {
            result.insert("layer-30".into(), true);
        };
        if (self & Self::LAYER_31).0 > 0 {
            result.insert("layer-31".into(), true);
        };
        if (self & Self::LAYER_32).0 > 0 {
            result.insert("layer-32".into(), true);
        };
        if (self & Self::LAYER_33).0 > 0 {
            result.insert("layer-33".into(), true);
        };
        if (self & Self::LAYER_34).0 > 0 {
            result.insert("layer-34".into(), true);
        };
        if (self & Self::LAYER_35).0 > 0 {
            result.insert("layer-35".into(), true);
        };
        if (self & Self::LAYER_36).0 > 0 {
            result.insert("layer-36".into(), true);
        };
        if (self & Self::LAYER_37).0 > 0 {
            result.insert("layer-37".into(), true);
        };
        if (self & Self::LAYER_38).0 > 0 {
            result.insert("layer-38".into(), true);
        };
        if (self & Self::LAYER_39).0 > 0 {
            result.insert("layer-39".into(), true);
        };
        if (self & Self::LAYER_40).0 > 0 {
            result.insert("layer-40".into(), true);
        };
        if (self & Self::LAYER_41).0 > 0 {
            result.insert("layer-41".into(), true);
        };
        if (self & Self::LAYER_42).0 > 0 {
            result.insert("layer-42".into(), true);
        };
        if (self & Self::LAYER_43).0 > 0 {
            result.insert("layer-43".into(), true);
        };
        if (self & Self::LAYER_44).0 > 0 {
            result.insert("layer-44".into(), true);
        };
        if (self & Self::LAYER_45).0 > 0 {
            result.insert("layer-45".into(), true);
        };
        if (self & Self::LAYER_46).0 > 0 {
            result.insert("layer-46".into(), true);
        };
        if (self & Self::LAYER_47).0 > 0 {
            result.insert("layer-47".into(), true);
        };
        if (self & Self::LAYER_48).0 > 0 {
            result.insert("layer-48".into(), true);
        };
        if (self & Self::LAYER_49).0 > 0 {
            result.insert("layer-49".into(), true);
        };
        if (self & Self::LAYER_50).0 > 0 {
            result.insert("layer-50".into(), true);
        };
        if (self & Self::LAYER_51).0 > 0 {
            result.insert("layer-51".into(), true);
        };
        if (self & Self::LAYER_52).0 > 0 {
            result.insert("layer-52".into(), true);
        };
        if (self & Self::LAYER_53).0 > 0 {
            result.insert("layer-53".into(), true);
        };
        if (self & Self::LAYER_54).0 > 0 {
            result.insert("layer-54".into(), true);
        };
        if (self & Self::LAYER_55).0 > 0 {
            result.insert("layer-55".into(), true);
        };
        result.to_lua(lua)
    }
}

impl BitAnd for CollisionMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for CollisionMask {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = CollisionMask(self.0 & rhs.0)
    }
}

impl BitOr for CollisionMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for CollisionMask {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = CollisionMask(self.0 | rhs.0)
    }
}

impl BitXor for CollisionMask {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for CollisionMask {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = CollisionMask(self.0 ^ rhs.0)
    }
}

struct CollisionMaskVisitor;

impl<'de> Visitor<'de> for CollisionMaskVisitor {
    type Value = CollisionMask;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A sequence of strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut result = CollisionMask::default();
        while let Some(layer) = seq.next_element::<&str>()? {
            if let Ok(layer_parsed) = layer.parse() {
                result |= layer_parsed;
            }
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for CollisionMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(CollisionMaskVisitor)
    }
}

/// <https://wiki.factorio.com/Types/EntityPrototypeFlags>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, Default)]
pub struct EntityPrototypeFlags(pub(crate) u32);

impl EntityPrototypeFlags {
    pub const NOT_ROTATABLE: Self = Self(1);
    pub const PLACEABLE_PLAYER: Self = Self(1 << 1);
    pub const PLACEABLE_NEUTRAL: Self = Self(1 << 2);
    pub const PLACEABLE_ENEMY: Self = Self(1 << 3);
    pub const PLACEABLE_OFF_GRID: Self = Self(1 << 4);
    pub const PLAYER_CREATION: Self = Self(1 << 5);
    pub const BUILDING_DIRECTION_8_WAY: Self = Self(1 << 6);
    pub const FILTER_DIRECTIONS: Self = Self(1 << 7);
    pub const FAST_REPLACEABLE_NO_BUILD_WHILE_MOVING: Self = Self(1 << 8);
    pub const BREATHS_AIR: Self = Self(1 << 9);
    pub const NOT_REPAIRABLE: Self = Self(1 << 10);
    pub const NOT_ON_MAP: Self = Self(1 << 11);
    pub const NOT_BLUEPRINTABLE: Self = Self(1 << 12);
    pub const NOT_DECONSTRUCTABLE: Self = Self(1 << 13);
    pub const HIDDEN: Self = Self(1 << 14);
    pub const HIDE_ALT_INFO: Self = Self(1 << 15);
    pub const FAST_REPLACEABLE_NO_CROSS_TYPE_WHILE_MOVING: Self = Self(1 << 16);
    pub const NO_GAR_FILL_WHILE_BUILDING: Self = Self(1 << 17);
    pub const NOT_FLAMMABLE: Self = Self(1 << 18);
    pub const NO_AUTOMATED_ITEM_REMOVAL: Self = Self(1 << 19);
    pub const NO_AUTOMATED_ITEM_INSERTION: Self = Self(1 << 20);
    pub const NO_COPY_PASTE: Self = Self(1 << 21);
    pub const NOT_SELECTABLE_IN_GAME: Self = Self(1 << 22);
    pub const NOT_UPGRADABLE: Self = Self(1 << 23);
    pub const NOT_IN_KILL_STATISTICS: Self = Self(1 << 24);
    pub const NOT_IT_MADE_IN: Self = Self(1 << 25);
    pub const ALL: Self = Self((1 << 26) - 1);
    pub const NONE: Self = Self(0);
}

impl<T: AsRef<str>> FromIterator<T> for EntityPrototypeFlags {
    fn from_iter<I: IntoIterator<Item = T>>(flags: I) -> Self {
        let mut result = Self(0);
        for flag in flags {
            if let Ok(flag_parsed) = flag.as_ref().parse() {
                result |= flag_parsed;
            }
        }
        result
    }
}

#[derive(Debug, Clone, Error)]
pub enum EntityPrototypeFlagsParseError {
    #[error("Invalid entity prototype flag name: \"{0}\"")]
    InvalidFlag(String),
}

impl FromStr for EntityPrototypeFlags {
    type Err = EntityPrototypeFlagsParseError;

    /// Parses just one flag, used in a loop in [`FromIterator`] implementation
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "not-rotatable" => Self::NOT_ROTATABLE,
            "placeable-player" => Self::PLACEABLE_PLAYER,
            "placeable-neutral" => Self::PLACEABLE_NEUTRAL,
            "placeable-enemy" => Self::PLACEABLE_ENEMY,
            "placeable-off-grid" => Self::PLACEABLE_OFF_GRID,
            "player-creation" => Self::PLAYER_CREATION,
            "building-direction-8-way" => Self::BUILDING_DIRECTION_8_WAY,
            "filter-directions" => Self::FILTER_DIRECTIONS,
            "fast-replaceable-no-build-while-moving" => {
                Self::FAST_REPLACEABLE_NO_BUILD_WHILE_MOVING
            }
            "breaths-air" => Self::BREATHS_AIR,
            "not-repairable" => Self::NOT_REPAIRABLE,
            "not-on-map" => Self::NOT_ON_MAP,
            "not-blueprintable" => Self::NOT_BLUEPRINTABLE,
            "not-deconstructable" => Self::NOT_DECONSTRUCTABLE,
            "hidden" => Self::HIDDEN,
            "hide-alt-info" => Self::HIDE_ALT_INFO,
            "fast-replaceable-no-cross-type-while-moving" => {
                Self::FAST_REPLACEABLE_NO_CROSS_TYPE_WHILE_MOVING
            }
            "no-gap-fill-while-building" => Self::NO_GAR_FILL_WHILE_BUILDING,
            "not-flammable" => Self::NOT_FLAMMABLE,
            "no-automated-item-removal" => Self::NO_AUTOMATED_ITEM_REMOVAL,
            "no-automated-item-insertion" => Self::NO_AUTOMATED_ITEM_INSERTION,
            "no-copy-paste" => Self::NO_COPY_PASTE,
            "not-selectable-in-game" => Self::NOT_SELECTABLE_IN_GAME,
            "not-upgradable" => Self::NOT_UPGRADABLE,
            "not-in-kill-statistics" => Self::NOT_IN_KILL_STATISTICS,
            "not-in-made-in" => Self::NOT_IT_MADE_IN,
            _ => return Err(EntityPrototypeFlagsParseError::InvalidFlag(s.to_string())),
        })
    }
}

impl BitAnd for EntityPrototypeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for EntityPrototypeFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = EntityPrototypeFlags(self.0 & rhs.0)
    }
}

impl BitOr for EntityPrototypeFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for EntityPrototypeFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = EntityPrototypeFlags(self.0 | rhs.0)
    }
}

impl BitXor for EntityPrototypeFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for EntityPrototypeFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = EntityPrototypeFlags(self.0 ^ rhs.0)
    }
}

struct EntityPrototypeFlagsVsitior;

impl<'de> Visitor<'de> for EntityPrototypeFlagsVsitior {
    type Value = EntityPrototypeFlags;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A sequence of strings")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut result = EntityPrototypeFlags::default();
        while let Some(flag) = seq.next_element::<&str>()? {
            if let Ok(flag_parsed) = flag.parse() {
                result |= flag_parsed;
            }
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for EntityPrototypeFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(EntityPrototypeFlagsVsitior)
    }
}

/// <https://wiki.factorio.com/Types/DamagePrototype>
#[derive(Debug, Clone, Deserialize)]
pub struct DamagePrototype {
    pub amount: f32,
    #[serde(rename = "type")]
    pub damage_type: String, // Name of Damage type
}

/// <https://wiki.factorio.com/Types/DamageTypeFilters>
#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct DamageTypeFilters {
    #[serde_with(as = "serde_with::OneOrMany::<_>")]
    types: Vec<String>, // If String, converted to Vec<String> with one element // Name of DamageType prototype
    #[serde(default)]
    whitelist: bool, // Default: false
}

/// <https://wiki.factorio.com/Types/ForceCondition>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ForceCondition {
    All,
    Enemy,
    Ally,
    Friend,
    NotFriend,
    Same,
    NotSame,
}

/// <https://wiki.factorio.com/Types/AreaTriggerItem#collision_mode>
#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumString, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum CollisionMode {
    DistanceFromCollisionBox,
    DistanceFromCenter,
}

/// <https://wiki.factorio.com/Types/MinableProperties>
#[derive(Debug, Clone, Deserialize)]
pub struct MinableProperties {
    pub mining_time: f64,
    pub results: ProductResults,
    #[serde(default)]
    pub fluid_amount: f64, // Default: 0
    pub mining_particle: Option<String>, // Name of Prototype/Particle
    pub required_fluid: Option<String>,  // Name of Prototype/Fluid
    // Converted to results item
    // if results are present, these are ignored
    //result: String,
    //count: u16, // Default: 1
    pub mining_trigger: Option<Trigger>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ProductResults {
    Result(ProductPrototype),
    Results { results: Vec<ProductPrototype> },
}

compile_error!("Continue from here");

/// <https://wiki.factorio.com/Types/ProductPrototype>
#[derive(Debug, Clone)]
pub enum ProductPrototype {
    /// type = "item" // Default
    Item(ItemProductPrototype),
    /// type = "fluid"
    Fluid(FluidProductPrototype),
}

impl<'lua> PrototypeFromLua<'lua> for ProductPrototype {
    fn prototype_from_lua(
        value: Value<'lua>,
        lua: &'lua Lua,
        data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        if let mlua::Value::Table(table) = &value {
            if table.get::<_, Option<f64>>("mining_time")?.is_some() {
                // this means that we are in MinableProperties definition
                let name = table.get::<_, String>("result")?;
                let amount = table.get::<_, Option<u16>>("count")?.unwrap_or(1);
                Ok(Self::Item(ItemProductPrototype::name_and_amount(
                    name, amount,
                )))
            } else if let Some(pp_type) = table.get::<_, Option<String>>("type")? {
                match pp_type.as_ref() {
                    "item" => Ok(Self::Item(ItemProductPrototype::prototype_from_lua(
                        value, lua, data_table,
                    )?)),
                    "fluid" => Ok(Self::Fluid(FluidProductPrototype::prototype_from_lua(
                        value, lua, data_table,
                    )?)),
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: value.type_name(),
                        to: "ProductPrototype",
                        message: Some("Invalid `type`".into()),
                    }),
                }
            } else {
                Ok(Self::Item(ItemProductPrototype::from_sequence(
                    value, lua, data_table,
                )?))
            }
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "ProductPrototype",
                message: Some("expected table".into()),
            })
        }
    }
}

/// Either a sequence or a table, first item stands for name and second for amount
/// <https://wiki.factorio.com/Types/ItemProductPrototype>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct ItemProductPrototype {
    pub name: String, // Name of Prototype/Item
    #[default(true)]
    pub show_details_in_recipe_tooltip: bool, // Default: true
    pub amount: Option<u16>, // Mandatory when defined in a sequence
    #[default(1_f64)]
    pub probability: f64, // Default: 1
    #[mandatory_if(amount.is_none())]
    pub amount_min: Option<u16>, // Mandatory if amount is not specified
    #[mandatory_if(amount.is_none())]
    pub amount_max: Option<u16>, // Mandatory if amount is not specified // Set to amount_min if amount_max < amount_min
    #[default(0_u16)]
    pub catalyst_amount: u16, // Default: 0
}

impl ItemProductPrototype {
    fn post_extr_fn(&mut self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.probability < 0.0 || self.probability > 1.0 {
            return Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "ItemProductPrototype",
                message: Some("`probability` must be in a range of [0; 1]".into()),
            });
        }
        if let Some(amount_min) = self.amount_min {
            if let Some(amount_max) = self.amount_max {
                if amount_max < amount_min {
                    self.amount_max = Some(amount_min)
                }
            }
        }
        Ok(())
    }
}

impl<'lua> ItemProductPrototype {
    fn from_sequence(
        value: Value<'lua>,
        _lua: &'lua Lua,
        _data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        if let Value::Table(t) = &value {
            let name = t.get::<_, String>(1)?;
            let amount = t.get::<_, u16>(2)?;
            Ok(Self::name_and_amount(name, amount))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "ItemProductPrototype",
                message: Some("expected table".into()),
            })
        }
    }

    fn name_and_amount(name: String, amount: u16) -> Self {
        Self {
            name,
            amount: Some(amount),
            show_details_in_recipe_tooltip: true,
            probability: 1.0,
            amount_min: None,
            amount_max: None,
            catalyst_amount: 0,
        }
    }
}

/// <https://wiki.factorio.com/Types/FluidProductPrototype>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct FluidProductPrototype {
    pub name: String, // Name of Prototype/Fluid
    #[default(true)]
    pub show_details_in_recipe_tooltip: bool, // Default: true
    #[default(1_f64)]
    pub probability: f64, // Default: 1
    pub amount: Option<f64>, // Cannot be < 0
    #[mandatory_if(amount.is_none())]
    pub amount_min: Option<f64>, // Mandatory if amount is not specified // Cannot be < 0
    #[mandatory_if(amount.is_none())]
    pub amount_max: Option<f64>, // Mandatory if amount is not specified // Set to amount_min if amount_max < amount_min
    pub temperature: Option<f64>,
    #[default(0_f64)]
    pub catalyst_amount: f64, // Default: 0
    #[default(0_u32)]
    pub fuildbox_index: u32, // Default: 0
}

impl FluidProductPrototype {
    fn post_extr_fn(&mut self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.probability < 0.0 || self.probability > 1.0 {
            return Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "FluidProductPrototype",
                message: Some("`probability` must be in a range of [0; 1]".into()),
            });
        }
        if let Some(amount) = self.amount {
            if amount.is_sign_negative() {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "table",
                    to: "FluidProductPrototype",
                    message: Some("`amount` can't be negative".into()),
                });
            }
        }
        if let Some(amount_min) = self.amount_min {
            if amount_min.is_sign_negative() {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "table",
                    to: "FluidProductPrototype",
                    message: Some("`amount_min` can't be negative".into()),
                });
            }
            if let Some(amount_max) = self.amount_max {
                if amount_max < amount_min {
                    self.amount_max = Some(amount_min)
                }
            }
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Prototype/Entity#remove_decoratives>
#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RemoveDecoratives {
    Automatic,
    True,
    False,
}

prot_from_str!(RemoveDecoratives);

/// <https://wiki.factorio.com/Prototype/Entity#placeable_by>
#[derive(Debug, Clone)]
pub struct ItemsToPlace(pub Vec<ItemToPlace>);

impl<'lua> PrototypeFromLua<'lua> for ItemsToPlace {
    fn prototype_from_lua(
        value: Value<'lua>,
        lua: &'lua Lua,
        data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        let type_name = value.type_name();
        if let Ok(v) = ItemToPlace::prototype_from_lua(value.clone(), lua, data_table) {
            Ok(Self(vec![v]))
        } else if let Ok(v) = <Vec<ItemToPlace>>::prototype_from_lua(value, lua, data_table) {
            Ok(Self(v))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: type_name,
                to: "ItemsToPlace",
                message: Some("expected ItemToPlace or array of ItemToPlace".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/ItemToPlace>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ItemToPlace {
    pub item: String, // Name of Item
    pub count: u32,   // Can't be larger than the stack size of the item
}

/// <https://wiki.factorio.com/Prototype/Cliff#orientations>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct OrientedCliffPrototypes {
    pub west_to_east: OrientedCliffPrototype,
    pub north_to_south: OrientedCliffPrototype,
    pub east_to_west: OrientedCliffPrototype,
    pub south_to_north: OrientedCliffPrototype,
    pub west_to_north: OrientedCliffPrototype,
    pub north_to_east: OrientedCliffPrototype,
    pub east_to_south: OrientedCliffPrototype,
    pub south_to_west: OrientedCliffPrototype,
    pub west_to_south: OrientedCliffPrototype,
    pub north_to_west: OrientedCliffPrototype,
    pub east_to_north: OrientedCliffPrototype,
    pub south_to_east: OrientedCliffPrototype,
    pub west_to_none: OrientedCliffPrototype,
    pub none_to_east: OrientedCliffPrototype,
    pub north_to_none: OrientedCliffPrototype,
    pub none_to_south: OrientedCliffPrototype,
    pub east_to_none: OrientedCliffPrototype,
    pub none_to_west: OrientedCliffPrototype,
    pub south_to_none: OrientedCliffPrototype,
    pub none_to_north: OrientedCliffPrototype,
}

/// <https://wiki.factorio.com/Types/OrientedCliffPrototype>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct OrientedCliffPrototype {
    pub collision_bounding_box: BoundingBox,
    pub pictures: Vec<SpriteVariation>,
    pub fill_volume: u32,
}

/// <https://wiki.factorio.com/Prototype/RailRemnants#bending_type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum BendingType {
    Straight,
    Turn,
}

prot_from_str!(BendingType);

/// <https://wiki.factorio.com/Types/ExplosionDefinition>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ExplosionDefinition {
    pub name: String, // Name of Prototype/Entity
    pub offset: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/Resistances>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct Resistance {
    #[rename("type")]
    pub resistance_type: String, // Name of Prototype/DamageType
    #[default(0_f32)]
    pub decrease: f32, // Default: 0
    #[default(0_f32)]
    pub percent: f32, // Default: 0
}

/// <https://wiki.factorio.com/Types/Loot>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct Loot {
    pub item: String, // Name of Prototype/Item
    #[default(1_f64)]
    pub probability: f64, // Default: 1
    #[default(1_f64)]
    pub count_min: f64, // Default: 1
    #[default(1_f64)]
    pub count_max: f64, // Default: 1 // Must be > 0
}

impl Loot {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.count_max <= 0.0 {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "Loot",
                message: Some("`count_max` must be > 0".into()),
            });
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/AttackReactionItem>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct AttackReactionItem {
    pub range: f32,
    pub action: Option<Trigger>,
    #[default(0_f32)]
    pub reaction_modifier: f32, // Default: 0
    pub damage_type: Option<String>, // name of Prototype/DamageType
}

/// <https://wiki.factorio.com/Types/EnergySource>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct EnergySourceBase {
    #[default(0_f64)]
    pub emissions_per_minute: f64, // Default: 0
    #[default(true)]
    pub render_no_power_icon: bool, // Default: true
    #[default(true)]
    pub render_no_network_icon: bool, // Default: true
}

/// <https://wiki.factorio.com/Types/EnergySource>
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString), strum(serialize_all = "kebab-case"))]
pub enum EnergySource {
    /// <https://wiki.factorio.com/Types/EnergySource#Electric_energy_source>
    Electric(ElectricEnergySource),
    /// <https://wiki.factorio.com/Types/EnergySource#Burner>
    Burner(BurnerEnergySource),
    /// <https://wiki.factorio.com/Types/EnergySource#Heat_energy_source>
    Heat(Box<HeatEnergySource>),
    /// <https://wiki.factorio.com/Types/EnergySource#Fluid_energy_source>
    Fluid(Box<FluidEnergySource>),
    /// <https://wiki.factorio.com/Types/EnergySource#Void_energy_source>
    Void,
}

impl<'lua> PrototypeFromLua<'lua> for EnergySource {
    fn prototype_from_lua(
        value: Value<'lua>,
        lua: &'lua Lua,
        data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        if let Value::Table(t) = &value {
            Ok(
                match t
                    .get::<_, String>("type")?
                    .parse::<EnergySourceDiscriminants>()
                    .map_err(LuaError::external)?
                {
                    EnergySourceDiscriminants::Electric => Self::Electric(
                        ElectricEnergySource::prototype_from_lua(value, lua, data_table)?,
                    ),
                    EnergySourceDiscriminants::Burner => Self::Burner(
                        BurnerEnergySource::prototype_from_lua(value, lua, data_table)?,
                    ),
                    EnergySourceDiscriminants::Heat => Self::Heat(Box::new(
                        HeatEnergySource::prototype_from_lua(value, lua, data_table)?,
                    )),
                    EnergySourceDiscriminants::Fluid => Self::Fluid(Box::new(
                        FluidEnergySource::prototype_from_lua(value, lua, data_table)?,
                    )),
                    EnergySourceDiscriminants::Void => Self::Void,
                },
            )
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "EnergySource",
                message: Some("Expected table".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/EnergySource#Electric_energy_source>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ElectricEnergySource {
    #[use_self_forced]
    pub base: EnergySourceBase,
    pub buffer_capacity: Option<Energy>,
    pub usage_priority: ElectricUsagePriority,
    #[default(Energy(f64::MAX))]
    pub input_flow_limit: Energy, // Default: f64::MAX
    #[default(Energy(f64::MAX))]
    pub output_flow_limit: Energy, // Default: f64::MAX
    pub drain: Option<Energy>,
}

/// <https://wiki.factorio.com/Types/EnergySource#Burner>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct BurnerEnergySource {
    #[use_self_forced]
    pub base: EnergySourceBase,
    pub fuel_inventory_size: ItemStackIndex,
    #[default(0_u16)]
    pub burnt_inventory_size: ItemStackIndex, // Default: 0
    pub smoke: Option<Vec<SmokeSource>>,
    pub light_flicker: Option<LightFlickeringDefinition>,
    #[default(1_f64)]
    pub effectivity: f64, // Default: 1
    #[default(vec!["chemical".to_string()])] // FIXME: ignores fuel_category
    pub fuel_categories: Vec<String>, // Default: "chemical"
}

/// <https://wiki.factorio.com/Types/EnergySource#Heat_energy_source>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct HeatEnergySource {
    #[use_self_forced]
    pub base: EnergySourceBase,
    pub max_temperature: f64, // Must be >= default_temperature
    #[default(15_f64)]
    pub default_temperature: f64, // Default: 15
    pub specific_heat: Energy,
    pub max_transfer: Energy,
    #[default(1_f64)]
    pub max_temperature_gradient: f64, // Default: 1
    #[default(15_f64)]
    pub min_working_temperature: f64, // Default: 15 // Must be >= default_temperature AND <= max_temperature
    #[default(1_f32)]
    pub minimum_glow_temperature: f32, // Default: 1
    pub pipe_covers: Option<Sprite4Way>,
    pub heat_pipe_covers: Option<Sprite4Way>,
    pub heat_picture: Option<Sprite4Way>,
    pub heat_glow: Option<Sprite4Way>,
    pub connections: Option<Vec<HeatConnection>>, // Up to 32 connections
}

impl HeatEnergySource {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.max_temperature < self.default_temperature {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "HeatEnergySource",
                message: Some("`max_temperature` must be >= `default_temperature`".into()),
            });
        }
        if self.min_working_temperature < self.default_temperature {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "HeatEnergySource",
                message: Some("`min_working_temperature` must be >= `default_temperature`".into()),
            });
        }
        if self.min_working_temperature > self.max_temperature {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "HeatEnergySource",
                message: Some("`min_working_temperature` must be <= `max_temperature`".into()),
            });
        }
        if let Some(connections) = &self.connections {
            if connections.len() > 32 {
                return Err(LuaError::FromLuaConversionError {
                    from: "table",
                    to: "HeatEnergySource",
                    message: Some("`connections` amount must be <= 32".into()),
                });
            }
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/EnergySource#Fluid_energy_source>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct FluidEnergySource {
    #[use_self_forced]
    pub base: EnergySourceBase,
    pub fluid_box: FluidBox,
    pub smoke: Option<Vec<SmokeSource>>,
    pub light_flicker: Option<LightFlickeringDefinition>,
    #[default(1_f64)]
    pub effectivity: f64, // Default: 1
    #[default(false)]
    pub burns_fluid: bool, // Default: false
    #[default(false)]
    pub scale_fluid_usage: bool, // Default: false
    #[default(0_f64)]
    pub fluid_usage_per_tick: f64, // Default: 0
    #[default(f64::INFINITY)]
    pub maximum_temperature: f64, // Default: f64::INFINITY
    #[default(true)]
    pub destroy_non_fuel_fluid: bool, // Default: true
}

/// <https://wiki.factorio.com/Types/ElectricUsagePriority>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ElectricUsagePriority {
    PrimaryInput,
    PrimaryOutput,
    SecondaryInput,
    SecondaryOutput,
    Tertiary,
    /// Can only be used by Prototype/SolarPanel
    Solar,
    /// Can only be used by Prototype/Lamp
    Lamp,
}

prot_from_str!(ElectricUsagePriority);

/// <https://wiki.factorio.com/Types/SmokeSource>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct SmokeSource {
    pub name: String,   // Name of Prototype/TrivialSmoke
    pub frequency: f64, // Can't be negative, NaN or infinite
    #[default(0_f64)]
    pub offset: f64, // Default: 0
    pub position: Option<Factorio2DVector>,
    pub north_position: Option<Factorio2DVector>,
    pub east_position: Option<Factorio2DVector>,
    pub south_position: Option<Factorio2DVector>,
    pub west_position: Option<Factorio2DVector>,
    pub deviation: Option<Position>,
    #[default(0_u16)]
    pub starting_frame_speed: u16, // Default: 0
    #[default(0_f64)]
    pub starting_frame_speed_deviation: f64, // Default: 0
    #[default(0_u16)]
    pub starting_frame: u16, // Default: 0
    #[default(0_f64)]
    pub starting_frame_deviation: f64, // Default: 0
    #[default(1_u8)]
    pub slow_down_factor: u8, // Default: 1
    #[default(0_f32)]
    pub height: f32, // Default: 0
    #[default(0_f32)]
    pub height_deviation: f32, // Default: 0
    #[default(0_f32)]
    pub starting_vertical_speed: f32, // Default: 0
    #[default(0_f32)]
    pub starting_vertical_speed_deviation: f32, // Default: 0
    #[default(0.965_f32)]
    pub vertical_speed_slowdown: f32, // Default: 0.965
}

impl SmokeSource {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.frequency.is_sign_negative()
            || self.frequency.is_nan()
            || self.frequency.is_infinite()
        {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "SmokeSource",
                message: Some("`frequency` can't be negative, NaN or infinite".into()),
            });
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/HeatConnection>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct HeatConnection {
    pub position: Position,
    pub direction: Direction,
}

/// <https://wiki.factorio.com/Types/FluidBox>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct FluidBox {
    pub pipe_connections: Vec<PipeConnectionDefinition>, // Max: 256
    #[default(1_f64)]
    pub base_area: f64,             // Default: 1 // Must be > 0
    #[default(0_f64)]
    pub base_level: f64,            // Default: 0
    #[default(1_f64)]
    pub height: f64,                // Default: 1 // Must be > 0
    pub filter: Option<String>,                          // Name of Prototype/Fluid
    #[default(RenderLayer::Object)]
    pub render_layer: RenderLayer, // Default: "object"
    pub pipe_covers: Option<Sprite4Way>,
    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,
    #[default(ProductionType::None)]
    pub production_type: ProductionType, // Default: None
    //secondary_draw_order: u8, // Default: 1 // Converted to secondary_draw_orders // FIXME
    pub secondary_draw_orders: SecondaryDrawOrders, // Default: (north = 1, east = 1, south = 1, west = 1)
}

impl FluidBox {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.pipe_connections.len() > 255 {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "FluidBox",
                message: Some("no more than 255 `pipe_connections` are allowed".into()),
            });
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/PipeConnectionDefinition>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct PipeConnectionDefinition {
    pub positions: Vec<Factorio2DVector>, // `position` takes priority and gets converted to this // FIXME
    #[default(0_u32)]
    pub max_underground_distance: u32, // Default: 0
    #[default(ProductionType::InputOutput)]
    #[rename("type")]
    pub production_type: ProductionType, // Default: "input-output"
}

/// <https://wiki.factorio.com/Types/Direction>
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    Northeast = 1,
    East = 2,
    Southeast = 3,
    South = 4,
    Southwest = 5,
    West = 6,
    Northwest = 7,
}

impl<'lua> FromLua<'lua> for Direction {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        Ok(match lua.unpack::<u8>(lua_value)? {
            0 => Self::North,
            1 => Self::Northeast,
            2 => Self::East,
            3 => Self::Southeast,
            4 => Self::South,
            5 => Self::Southwest,
            6 => Self::West,
            7 => Self::Northwest,
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "u8",
                    to: "Direction",
                    message: Some("Value must be in range [0; 7]".into()),
                })
            }
        })
    }
}

/// <https://wiki.factorio.com/Types/FluidBox#production_type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ProductionType {
    None,
    Input,
    InputOutput,
    Output,
}

prot_from_str!(ProductionType);

/// <https://wiki.factorio.com/Types/WireConnectionPoint>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct WireConnectionPoint {
    pub wire: WirePosition,
    pub shadow: WirePosition,
}

/// <https://wiki.factorio.com/Types/WirePosition>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct WirePosition {
    pub copper: Option<Factorio2DVector>,
    pub red: Option<Factorio2DVector>,
    pub green: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/SignalIDConnector>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SignalIDConnector {
    #[rename("type")]
    pub signal_type: SignalType,
    pub name: String, // Name of a circuit network signal
}

/// <https://wiki.factorio.com/Types/ModuleSpecification>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ModuleSpecification {
    pub module_slots: u16,                                  // Default: 0
    pub module_info_max_icons_per_row: u8, // Default: width of selection box / 0,75
    pub module_info_max_icon_rows: u8,     // Default: width of selection box / 1.5
    pub module_info_icon_shift: Factorio2DVector, // Default: (0, 0.7)
    pub module_info_icon_scale: f32,       // Default: 0.5
    pub module_info_separation_multiplier: f32, // Default: 1.1
    pub module_info_multi_row_initial_height_modifier: f32, // Default: -0.1
}

/// <https://wiki.factorio.com/Types/EffectTypeLimitation>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub struct EffectTypeLimitation(u8);

impl EffectTypeLimitation {
    pub const SPEED: Self = Self(1);
    pub const PRODUCTIVITY: Self = Self(1 << 1);
    pub const CONSUMPTION: Self = Self(1 << 2);
    pub const POLLUTION: Self = Self(1 << 3);
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self((1 << 4) - 1);
}

impl<T: AsRef<str>> FromIterator<T> for EffectTypeLimitation {
    fn from_iter<I: IntoIterator<Item = T>>(in_arr: I) -> Self {
        let mut result = Self(0);
        for item in in_arr {
            match item.as_ref() {
                "speed" => result |= Self::SPEED,
                "productivity" => result |= Self::PRODUCTIVITY,
                "consumption" => result |= Self::CONSUMPTION,
                "pollution" => result |= Self::POLLUTION,
                _ => {}
            }
        }
        result
    }
}

impl<'lua> FromLua<'lua> for EffectTypeLimitation {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let arr: Vec<String> = lua.unpack(lua_value)?;
        Ok(Self::from_iter(arr))
    }
}

impl BitAnd for EffectTypeLimitation {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for EffectTypeLimitation {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl BitOr for EffectTypeLimitation {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for EffectTypeLimitation {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl BitXor for EffectTypeLimitation {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for EffectTypeLimitation {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}

/// <https://wiki.factorio.com/Prototype/Boiler#mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum BoilerMode {
    HeatWaterInside,
    OutputToSeparatePipe,
}

prot_from_str!(BoilerMode);

/// <https://wiki.factorio.com/Types/FootprintParticle>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct FootprintParticle {
    pub tiles: Vec<String>,            // (Names) Name of a tile
    pub particle_name: Option<String>, // Name of a particle
    #[default(false)]
    pub use_as_default: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/LogisticContainer#logistic_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum LogisticMode {
    PassiveProvider,
    ActiveProvider,
    Storage,
    Buffer,
    Requester,
}

prot_from_str!(LogisticMode);

/// Used in many places, specified as string
/// <https://wiki.factorio.com/Prototype/ElectricEnergyInterface#gui_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum GuiMode {
    All,
    None,
    Admins,
}

prot_from_str!(GuiMode);

// Can also be converted from array
/// <https://wiki.factorio.com/Types/UnitSpawnDefinition>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct UnitSpawnDefinition {
    pub unit: String,                  // Name of Entity
    pub spawn_points: Vec<SpawnPoint>, // `evolution_factor` must be ascending from entry to entry
}

// Can also be converted from array
/// <https://wiki.factorio.com/Types/SpawnPoint>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SpawnPoint {
    pub evolution_factor: f64,
    pub spawn_height: f64, // Must be >= 0 // TODO
}

/// <https://wiki.factorio.com/Types/AmmoType>
#[derive(Debug, Clone, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct AmmoType {
    pub category: String, // Name of AmmoCategory
    pub action: Option<Trigger>,
    #[default(false)]
    pub clamp_position: bool, // Default: false // Forced to be false if `target_type` is "entity"
    pub energy_consumption: Option<Energy>,
    #[default(1_f64)]
    pub range_modifier: f64, // Default: 1
    #[default(1_f64)]
    pub cooldown_modifier: f64, // Default: 1
    #[default(1_f64)]
    pub consumption_modifier: f64, // Default: 1
    pub target_type: TargetType,
}

impl AmmoType {
    fn post_extr_fn(&mut self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.target_type == TargetType::Entity {
            self.clamp_position = false
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Types/AmmoType#target_type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TargetType {
    Entity,
    Position,
    Direction,
}

prot_from_str!(TargetType);

/// <https://wiki.factorio.com/Types/CircularProjectileCreationSpecification>
#[derive(Debug, Clone)]
pub struct CircularProjectileCreationSpecification(
    pub Vec<CircularProjectileCreationSpecificationItem>,
);

impl<'lua> PrototypeFromLua<'lua> for CircularProjectileCreationSpecification {
    fn prototype_from_lua(
        value: Value<'lua>,
        _lua: &'lua Lua,
        _data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        let type_name = value.type_name();
        if let Value::Table(t) = value {
            Ok(Self(t.sequence_values().collect::<LuaResult<_>>()?))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: type_name,
                to: "CircularProjectileCreationSpecification",
                message: Some("expected table".into()),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct CircularProjectileCreationSpecificationItem(pub RealOrientation, pub Factorio2DVector);

impl<'lua> FromLua<'lua> for CircularProjectileCreationSpecificationItem {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        if let Value::Table(t) = &value {
            let orientation: RealOrientation = t.get(1)?;
            let vector: Factorio2DVector = t.get(2)?;
            Ok(Self(orientation, vector))
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "CircularProjectileCreationSpecification item",
                message: Some("expected table".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/CircularParticleCreationSpecification>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct CircularParticleCreationSpecification {
    pub name: String, // Name of Entity
    pub starting_frame_speed: f32,
    #[default(0.5_f32)]
    pub direction: f32,
    #[default(0_f32)]
    pub direction_deviation: f32,
    #[default(0.1_f32)]
    pub speed: f32,
    #[default(0_f32)]
    pub speed_deviation: f32,
    #[default(0_f32)]
    pub starting_frame_speed_deviation: f32,
    #[default(1_f32)]
    pub height: f32,
    #[default(0_f32)]
    pub height_deviation: f32,
    #[default(0_f32)]
    pub vertical_speed: f32,
    #[default(0_f32)]
    pub vertical_speed_deviation: f32,
    #[default(Factorio2DVector(0.0, 0.0))]
    pub center: Factorio2DVector,
    #[default(0_f64)]
    pub creation_distance: f64,
    #[default(0_f64)]
    pub creation_distance_orientation: f64,
    pub use_source_position: Option<bool>,
}

/// <https://wiki.factorio.com/Types/HeatBuffer>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct HeatBuffer {
    pub max_temperature: f64, // Must be >= `default_temperature` // TODO
    pub specific_heat: Energy,
    pub max_transfer: Energy,
    #[default(15_f64)]
    pub default_temperature: f64, // Default: 15
    #[default(1_f64)]
    pub min_temperature_gradient: f64, // Default: 1
    #[default(15_f64)]
    pub min_working_temperature: f64, // Default: 15
    #[default(1_f32)]
    pub minimum_glow_temperature: f32, // Default: 1
    pub pipe_covers: Option<Sprite4Way>,
    pub heat_pipe_covers: Option<Sprite4Way>,
    pub heat_picture: Option<Sprite4Way>,
    pub heat_glow: Option<Sprite4Way>,
    pub connections: Option<Vec<HeatConnection>>, // 32 max // TODO
}

/// <https://wiki.factorio.com/Types/SignalColorMapping>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SignalColorMapping {
    #[rename("type")]
    r#type: SignalType,
    name: String, // Name of a signal
    color: Color,
}

/// <https://wiki.factorio.com/Types/SignalColorMapping#type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SignalType {
    Virtual,
    Item,
    Fluid,
}

prot_from_str!(SignalType);

/// <https://wiki.factorio.com/Prototype/ProgrammableSpeaker#instruments>
#[derive(Debug, Clone)]
pub struct Instrument {
    name: String,
    notes: Vec<Note>,
}

/// <https://wiki.factorio.com/Prototype/ProgrammableSpeaker#instruments>
#[derive(Debug, Clone)]
pub struct Note {
    name: String,
    sound: Sound,
}

/// <https://wiki.factorio.com/Types/AnimatedVector>
#[derive(Debug, Clone)]
pub struct AnimatedVector {
    rotations: Vec<AnimatedVectorRotation>,
    //render_layer: Option<RenderLayer>, // Just copied over to all rotations
    direction_shift: Option<AnimatedVectorDirectionShift>,
}

/// <https://wiki.factorio.com/Types/AnimatedVector#rotations>
#[derive(Debug, Clone)]
pub struct AnimatedVectorRotation {
    frames: Vec<Factorio2DVector>, // Sizes of all arrays must be the same
    render_layer: RenderLayer,
}

/// <https://wiki.factorio.com/Types/AnimatedVector#direction_shift>
#[derive(Debug, Clone)]
pub struct AnimatedVectorDirectionShift {
    north: Option<Factorio2DVector>,
    east: Option<Factorio2DVector>,
    south: Option<Factorio2DVector>,
    west: Option<Factorio2DVector>,
}

/// <https://wiki.factorio.com/Types/UnitAISettings>
#[derive(Debug, Clone)]
pub struct UnitAISettings {
    destroy_when_commands_fail: bool,  // Default: false
    allow_try_return_to_spawner: bool, // Default: false
    do_separation: bool,               // Default: true
    path_resolution_modifier: i8,      // Default: 0 // Must be between -8 and 8
}

/// <https://wiki.factorio.com/Prototype/Unit#alternative_attacking_frame_sequence>
#[derive(Debug, Clone)]
pub struct UnitAlternativeAttackingFrameSequence {
    warmup_frame_sequence: Vec<u16>,
    warmup2_frame_sequence: Vec<u16>,
    attacking_frame_sequence: Vec<u16>,
    cooldown_frame_sequence: Vec<u16>,
    prepared_frame_sequence: Vec<u16>,
    back_to_walk_frame_sequence: Vec<u16>,
    warmup_animation_speed: f32,
    attacking_animation_speed: f32,
    cooldown_animation_speed: f32,
    prepared_animation_speed: f32,
    back_to_walk_animation_speed: f32,
}

/// <https://wiki.factorio.com/Types/SpiderEnginePrototype>
#[derive(Debug, Clone)]
pub struct SpiderEnginePrototype {
    military_target: bool, // Converted from Option<String>. if Some(_), set to true. If None, set to false
    legs: Vec<SpiderLegSpecification>, // Single leg is converted to Vec with one leg
}

/// <https://wiki.factorio.com/Types/SpiderLegSpecification>
#[derive(Debug, Clone)]
pub struct SpiderLegSpecification {
    leg: String, // Name of SpiderLeg
    mount_position: Factorio2DVector,
    ground_position: Factorio2DVector,
    blocking_legs: Vec<u32>,
    leg_hit_the_ground_trigger: Option<TriggerEffect>,
}

/// <https://wiki.factorio.com/Prototype/FireFlame#burnt_patch_alpha_variations>
#[derive(Debug, Clone)]
pub struct FireFlameBurntPatchAlphaVariation {
    tile: String, // Name of a tile
    alpha: f32,
}

/// <https://wiki.factorio.com/Prototype/FlyingText>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

/// <https://wiki.factorio.com/Types/CursorBoxType>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum CursorBoxType {
    Entity,
    NotAllowed,
    Electricity,
    Pair,
    Copy,
    TrainVisualization,
    Logistics,
    BlueprintSnapRectangle,
}

/// <https://wiki.factorio.com/Types/EquipmentShape>
#[derive(Debug, Clone)]
pub struct EquipmentShape {
    width: u32,
    height: u32,
    shape_type: EquipmentShapeType,
    points: Option<EquipmentShapePoints>, // Mandatory if type is manual
}

/// <https://wiki.factorio.com/Types/EquipmentShape#type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum EquipmentShapeType {
    Full,
    Manual,
}

// Constructor should accept width and height, as points can't exceed them.
/// <https://wiki.factorio.com/Types/EquipmentShape#points>
#[derive(Debug, Clone)]
pub struct EquipmentShapePoints(pub Vec<Vec<u32>>);

/// <https://wiki.factorio.com/Prototype/NightVisionEquipment>
pub type DaytimeColorLookupTable = Vec<DaytimeColorLookupTableItem>;

#[derive(Debug, Clone)]
pub struct DaytimeColorLookupTableItem(pub f64, pub ColorLookupTable);

impl<'lua> PrototypeFromLua<'lua> for DaytimeColorLookupTableItem {
    fn prototype_from_lua(
        value: Value<'lua>,
        lua: &'lua Lua,
        data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        if let Value::Table(t) = &value {
            Ok(Self(t.get(1_isize)?, t.get_prot(2, lua, data_table)?))
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "DaytimeColorLookupTable item",
                message: Some("expected table".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/DaytimeColorLookupTable#Second_member>
#[derive(Debug, Clone)]
pub enum ColorLookupTable {
    Identity,
    Filename(FileName),
}

impl<'lua> PrototypeFromLua<'lua> for ColorLookupTable {
    fn prototype_from_lua(
        value: Value<'lua>,
        _lua: &'lua Lua,
        _data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        if let Value::String(s) = &value {
            let s = s.to_str()?;
            if s == "identity" {
                Ok(Self::Identity)
            } else {
                Ok(Self::Filename(s.into()))
            }
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "ColorLookupTable",
                message: Some("expected string".into()),
            })
        }
    }
}

/// <https://wiki.factorio.com/Types/PlaceAsTile>
#[derive(Debug, Clone)]
pub struct PlaceAsTile {
    result: String, // Name of Tile
    condition: CollisionMask,
    condition_size: i32,
}

/// <https://wiki.factorio.com/Types/ItemPrototypeFlags>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub struct ItemPrototypeFlags(u16);

impl ItemPrototypeFlags {
    pub const DRAW_LOGISTIC_OVERLAY: Self = Self(1);
    pub const HIDDEN: Self = Self(1 << 1);
    pub const ALWAYS_SHOW: Self = Self(1 << 2);
    pub const HIDE_FROM_BONUS_GUI: Self = Self(1 << 3);
    pub const HIDE_FROM_FUEL_TOOLTIP: Self = Self(1 << 4);
    pub const NOT_STACKABLE: Self = Self(1 << 5);
    pub const CAN_EXTEND_INVENTORY: Self = Self(1 << 6);
    pub const PRIMARY_PLACE_RESULT: Self = Self(1 << 7);
    pub const MOD_OPENABLE: Self = Self(1 << 8);
    pub const ONLY_IN_CURSOR: Self = Self(1 << 9);
    pub const SPAWNABLE: Self = Self(1 << 10);
}

impl<T: AsRef<str>> FromIterator<T> for ItemPrototypeFlags {
    fn from_iter<I: IntoIterator<Item = T>>(in_arr: I) -> Self {
        let mut result = Self(0);
        for item in in_arr {
            match item.as_ref() {
                "draw-logistic-overlay" => result |= Self::DRAW_LOGISTIC_OVERLAY,
                "hidden" => result |= Self::HIDDEN,
                "always-show" => result |= Self::ALWAYS_SHOW,
                "hide-from-bonus-gui" => result |= Self::HIDE_FROM_BONUS_GUI,
                "hide-from-fuel-tooltip" => result |= Self::HIDE_FROM_FUEL_TOOLTIP,
                "not-stackable" => result |= Self::NOT_STACKABLE,
                "can-extend-inventory" => result |= Self::CAN_EXTEND_INVENTORY,
                "primary-place-result" => result |= Self::PRIMARY_PLACE_RESULT,
                "mod-openable" => result |= Self::MOD_OPENABLE,
                "only-in-cursor" => result |= Self::ONLY_IN_CURSOR,
                "spawnable" => result |= Self::SPAWNABLE,
                _ => {}
            }
        }
        result
    }
}

impl BitAnd for ItemPrototypeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for ItemPrototypeFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl BitOr for ItemPrototypeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ItemPrototypeFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl BitXor for ItemPrototypeFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for ItemPrototypeFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}

/// <https://wiki.factorio.com/Prototype/AmmoItem#ammo_type>
#[derive(Debug, Clone)]
pub struct AmmoItemAmmoType {
    ammo_type: AmmoType,
    source_type: AmmoSourceType,
}

/// <https://wiki.factorio.com/Types/AmmoSourceType>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum AmmoSourceType {
    Default,
    Player,
    Turret,
    Vehicle,
}

/// <https://wiki.factorio.com/Prototype/ItemWithInventory#filter_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum FilterMode {
    Whitelist,
    Blacklist,
}

/// <https://wiki.factorio.com/Prototype/ItemWithInventory#insertion_priority_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum InsertionPriorityMode {
    Default,
    Never,
    Always,
    WhenManuallyFiltered,
}

/// <https://wiki.factorio.com/Prototype/SelectionTool#selection_mode>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub struct SelectionMode(u32);

impl SelectionMode {
    pub const BLUEPRINT: Self = Self(1);
    pub const DECONSTRUCT: Self = Self(1 << 1);
    pub const CANCEL_DECONSTRUCT: Self = Self(1 << 2);
    pub const ITEMS: Self = Self(1 << 3);
    pub const TREES: Self = Self(1 << 4);
    pub const BUILDABLE_TYPE: Self = Self(1 << 5);
    pub const NOTHING: Self = Self(1 << 6);
    pub const ITEMS_TO_PLACE: Self = Self(1 << 7);
    pub const ANY_ENTITY: Self = Self(1 << 8);
    pub const ANY_TILE: Self = Self(1 << 9);
    pub const SAME_FORCE: Self = Self(1 << 10);
    pub const NOT_SAME_FORCE: Self = Self(1 << 11);
    pub const FRIEND: Self = Self(1 << 12);
    pub const ENEMY: Self = Self(1 << 13);
    pub const UPGRADE: Self = Self(1 << 14);
    pub const CANCEL_UPGRADE: Self = Self(1 << 15);
    pub const DOWNGRADE: Self = Self(1 << 16);
    pub const ENTITY_WITH_HEALTH: Self = Self(1 << 17);
    pub const ENTITY_WITH_FORCE: Self = Self(1 << 18);
    pub const ENTITY_WITH_OWNER: Self = Self(1 << 19);
    pub const AVOID_ROLLING_STOCK: Self = Self(1 << 20);
}

impl<T: AsRef<str>> FromIterator<T> for SelectionMode {
    fn from_iter<I: IntoIterator<Item = T>>(in_arr: I) -> Self {
        let mut result = Self(0);
        for item in in_arr {
            match item.as_ref() {
                "blueprint" => result |= Self::BLUEPRINT,
                "deconstruct" => result |= Self::DECONSTRUCT,
                "cancel-deconstruct" => result |= Self::CANCEL_DECONSTRUCT,
                "items" => result |= Self::ITEMS,
                "trees" => result |= Self::TREES,
                "buildable-type" => result |= Self::BUILDABLE_TYPE,
                "nothing" => result |= Self::NOTHING,
                "items-to-place" => result |= Self::ITEMS_TO_PLACE,
                "any-entity" => result |= Self::ANY_ENTITY,
                "any-tile" => result |= Self::ANY_TILE,
                "same-force" => result |= Self::SAME_FORCE,
                "not-same-force" => result |= Self::NOT_SAME_FORCE,
                "friend" => result |= Self::FRIEND,
                "enemy" => result |= Self::ENEMY,
                "upgrade" => result |= Self::UPGRADE,
                "cancel-upgrade" => result |= Self::CANCEL_UPGRADE,
                "downgrade" => result |= Self::DOWNGRADE,
                "entity-with-health" => result |= Self::ENTITY_WITH_HEALTH,
                "entity-with-force" => result |= Self::ENTITY_WITH_FORCE,
                "entity-with-owner" => result |= Self::ENTITY_WITH_OWNER,
                "avoid-rolling-stock" => result |= Self::AVOID_ROLLING_STOCK,
                _ => {}
            }
        }
        result
    }
}

impl BitAnd for SelectionMode {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for SelectionMode {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl BitOr for SelectionMode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for SelectionMode {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl BitXor for SelectionMode {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for SelectionMode {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}

/// <https://wiki.factorio.com/Types/Effect>
#[derive(Debug, Clone)]
pub struct Effect {
    consumption: EffectSpec,
    speed: EffectSpec,
    productivity: EffectSpec,
    pollution: EffectSpec,
}

/// <https://wiki.factorio.com/Types/Effect#consumption>
#[derive(Debug, Clone)]
pub struct EffectSpec(f64); // `bonus` key // Pecision is ignored beyond two decimals: 17.567 -> 17.56

/// <https://wiki.factorio.com/Types/Resistances>
#[derive(Debug, Clone)]
pub struct Resistances(Vec<Resistance>);

/// <https://wiki.factorio.com/Prototype/Recipe#Recipe_data>
#[derive(Debug, Clone)]
pub struct RecipeData {
    ingredients: Vec<IngredientPrototype>, // Max amount is 65535 // can be empty // Duplicates are not allowed
    results: Vec<ProductPrototype>, // `result` and `result_count` are converted to this // Duplicate entries not allowed // Takes priority over `result`
    // result_count: u32 // Default: 1
    energy_required: f64,            // Default: 0.5
    emissions_multiplier: f64,       // Default: 1
    requester_paste_multiplier: u32, // Default: 30
    overload_multiplier: u32,        // Default: 0
    allow_inserter_overload: bool,   // Default: true
    enabled: bool,                   // Default: true
    hidden: bool,                    // default: false
    hide_from_stats: bool,           // Default: false
    hide_from_player_crafting: bool, // Default: false
    allow_decomposition: bool,       // Default: true
    allow_as_intermediate: bool,     // Default: true
    allow_intermediates: bool,       // Default: true
    always_show_made_in: bool,       // Default: false
    show_amount_in_title: bool,      // Default: true
    always_show_products: bool,      // Default: false
    unlock_results: bool,            // Default: true
    main_product: Option<String>,    // Name (not prototype reference) of recipe from `results`
}

/// <https://wiki.factorio.com/Types/IngredientPrototype>
#[derive(Debug, Clone)]
pub enum IngredientPrototype {
    // Determined by type
    Item(ItemIngredientPrototype),
    Fluid(FluidIngredientPrototype),
}

/// <https://wiki.factorio.com/Types/ItemIngredientPrototype>
#[derive(Debug, Clone)]
pub struct ItemIngredientPrototype {
    name: String,         // Name of Item // Also index 1
    amount: u16,          // Also index 2
    catalyst_amount: u16, // Default: 0
}

/// <https://wiki.factorio.com/Types/FluidIngredientPrototype>
#[derive(Debug, Clone)]
pub struct FluidIngredientPrototype {
    name: String, // Name of Fluid
    amount: f64,
    temperature: Option<f64>,
    minimum_temperature: Option<f64>,
    maximum_temperature: Option<f64>,
    catalyst_amount: f64, // Default: 0
    fluidbox_index: u32,  // Default: 0
}

/// <https://wiki.factorio.com/Prototype/Shortcut#action>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ShortcutAction {
    ToggleAltMode,
    Undo,
    Copy,
    Cut,
    Paste,
    ImportString,
    TogglePersonalRoboport,
    RoggleEquipmentMovementBonus,
    SpawnItem,
    Lua,
}

/// <https://wiki.factorio.com/Prototype/Shortcut#style>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ShortcutStyle {
    Default,
    Blue,
    Red,
    Green,
}

/// <https://wiki.factorio.com/Prototype/Technology#Technology_data>
#[derive(Debug, Clone)]
pub struct TechnologyData {
    upgrade: bool,                     // Default: false
    enabled: bool,                     // Default: true
    hidden: bool,                      // Default: false
    visible_when_disabled: bool,       // Default: false
    ignore_tech_cost_multiplier: bool, // Default: false
    unit: TechnologyUnit,
    max_level: TechnologyMaxLevel,
    prerequisites: Vec<String>, // (Names) Name of Technology
    effects: Vec<ModifierPrototype>,
}

/// <https://wiki.factorio.com/Prototype/Technology#unit>
#[derive(Debug, Clone)]
pub struct TechnologyUnit {
    // One of these 2 or both can be defined
    count: Option<u64>, // Must be > 0
    count_formula: Option<String>,
    time: f64,
    ingredients: Vec<IngredientPrototype>, // All Items must be tools
}

/// <https://wiki.factorio.com/Prototype/Technology#max_level>
#[derive(Debug, Clone)]
pub enum TechnologyMaxLevel {
    Level(u32),
    Infinite, // "infinite"
}

/// <https://wiki.factorio.com/Types/ModifierPrototype>
#[derive(Debug, Clone)]
pub struct ModifierPrototype {
    mp_type: ModifierPrototypeType,
    body: ModifierPrototypeBody,
    icon: IconSpecification,
    infer_icon: Option<bool>,
    use_icon_overlay_constant: Option<bool>, // This is stupid
}

/// <https://wiki.factorio.com/Types/ModifierPrototype>
#[derive(Debug, Clone)]
pub enum ModifierPrototypeBody {
    /// Variant for the types:
    /// "inserter-stack-size-bonus"
    /// "stack-inserter-capacity-bonus"
    /// "laboratory-speed"
    /// "character-logistic-trash-slots"
    /// "maximum-following-robots-count"
    /// "worker-robot-speed"
    /// "worker-robot-storage"
    /// "ghost-time-to-live"
    /// "character-crafting-speed"
    /// "character-mining-speed"
    /// "character-running-speed"
    /// "character-build-distance"
    /// "character-item-drop-distance"
    /// "character-reach-distance"
    /// "character-resource-reach-distance"
    /// "character-item-pickup-distance"
    /// "character-loot-pickup-distance"
    /// "character-inventory-slots-bonus"
    /// "deconstruction-time-to-live"
    /// "max-failed-attempts-per-tick-per-construction-queue"
    /// "max-successful-attempts-per-tick-per-construction-queue"
    /// "character-health-bonus"
    /// "mining-drill-productivity-bonus"
    /// "train-braking-force-bonus"
    /// "worker-robot-battery"
    /// "laboratory-productivity"
    /// "follower-robot-lifetime"
    /// "artillery-range"
    Simple(SimpleModifierPrototype),
    /// Variant for the types:
    /// "turret-attack"
    TurretAttack(TurretAttackModifierPrototype),
    /// Variant for the types:
    /// "ammo-damage"
    AmmoDamage(AmmoDamageModifierPrototype),
    /// Variant for the types:
    /// "give-item"
    GiveItem(GiveItemModifierPrototype),
    /// Variant for the types:
    /// "gun-speed"
    GunSpeed(GunSpeedModifierPrototype),
    /// Variant for the types:
    /// "unlock-recipe"
    UnlockRecipe(UnlockRecipeModifierPrototype),
    /// Variant for the types:
    /// "zoom-to-world-enabled"
    /// "zoom-to-world-ghost-building-enabled"
    /// "zoom-to-world-blueprint-enabled"
    /// "zoom-to-world-deconstruction-planner-enabled"
    /// "zoom-to-world-upgrade-planner-enabled"
    /// "zoom-to-world-selection-tool-enabled"
    /// "character-logistic-requests"
    Bool(BoolModifierPrototype),
    /// Variant for the types:
    /// "nothing"
    Mothing(NothingModifierPrototype),
}

/// <https://wiki.factorio.com/Types/SimpleModifierPrototype>
#[derive(Debug, Clone)]
pub struct SimpleModifierPrototype {
    modifier: f64,
}

/// <https://wiki.factorio.com/Types/TurretAttackModifierPrototype>
#[derive(Debug, Clone)]
pub struct TurretAttackModifierPrototype {
    turret_id: String, // Name of Entity
    modifier: f64,
}

/// <https://wiki.factorio.com/Types/AmmoDamageModifierPrototype>
#[derive(Debug, Clone)]
pub struct AmmoDamageModifierPrototype {
    ammo_category: String, // Name of AmmoCategory
    modifier: f64,
}

/// <https://wiki.factorio.com/Types/GiveItemModifierPrototype>
#[derive(Debug, Clone)]
pub struct GiveItemModifierPrototype {
    item: String,         // Name of Item
    count: ItemCountType, // Default: 1 // Must be > 0
}

/// <https://wiki.factorio.com/Types/GunSpeedModifierPrototype>
#[derive(Debug, Clone)]
pub struct GunSpeedModifierPrototype {
    ammo_category: String, // Name of AmmoCategory
    modifier: f64,
}

/// <https://wiki.factorio.com/Types/UnlockRecipeModifierPrototype>
#[derive(Debug, Clone)]
pub struct UnlockRecipeModifierPrototype {
    recipe: String, // Name of the recipe
}

/// <https://wiki.factorio.com/Types/BoolModifierPrototype>
#[derive(Debug, Clone)]
pub struct BoolModifierPrototype {
    modifier: bool,
}

/// <https://wiki.factorio.com/Types/NothingModifierPrototype>
#[derive(Debug, Clone)]
pub struct NothingModifierPrototype {
    effect_description: LocalisedString,
}

/// <https://wiki.factorio.com/Types/ModifierPrototype#type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ModifierPrototypeType {
    InserterStackSizeBonus,
    StackInserterCapacityBonus,
    LaboratorySpeed,
    CharacterLogisticTrashSlots,
    MaximumFollowingRobotsCount,
    WorkerRobotSpeed,
    WorkerRobotStorage,
    GhostTimeToLive,
    TurretAttack,
    AmmoDamage,
    GiveItem,
    GunSpeed,
    UnlockRecipe,
    CharacterCraftingSpeed,
    CharacterMiningSpeed,
    CharacterRunningSpeed,
    CharacterBuildDistance,
    CharacterItemDropDistance,
    CharacterReachDistance,
    CharacterResourceReachDistance,
    CharacterItemPickupDistance,
    CharacterLootPickupDistance,
    CharacterInventorySlotsBonus,
    DeconstructionTimeToLive,
    MaxFailedAttemptsPerTickPerConstructionQueue,
    MaxSuccessfulAttemptsPerTickPerConstructionQueue,
    CharacterHealthBonus,
    MiningDrillProductivityBonus,
    TrainBrakingForceBonus,
    ZoomToWorldEnabled,
    ZoomToWorldGhostBuildingEnabled,
    ZoomToWorldBlueprintEnabled,
    ZoomToWorldDeconstructionPlannerEnabled,
    ZoomToWorldUpgradePlannerEnabled,
    ZoomToWorldSelectionToolEnabled,
    WorkerRobotBattery,
    LaboratoryProductivity,
    FollowerRobotLifetime,
    ArtilleryRange,
    Nothing,
    CharacterLogisticRequests,
}

/// <https://wiki.factorio.com/Types/SimulationDefinition>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct SimulationDefinition {
    pub save: Option<FileName<SaveFileType>>,
    pub init_file: Option<FileName<ScriptFileType>>,
    #[default("")]
    pub init: String, // Default: "" // Only loaded if `init_file` is not present
    pub update_file: Option<FileName<ScriptFileType>>,
    #[default("")]
    pub update: String, // Default: "" // Only loaded if `update_file` is not present
    #[default(0_u32)]
    pub init_update_count: u32, // Default: 0
    #[default(0_u32)]
    pub length: u32, // Default: 0
    #[default(false)]
    pub generate_map: bool, // Default: false
    #[default(true)]
    pub checkboard: bool, // Default: true
    pub volume_modifier: Option<f32>,
    #[default(false)]
    pub override_volume: bool, // Default: false // default not confirmed
}

/// <https://wiki.factorio.com/Types/TipStatus>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TipStatus {
    Locked,
    Optional,
    DependenciesNotMet,
    Unlocked,
    Suggested,
    NotToBeSuggested,
    CompletedWithoutTutorial,
    Completed,
}

/// <https://wiki.factorio.com/Types/BoxSpecification>
#[derive(Debug, Clone)]
pub struct BoxSpecification {
    pub sprite: Sprite,
    pub dimension_spec: BoxSpecificationDimensionSpec,
}

impl<'lua> PrototypeFromLua<'lua> for BoxSpecification {
    fn prototype_from_lua(
        value: Value<'lua>,
        lua: &'lua Lua,
        data_table: &mut DataTable,
    ) -> LuaResult<Self> {
        if let Value::Table(t) = &value {
            let sprite = t.get_prot::<_, Sprite>("sprite", lua, data_table)?;
            let is_whole_box: bool = t.get("is_whole_box")?;
            let dimension_spec = if is_whole_box {
                let side_length: f64 = t.get("side_length")?;
                let side_height: f64 = t.get("side_height")?;
                BoxSpecificationDimensionSpec::WholeBox(side_length, side_height)
            } else {
                let max_side_length: f64 = t.get("max_side_length")?;
                BoxSpecificationDimensionSpec::NotWholeBox(max_side_length)
            };
            Ok(Self {
                sprite,
                dimension_spec,
            })
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "BoxSpecification",
                message: Some("expected table".into()),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoxSpecificationDimensionSpec {
    WholeBox(f64, f64), // `is_whole_box` = true; `side_length` and `side_height`
    NotWholeBox(f64),   // `is_whole_box` = false (Default); `max_side_length`
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TrackType {
    EarlyGame,
    MainTrack,
    Interlude,
    LateGame,
    MenuTrack,
}

prot_from_str!(TrackType);
