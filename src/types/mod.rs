mod attack_parameters;
mod autoplace;
mod graphics;
mod sound;
mod style_specification;
mod trigger;

pub use attack_parameters::*;
pub use autoplace::*;
pub use graphics::*;
pub use sound::*;
pub use style_specification::*;
pub use trigger::*;

use std::collections::HashMap;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use std::str::FromStr;
use std::fmt;
use crate::prototypes::PrototypesErr;

/// May be made into struct in the future <https://wiki.factorio.com/Types/FileName>
pub type FileName = String;
/// <https://wiki.factorio.com/Types/ItemStackIndex>
pub type ItemStackIndex = u16;
/// <https://wiki.factorio.com/Types/ItemCountType>
pub type ItemCountType = u32;
// Type derived from Factorio3DVector definition (https://wiki.factorio.com/Types/Vector3D)
/// 2D Vector defined by Factorio <https://wiki.factorio.com/Types/vector>
pub type Factorio2DVector = (f32, f32);
/// 3D Vector defined by Factorio <https://wiki.factorio.com/Types/Vector3D>
pub type Factorio3DVector = (f32, f32, f32);
// Parser and checker maybe?
/// Keyboard keys sequence <https://wiki.factorio.com/Prototype/CustomInput#key_sequence>
pub type KeySequence = String;
// Consider adding Option<f32> as specified in https://wiki.factorio.com/Types/BoundingBox? It's kinda undocumented
/// <https://wiki.factorio.com/Types/BoundingBox>
pub type BoundingBox = (Position, Position);
/// Value range: [0.0; 1.0) <https://wiki.factorio.com/Types/RealOrientation>
pub type RealOrientation = f32;

/// Can be constructed from an array or table with x and y values <https://wiki.factorio.com/Types/Position>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position(i32, i32);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

/// Any of the color components are optional <https://wiki.factorio.com/Types/Color>
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color(f32, f32, f32, f32);

impl Color {
    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(r, g, b, a)
    }

    pub fn new_rgba_opt(r: Option<f32>, g: Option<f32>, b: Option<f32>, a: Option<f32>) -> Self {
        let r = r.or(Some(0.0 as f32)).unwrap();
        let g = g.or(Some(0.0 as f32)).unwrap();
        let b = b.or(Some(0.0 as f32)).unwrap();
        let a = a.or(Some(0.0 as f32)).unwrap();
        Self(r, g, b, a)
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self { // r, g, b default is 0
        Self(r, g, b, 1.0)
    }
}

/// <https://lua-api.factorio.com/latest/defines.html#defines.difficulty_settings>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DifficultySetting {
    Normal,
    Expensive
}

impl FromStr for DifficultySetting {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "expensive" => Ok(Self::Expensive),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("DifficultySetting"), String::from(s)))
        }
    }
}

impl fmt::Display for DifficultySetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Normal => "normal",
            Self::Expensive => "expensive",
        })
    }
}

/// <https://wiki.factorio.com/Prototype/MapSettings#difficulty_settings>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ResearchQueueSetting {
    AfterVictory,
    Always,
    Never
}

impl FromStr for ResearchQueueSetting {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "after-victory" => Ok(Self::AfterVictory),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("ResearchQueueSetting"), String::from(s)))
        }
    }
}

impl fmt::Display for ResearchQueueSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::AfterVictory => "after-victory",
            Self::Always => "always",
            Self::Never => "never",
        })
    }
}

/// <https://wiki.factorio.com/Tutorial:Mod_settings#The_setting_type_property>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ModSettingType {
    Startup,
    RuntimeGlobal,
    RuntimePerUser,
}

impl fmt::Display for ModSettingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ModSettingType::Startup => "startup",
            ModSettingType::RuntimeGlobal => "runtime-global",
            ModSettingType::RuntimePerUser => "runtime-per-user",
        })
    }
}

impl FromStr for ModSettingType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "startup" => Ok(ModSettingType::Startup),
            "runtime-global" => Ok(ModSettingType::RuntimeGlobal),
            "runtime-per-user" => Ok(ModSettingType::RuntimePerUser),
            _ => Err(PrototypesErr::InvalidModSettingType(s.to_string()))
        }
    }
}

/// <https://wiki.factorio.com/Types/MapGenPreset>
#[derive(Debug)]
pub enum MapGenPreset {
    // Decided by `default` field
    Default(MapGenPresetDefault),
    NonDefault(MapGenPresetNonDefault)
}

/// <https://wiki.factorio.com/Types/MapGenPreset#default>
#[derive(Debug)]
pub struct MapGenPresetDefault {
    order: String
}

/// <https://wiki.factorio.com/Types/MapGenPreset#default>
#[derive(Debug)]
pub struct MapGenPresetNonDefault {
    order: String,
    // Should these be optional or just have defaults? TODO
    basic_settings: Option<MapGenPresetBasicSettings>,
    advanced_settings: Option<MapGenPresetAdvancedSettings>
}

/// <https://wiki.factorio.com/Types/MapGenSize>
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct MapGenSize(f64); // Exact type is unknown, so slap an f64

impl FromStr for MapGenSize {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self(0.0)),
            "very-low" | "very-small" | "very-poor" => Ok(Self(0.5)),
            "low" | "small" | "poor" => Ok(Self(1.0 / (2.0 as f64).sqrt())),
            "normal" | "medium" | "regular" => Ok(Self(1.0)),
            "high" | "big" | "good" => Ok(Self((2.0 as f64).sqrt())),
            "very-high" | "very-big" | "very-good" => Ok(Self(2.0)),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("MapGenSize"), String::from(s)))
        }
    }
}

impl MapGenSize {
    pub fn new(size: f64) -> Self {
        Self(size)
    }
}

/// <https://wiki.factorio.com/Types/MapGenPreset#basic_settings>
#[derive(Debug)]
pub struct MapGenAutoplaceControl {
    frequency: Option<MapGenSize>,
    size: Option<MapGenSize>,
    rechness: Option<MapGenSize>,
}

/// <https://lua-api.factorio.com/latest/Concepts.html#CliffPlacementSettings>
#[derive(Debug)]
pub struct CliffPlacementSettings {
    name: String, // Name of the cliff prototype
    cliff_elevation_0: f32, // Default 10.0
    cliff_elevation_interval: f32,
    richness: MapGenSize
}

/// <https://wiki.factorio.com/Types/MapGenPreset#basic_settings>
#[derive(Debug)]
pub struct MapGenPresetBasicSettings {
    // Defaults are not documented for some f'ing reason
    terain_segmentation: MapGenSize, // Default is... Unknown
    water: MapGenSize, // Same here
    default_enable_all_autoplace_controls: bool, // Default: true
    autoplace_controls: HashMap<String, MapGenAutoplaceControl>, // key is AutoplaceControl name
    // autoplace_settings // TODO: UNDOCUMENTED // "Types/table", refuses to elaborate further
    property_expression_names: HashMap<String, String>, // Map property name to noise expression name
    starting_points: Position,
    seed: u32,
    width: u32,
    height: u32,
    starting_area: MapGenSize,
    peaceful_mode: bool,
    cliff_settings: CliffPlacementSettings
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug)]
pub struct MapGenPresetAdvancedSettings {
    // Defaults are not documented too
    pollution: MapGenPollution,
    enemy_evolution: MapGenEnemyEvolution,
    enemy_expansion: MapGenEnemyExpansion,
    difficulty_settings: MapGenDifficultySettings
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug)]
pub struct MapGenPollution {
    enabled: bool,
    diffusion_ratio: f64, // Must be <= 0.25
    ageing: f64, // Must be >= 0.5
    enemy_attack_pollution_consumption_modifier: f64,
    min_pollution_to_damage_trees: f64,
    pollution_restored_per_tree_damage: f64
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug)]
pub struct MapGenEnemyEvolution {
    enabled: bool,
    time_factor: f64,
    destroy_factor: f64,
    pollution_factor: f64
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug)]
pub struct MapGenEnemyExpansion {
    enabled: bool,
    // Oddly satisfying how lines strings line up
    max_expansion_distance: f64,
    settler_group_min_size: f64,
    settler_group_max_size: f64,
    max_expansion_cooldown: f64,
    min_expansion_cooldown: f64
}

/// <https://wiki.factorio.com/Types/MapGenPreset#advanced_settings>
#[derive(Debug)]
pub struct MapGenDifficultySettings {
    recipe_difficulty: DifficultySetting,
    technology_difficulty: DifficultySetting,
    technology_price_multiplier: f64,
    research_queue_setting: ResearchQueueSetting
}

/// <https://wiki.factorio.com/Prototype/MapSettings#pollution>
#[derive(Debug)]
pub struct MapPollutionSettings {
    enabled: bool,
    diffusion_ratio: f64,
    min_to_diffuse: f64,
    ageing: f64,
    expected_max_per_chunk: f64,
    min_to_show_per_chunk: f64,
    min_pollution_to_damage_trees: f64,
    pollution_with_max_forest_damage: f64,
    pollution_restored_per_tree_damage: f64,
    pollution_per_tree_damage: f64,
    max_pollution_to_restore_trees: f64,
    enemy_attack_pollution_consumption_modifier: f64
}

/// <https://wiki.factorio.com/Prototype/MapSettings#steering>
#[derive(Debug)]
pub struct MapSteering {
    default: MapSteeringSettings,
    moving: MapSteeringSettings
}

/// <https://wiki.factorio.com/Prototype/MapSettings#steering>
#[derive(Debug)]
pub struct MapSteeringSettings {
    radius: f64,
    separation_factor: f64,
    separation_force: f64,
    force_unit_fuzzy_goto_behavior: bool
}

/// <https://wiki.factorio.com/Prototype/MapSettings#enemy_evolution>
#[derive(Debug)]
pub struct MapEnemyEvolution {
    enabled: bool,
    time_factor: f64,
    destroy_factor: f64,
    pollution_factor: f64
}

/// <https://wiki.factorio.com/Prototype/MapSettings#unit_group>
#[derive(Debug)]
pub struct MapUnitGroup {
    min_group_gathering_time: u32,
    max_group_gathering_time: u32,
    max_wait_time_for_late_members: u32,
    max_group_radius: f64,
    min_group_radius: f64,
    max_member_speedup_when_behind: f64,
    max_member_slowdown_when_ahead: f64,
    max_group_slowdown_facor: f64,
    max_group_member_fallback_factor: f64,
    member_disown_distance: f64,
    tick_tolerance_when_member_arrives: u32,
    max_gathering_unit_groups: u32,
    max_unit_group_size: u32
}

/// <https://wiki.factorio.com/Prototype/MapSettings#enemy_expansion>
#[derive(Debug)]
pub struct MapEnemyExpansion {
    enabled: bool,
    max_expansion_distance: u32,
    friendly_base_influence_radius: u32,
    enemy_building_influence_radius: u32,
    building_coefficient: f64,
    other_base_coefficient: f64,
    neighbouring_chunk_coefficient: f64,
    neighbouring_base_chunk_coefficient: f64,
    max_colliding_tiles_coefficient: f64,
    settler_group_min_size: u32,
    settler_group_max_size: u32,
    min_expansion_cooldown: u32,
    max_expansion_cooldown: u32
}

/// <https://wiki.factorio.com/Prototype/MapSettings#path_finder>
#[derive(Debug)]
pub struct MapPathFinder {
    fwd2bwd_ratio: i32,
    goal_pressure_ratio: f64,
    use_path_cache: bool,
    max_steps_worked_per_tick: f64,
    max_work_done_per_tick: u32,
    short_cache_size: u32,
    short_cache_min_cacheable_distance: f64,
    short_cache_min_algo_steps_to_cache: u32,
    long_cache_min_cacheable_distance: f64,
    cache_max_connect_to_cache_steps_multiplier: u32,
    cache_accept_path_start_distance_ratio: f64,
    cache_accept_path_end_distance_ratio: f64,
    negative_cache_accept_path_start_distance_ratio: f64,
    negative_cache_accept_path_end_distance_ratio: f64,
    cache_path_start_distance_rating_multiplier: f64,
    cache_path_end_distance_rating_multiplier: f64,
    stale_enemy_with_same_destination_collision_penalty: f64,
    ignore_moving_enemy_collision_distance: f64,
    enemy_with_different_destination_collision_penalty: f64,
    general_entity_collision_penalty: f64,
    general_entity_subsequent_collision_penalty: f64,
    extended_collision_penalty: f64,
    max_clients_to_accept_any_new_request: u32,
    max_clients_to_accept_short_new_request: u32,
    direct_distance_to_consider_short_request: u32,
    short_request_max_steps: u32,
    short_request_ratio: f64,
    min_steps_to_check_path_find_termination: u32,
    start_to_goal_cost_multiplier_to_terminate_path_find: f64,
    overload_levels: Vec<u32>,
    overload_multipliers: Vec<f64>
}

/// <https://wiki.factorio.com/Prototype/MapSettings#difficulty_settings>
#[derive(Debug)]
pub struct MapDifficultySettings {
    recipe_difficulty: DifficultySetting,
    technology_difficulty: DifficultySetting,
    technology_price_multiplier: f64, // Default: 1.0 // Must be >= 0.001 and <= 1000.0
    research_queue_setting: Option<ResearchQueueSetting>
}

/// <https://wiki.factorio.com/Prototype/MouseCursor>
#[derive(Debug)]
pub enum MouseCursorType {
    SystemCursor(SystemCursor),
    CustomCursor(CustomCursor)
}

/// <https://wiki.factorio.com/Prototype/MouseCursor#system_cursor>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SystemCursor {
    Arrow,
    IBeam,
    Crosshair,
    WaitArrow,
    SizeAll,
    No,
    Hand
}

impl FromStr for SystemCursor {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arrow" => Ok(Self::Arrow),
            "i-beam" => Ok(Self::IBeam),
            "crosshair" => Ok(Self::Crosshair),
            "wait-arrow" => Ok(Self::WaitArrow),
            "size-all" => Ok(Self::SizeAll),
            "no" => Ok(Self::No),
            "hand" => Ok(Self::Hand),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("SystemCursor"), String::from(s)))
        }
    }
}

impl fmt::Display for SystemCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Arrow => "arrow",
            Self::IBeam => "i-beam",
            Self::Crosshair => "crosshair",
            Self::WaitArrow => "wait-arrow",
            Self::SizeAll => "size-all",
            Self::No => "no",
            Self::Hand => "hand",
        })
    }
}

/// <https://wiki.factorio.com/Prototype/MouseCursor>
#[derive(Debug)]
pub struct CustomCursor {
    filename: FileName,
    hot_pixel_x: i16,
    hot_pixel_y: i16
}

/// <https://wiki.factorio.com/Types/IconSpecification>
#[derive(Debug)]
pub enum IconSpecification {
    Icon(IconSpec),
    Icons(IconsSpec)
}

/// <https://wiki.factorio.com/Types/IconSpecification#Prototype_properties:_Option_2>
#[derive(Debug)]
pub struct IconSpec {
    icon: FileName,
    icon_size: i16,
    icon_mipmaps: u8 // Default: 0
}

/// <https://wiki.factorio.com/Types/IconSpecification#Prototype_properties:_Option_1>
#[derive(Debug)]
pub struct IconsSpec {
    icons: Vec<IconData>,
    // icon_size omitted here, it will be copied to each IconData
    icon_mipmaps: u8 // Default: 0
}

/// <https://wiki.factorio.com/Types/IconData>
#[derive(Debug)]
pub struct IconData {
    icon: FileName,
    icon_size: i16, // Copied from `icon_size` from prototype
    tint: Color, // Default: (0, 0, 0 , 1)
    shift: Factorio2DVector, // Default: (0, 0)
    scale: f64, // Default: 1
    icon_mipmaps: u8 // Default: 0
}

// TODO: fmt::Display
/// Input data is converted to J/tick or Joule
/// J/s (Joule/second) is not supported, as I can't find any uses and it's equvalent to W (Watt)
/// <https://wiki.factorio.com/Types/Energy>
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Energy(f64); // I don't know which type factorio uses internally, so I will use this

impl FromStr for Energy {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_num(num_string: &str, original: &str) -> Result<f64, PrototypesErr> {
            num_string.parse().map_err(|_| PrototypesErr::InvalidTypeStr("Energy".into(), original.into()))
        }

        fn get_multiplier(multiplier_char: &char, original: &str) -> Result<f64, PrototypesErr> {
            match multiplier_char {
                    'k' | 'K' => Ok(1000.0),
                    'M' => Ok(1000000.0),
                    'G' => Ok(1000000000.0),
                    'T' => Ok((10.0 as f64).powi(12)),
                    'P' => Ok((10.0 as f64).powi(15)),
                    'E' => Ok((10.0 as f64).powi(18)),
                    'Z' => Ok((10.0 as f64).powi(21)),
                    'Y' => Ok((10.0 as f64).powi(24)),
                    _ => Err(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(original)))
            }
        }

        let len = s.len();
        let mut rev_s = s.chars().rev();
        let last_char: char = rev_s.next().ok_or(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))?;
        if last_char == 'W' {
            let next_char: char = rev_s.next().ok_or(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))?;
            if next_char.is_ascii_digit() {
                return Ok(Self(parse_num(&s[0..len-1], s)?/60.0))
            } else {
                let value = parse_num(&s[0..len-2], s)?;
                return Ok(Self(value * get_multiplier(&next_char, &s)?/60.0))
            }
        } else if last_char == 'J' {
            let next_char: char = rev_s.next().ok_or(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))?;
            if next_char.is_ascii_digit() {
                return Ok(Self(parse_num(&s[0..len-1], s)?))
            } else {
                let value = parse_num(&s[0..len-2], s)?;
                return Ok(Self(value * get_multiplier(&next_char, &s)?))
            }
        } else {
            return Err(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))
        } 
    }
}

/// <https://wiki.factorio.com/Types/ProductPrototype>
#[derive(Debug)]
pub enum ProductType {
    Item(String),
    Fluid(String)
}

/// <https://wiki.factorio.com/Prototype/ResearchAchievement>
#[derive(Debug)]
pub enum ResearchTarget {
    All,
    Technology(String)
}

/// <https://wiki.factorio.com/Prototype/AutoplaceControl#category>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AutoplaceControlCategory {
    Resource,
    Terrain,
    Enemy
}

impl FromStr for AutoplaceControlCategory {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "resource" => Ok(Self::Resource),
            "terrain" => Ok(Self::Terrain),
            "enemy" => Ok(Self::Enemy),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("AutoplaceControlCategory"), String::from(s)))
        }
    }
}

impl fmt::Display for AutoplaceControlCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Resource => "resource",
            Self::Terrain => "terrain",
            Self::Enemy => "enemy",
        })
    }
}

/// <https://wiki.factorio.com/Prototype/CustomInput#consuming>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ConsumingType {
    None,
    GameOnly
}

impl FromStr for ConsumingType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "game-only" => Ok(Self::GameOnly),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("ConsumingType"), String::from(s)))
        }
    }
}

impl fmt::Display for ConsumingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::None => "none",
            Self::GameOnly => "game-only",
        })
    }
}

/// <https://wiki.factorio.com/Prototype/CustomInput#action>
#[derive(Debug)]
pub enum CustomInputAction {
    Lua,
    SpawnItem,
    TogglePersonalRoboport,
    TogglePersonalLogisticRequests,
    ToggleEquipmentMovementBonus
}

impl FromStr for CustomInputAction {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lua" => Ok(Self::Lua),
            "spawn-item" => Ok(Self::SpawnItem),
            "toggle-personal-roboport" => Ok(Self::TogglePersonalRoboport),
            "toggle-personal-logistic-requests" => Ok(Self::TogglePersonalLogisticRequests),
            "toggle-equipment-movement-bonus" => Ok(Self::ToggleEquipmentMovementBonus),
            _ => Err(PrototypesErr::InvalidTypeStr("CustomInputAction".into(), s.into()))
        }
    }
}

impl fmt::Display for CustomInputAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Lua => "lua",
            Self::SpawnItem => "spawn-item",
            Self::TogglePersonalRoboport => "toggle-personal-roboport",
            Self::TogglePersonalLogisticRequests => "toggle-personal-logistic-requests",
            Self::ToggleEquipmentMovementBonus => "toggle-equipment-movement-bonus",
        })
    }
}

/// <https://wiki.factorio.com/Types/CollisionMask>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct CollisionMask(u64);

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
    pub const ALL: CollisionMask = CollisionMask(u64::MAX); // Just sets all bits 1, instead of setting all usable bits
}

impl From<Vec<&str>> for CollisionMask {
    fn from(layers: Vec<&str>) -> Self {
        let mut result = Self(0);
        for layer in layers {
            match layer {
                "ground-tile" => result |= Self::GROUND_TILE,
                "water-tile" => result |= Self::WATER_TILE,
                "resource-layer" => result |= Self::RESOURCE_LAYER,
                "doodad-layer" => result |= Self::DOODAD_LAYER,
                "floor-layer" => result |= Self::FLOOR_LAYER,
                "item-layer" => result |= Self::ITEM_LAYER,
                "ghost-layer" => result |= Self::GHOST_LAYER,
                "object-layer" => result |= Self::OBJECT_LAYER,
                "player-layer" => result |= Self::PLAYER_LAYER,
                "train-layer" => result |= Self::TRAIN_LAYER,
                "rail-layer" => result |= Self::RAIL_LAYER,
                "transport-belt-layer" => result |= Self::TRANSPORT_BELT_LAYER,
                "not-colliding-with-itself" => result |= Self::NOT_COLLIDING_WITH_ITSELF,
                "consider-tile-transitions" => result |= Self::CONSIDER_TILE_TRANSITIONS,
                "colliding-with-tiles-only" => result |= Self::COLLIDING_WITH_TILES_ONLY,
                // I love vim
                // https://vim.fandom.com/wiki/Increasing_or_decreasing_numbers
                // https://vim.fandom.com/wiki/Macros
                "layer-13" => result |= Self::LAYER_13,
                "layer-14" => result |= Self::LAYER_14,
                "layer-15" => result |= Self::LAYER_15,
                "layer-16" => result |= Self::LAYER_16,
                "layer-17" => result |= Self::LAYER_17,
                "layer-18" => result |= Self::LAYER_18,
                "layer-19" => result |= Self::LAYER_19,
                "layer-20" => result |= Self::LAYER_20,
                "layer-21" => result |= Self::LAYER_21,
                "layer-22" => result |= Self::LAYER_22,
                "layer-23" => result |= Self::LAYER_23,
                "layer-24" => result |= Self::LAYER_24,
                "layer-25" => result |= Self::LAYER_25,
                "layer-26" => result |= Self::LAYER_26,
                "layer-27" => result |= Self::LAYER_27,
                "layer-28" => result |= Self::LAYER_28,
                "layer-29" => result |= Self::LAYER_29,
                "layer-30" => result |= Self::LAYER_30,
                "layer-31" => result |= Self::LAYER_31,
                "layer-32" => result |= Self::LAYER_32,
                "layer-33" => result |= Self::LAYER_33,
                "layer-34" => result |= Self::LAYER_34,
                "layer-35" => result |= Self::LAYER_35,
                "layer-36" => result |= Self::LAYER_36,
                "layer-37" => result |= Self::LAYER_37,
                "layer-38" => result |= Self::LAYER_38,
                "layer-39" => result |= Self::LAYER_39,
                "layer-40" => result |= Self::LAYER_40,
                "layer-41" => result |= Self::LAYER_41,
                "layer-42" => result |= Self::LAYER_42,
                "layer-43" => result |= Self::LAYER_43,
                "layer-44" => result |= Self::LAYER_44,
                "layer-45" => result |= Self::LAYER_45,
                "layer-46" => result |= Self::LAYER_46,
                "layer-47" => result |= Self::LAYER_47,
                "layer-48" => result |= Self::LAYER_48,
                "layer-49" => result |= Self::LAYER_49,
                "layer-50" => result |= Self::LAYER_50,
                "layer-51" => result |= Self::LAYER_51,
                "layer-52" => result |= Self::LAYER_52,
                "layer-53" => result |= Self::LAYER_53,
                "layer-54" => result |= Self::LAYER_54,
                "layer-55" => result |= Self::LAYER_55,
                _ => {}
            }
        }
        result
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

/// <https://wiki.factorio.com/Types/EntityPrototypeFlags>
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct EntityPrototypeFlags(u32);

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
    pub const ALL: Self = Self(u32::MAX);
}

impl From<Vec<&str>> for EntityPrototypeFlags {
    fn from(flags: Vec<&str>) -> Self {
        let mut result = Self(0);
        for flag in flags {
            match flag {
                "not-rotatable" => result |= Self::NOT_ROTATABLE,
                "placeable-player" => result |= Self::PLACEABLE_PLAYER,
                "placeable-neutral" => result |= Self::PLACEABLE_NEUTRAL,
                "placeable-enemy" => result |= Self::PLACEABLE_ENEMY,
                "placeable-off-grid" => result |= Self::PLACEABLE_OFF_GRID,
                "player-creation" => result |= Self::PLAYER_CREATION,
                "building-direction-8-way" => result |= Self::BUILDING_DIRECTION_8_WAY,
                "filter-directions" => result |= Self::FILTER_DIRECTIONS,
                "fast-replaceable-no-build-while-moving" => result |= Self::FAST_REPLACEABLE_NO_BUILD_WHILE_MOVING,
                "breaths-air" => result |= Self::BREATHS_AIR,
                "not-repairable" => result |= Self::NOT_REPAIRABLE,
                "not-on-map" => result |= Self::NOT_ON_MAP,
                "not-blueprintable" => result |= Self::NOT_BLUEPRINTABLE,
                "not-deconstructable" => result |= Self::NOT_DECONSTRUCTABLE,
                "hidden" => result |= Self::HIDDEN,
                "hide-alt-info" => result |= Self::HIDE_ALT_INFO,
                "fast-replaceable-no-cross-type-while-moving" => result |= Self::FAST_REPLACEABLE_NO_CROSS_TYPE_WHILE_MOVING,
                "no-gap-fill-while-building" => result |= Self::NO_GAR_FILL_WHILE_BUILDING,
                "not-flammable" => result |= Self::NOT_FLAMMABLE,
                "no-automated-item-removal" => result |= Self::NO_AUTOMATED_ITEM_REMOVAL,
                "no-automated-item-insertion" => result |= Self::NO_AUTOMATED_ITEM_INSERTION,
                "no-copy-paste" => result |= Self::NO_COPY_PASTE,
                "not-selectable-in-game" => result |= Self::NOT_SELECTABLE_IN_GAME,
                "not-upgradable" => result |= Self::NOT_UPGRADABLE,
                "not-in-kill-statistics" => result |= Self::NOT_IN_KILL_STATISTICS,
                _ => {}
            }
        }
        result
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

/// <https://wiki.factorio.com/Types/DamagePrototype>
#[derive(Debug)]
pub struct DamagePrototype {
    amount: f32,
    r#type: String // Damage type
}

/// <https://wiki.factorio.com/Types/DamageTypeFilters>
#[derive(Debug)]
pub struct DamageTypeFilters {
    types: Vec<String>, // If String, converted to Vec<String> with one element // Name of DamageType prototype
    whitelist: bool // Default: false
}

/// <https://wiki.factorio.com/Types/ForceCondition>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ForceCondition {
    All,
    Enemy,
    Ally,
    Friend,
    NotFriend,
    Same,
    NotSame
}

impl fmt::Display for ForceCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::All => "all",
            Self::Enemy => "enemy",
            Self::Ally => "ally",
            Self::Friend => "friend",
            Self::NotFriend => "not-friend",
            Self::Same => "same",
            Self::NotSame => "not-same",
        })
    }
}

impl FromStr for ForceCondition {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "enemy" => Ok(Self::Enemy),
            "ally" => Ok(Self::Ally),
            "friend" => Ok(Self::Friend),
            "not-friend" => Ok(Self::NotFriend),
            "same" => Ok(Self::Same),
            "not-same" => Ok(Self::NotSame),
            _ => Err(PrototypesErr::InvalidTypeStr("ForceCondition".into(), s.into()))
        }
    }
}

/// <https://wiki.factorio.com/Types/AreaTriggerItem#collision_mode>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CollisionMode {
    DistanceFromCollisionBox,
    DistanceFromCenter,
}

impl fmt::Display for CollisionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::DistanceFromCollisionBox => "distance-from-collision-box",
            Self::DistanceFromCenter => "distance-from-center",
        })
    }
}

impl FromStr for CollisionMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "distance-from-collision-box" => Ok(Self::DistanceFromCollisionBox),
            "distance-from-center" => Ok(Self::DistanceFromCenter),
            _ => Err(PrototypesErr::InvalidTypeStr("CollisionMode".into(), s.into()))
        }
    }
}

/// <https://wiki.factorio.com/Types/MinableProperties>
#[derive(Debug)]
pub struct MinableProperties {
    mining_type: f64,
    results: Vec<ProductPrototype>,
    fluid_amount: f64, // Default: 0
    mining_particle: Option<String>, // Name of Prototype/Particle
    required_fluid: Option<String>, // Name of Prototype/Fluid
    // Converted to results item
    // if results are present, these are ignored
    //result: String,
    //count: u16, // Default: 1
    mining_trigger: Option<Trigger>
}

/// <https://wiki.factorio.com/Types/ProductPrototype>
#[derive(Debug)]
pub enum ProductPrototype {
    /// type = "item" // Default
    Item(ItemProductPrototype),
    /// type = "fluid"
    Fluid(FluidProductPrototype)
}

/// Either a sequence or a table, first item stands for name and second for amount
/// <https://wiki.factorio.com/Types/ItemProductPrototype>
#[derive(Debug)]
pub struct ItemProductPrototype {
    name: String, // Name of Prototype/Item
    show_details_in_recipe_tooltip: bool, // Default: true
    amount: Option<u16>, // Mandatory when defined in a sequence
    probability: f64, // Default: 1
    amount_min: Option<u16>, // Mandatory if amount is not specified
    amount_max: Option<u16>, // Mandatory if amount is not specified // Set to amount_min if amount_max < amount_min
    catalyst_amount: u16, // Default: 0
}

/// <https://wiki.factorio.com/Types/FluidProductPrototype>
#[derive(Debug)]
pub struct FluidProductPrototype {
    name: String, // Name of Prototype/Fluid
    show_details_in_recipe_tooltip: bool, // Default: true
    probability: f64, // Default: 1
    amount: Option<u16>, // Mandatory when defined in a sequence // Cannot be < 0
    amount_min: Option<u16>, // Mandatory if amount is not specified // Cannot be < 0
    amount_max: Option<u16>, // Mandatory if amount is not specified // Set to amount_min if amount_max < amount_min
    temperature: Option<f64>,
    catalyst_amount: f64, // Default: 0
    fuildbox_index: u32, // Default: 0
}

/// <https://wiki.factorio.com/Prototype/Entity#remove_decoratives>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RemoveDecoratives {
    Automatic,
    True,
    False,
}

impl fmt::Display for RemoveDecoratives {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Automatic => "automatic",
            Self::True => "true",
            Self::False => "false",
        })
    }
}

impl FromStr for RemoveDecoratives {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            _ => Err(PrototypesErr::InvalidTypeStr("RemoveDecoratives".into(), s.into()))
        }
    }
}

/// <https://wiki.factorio.com/Types/ItemToPlace>
#[derive(Debug)]
pub struct ItemToPlace {
    item: String, // Name of Item
    count: u32 // Can't be larger than the stack size of the item
}

/// <https://wiki.factorio.com/Prototype/Cliff#orientations>
#[derive(Debug)]
pub struct OrientedCliffPrototypes {
    west_to_east: OrientedCliffPrototype,
    north_to_south: OrientedCliffPrototype,
    east_to_west: OrientedCliffPrototype,
    south_to_north: OrientedCliffPrototype,
    west_to_north: OrientedCliffPrototype,
    north_to_east: OrientedCliffPrototype,
    east_to_south: OrientedCliffPrototype,
    south_to_west: OrientedCliffPrototype,
    west_to_south: OrientedCliffPrototype,
    north_to_west: OrientedCliffPrototype,
    east_to_north: OrientedCliffPrototype,
    south_to_east: OrientedCliffPrototype,
    west_to_none: OrientedCliffPrototype,
    none_to_east: OrientedCliffPrototype,
    north_to_none: OrientedCliffPrototype,
    none_to_south: OrientedCliffPrototype,
    east_to_none: OrientedCliffPrototype,
    none_to_west: OrientedCliffPrototype,
    south_to_none: OrientedCliffPrototype,
    none_to_north: OrientedCliffPrototype,
}

/// <https://wiki.factorio.com/Types/OrientedCliffPrototype>
#[derive(Debug)]
pub struct OrientedCliffPrototype {
    collision_bounding_box: BoundingBox,
    pictures: Vec<SpriteVariation>,
    fill_volume: u32
}

/// <https://wiki.factorio.com/Prototype/RailRemnants#bending_type>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BendingType {
    Straight,
    Turn,
}

impl FromStr for BendingType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "straight" => Ok(Self::Straight),
            "turn" => Ok(Self::Turn),
            _ => Err(PrototypesErr::InvalidTypeStr("BendingType".into(), s.into()))
        }
    }
}

impl fmt::Display for BendingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Straight => "straight",
            Self::Turn => "turn",
        })
    }
}

/// <https://wiki.factorio.com/Types/ExplosionDefinition>
#[derive(Debug)]
pub struct ExplosionDefinition {
    name: String, // Name of Prototype/Entity
    offset: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/Resistances>
#[derive(Debug)]
pub struct Resistance {
    r#type: String, // Name of Prototype/DamageType
    decrease: f32, // Default: 0
    percent: f32, // Default: 0
}

/// <https://wiki.factorio.com/Types/Loot>
#[derive(Debug)]
pub struct Loot {
    item: String, // Name of Prototype/Item
    probability: f64, // Default: 1
    count_min: f64, // Default: 1
    count_max: f64, // Default: 1 // Must be > 0
}

/// <https://wiki.factorio.com/Types/AttackReactionItem>
#[derive(Debug)]
pub struct AttackReactionItem {
    range: f32,
    action: Option<Trigger>,
    reaction_modifier: f32, // Default: 0
    damage_type: Option<String>, // name of Prototype/DamageType
}

/// <https://wiki.factorio.com/Types/EnergySource>
#[derive(Debug)]
pub struct EnergySourceBase {
    emissions_per_minute: f64, // Default: 0
    render_no_power_icon: bool, // Default: true
    render_no_network_icon: bool, // Default: true
}

/// <https://wiki.factorio.com/Types/EnergySource>
#[derive(Debug)]
pub enum EnergySource {
    /// <https://wiki.factorio.com/Types/EnergySource#Electric_energy_source>
    Electric(ElectricEnergySource),
    /// <https://wiki.factorio.com/Types/EnergySource#Burner>
    Burner(BurnerEnergySource),
    /// <https://wiki.factorio.com/Types/EnergySource#Heat_energy_source>
    Heat(HeatEnergySource),
    /// <https://wiki.factorio.com/Types/EnergySource#Fluid_energy_source>
    Fluid(FluidEnergySource),
    /// <https://wiki.factorio.com/Types/EnergySource#Void_energy_source>
    Void
}

/// <https://wiki.factorio.com/Types/EnergySource#Electric_energy_source>
#[derive(Debug)]
pub struct ElectricEnergySource {
    base: EnergySourceBase,
    buffer_capacity: Option<Energy>,
    usage_priority: ElectricUsagePriority,
    input_flow_limit: Energy, // Default: f64::MAX
    output_flow_limit: Energy, // Default: f64::MAX
    drain: Option<Energy>
}

/// <https://wiki.factorio.com/Types/EnergySource#Burner>
#[derive(Debug)]
pub struct BurnerEnergySource {
    base: EnergySourceBase,
    fuel_inventory_size: ItemStackIndex,
    burnt_inventory_size: ItemStackIndex, // Default: 0
    smoke: Option<Vec<SmokeSource>>,
    light_flicker: Option<LightFlickeringDefinition>,
    effectivity: f64, // Default: 1
    fuel_categories: Vec<String>, // Default: "chemical"
}

/// <https://wiki.factorio.com/Types/EnergySource#Heat_energy_source>
#[derive(Debug)]
pub struct HeatEnergySource {
    base: EnergySourceBase,
    max_temperature: f64, // Must be >= default_temperature
    default_temperature: f64, // Default: 15
    specific_heat: Energy,
    max_transfer: Energy,
    max_temperature_gradient: f64, // Default: 1
    min_working_temperature: f64, // Default: 15 // Must be >= default_temperature AND <= max_temperature
    minimum_glow_temperature: f32, // Default: 1
    pipe_covers: Option<Sprite4Way>,
    heat_pipe_covers: Option<Sprite4Way>,
    heat_picture: Option<Sprite4Way>,
    heat_glow: Option<Sprite4Way>,
    connections: Option<Vec<HeatConnection>> // Up to 32 connections
}

/// <https://wiki.factorio.com/Types/EnergySource#Fluid_energy_source>
#[derive(Debug)]
pub struct FluidEnergySource {
    base: EnergySourceBase,
    fluid_box: FluidBox,
    smoke: Option<Vec<SmokeSource>>,
    light_flicker: Option<LightFlickeringDefinition>,
    effectivity: f64, // Default: 1
    burns_fluid: bool, // Default: false
    scale_fluid_usage: bool, // Default: false
    fluid_usage_per_tick: f64, // Default: 0
    maximum_temperature: f64, // Default: f64::INFINITY
}

/// <https://wiki.factorio.com/Types/ElectricUsagePriority>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

impl FromStr for ElectricUsagePriority {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "primary-input" => Ok(Self::PrimaryInput),
            "primary-output" => Ok(Self::PrimaryOutput),
            "secondary-input" => Ok(Self::SecondaryInput),
            "secondary-output" => Ok(Self::SecondaryOutput),
            "tertiary" => Ok(Self::Tertiary),
            "solar" => Ok(Self::Solar),
            "lamp" => Ok(Self::Lamp),
            _ => Err(PrototypesErr::InvalidTypeStr("ElectricUsagePriority".into(), s.into()))
        }
    }
}

impl fmt::Display for ElectricUsagePriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::PrimaryInput => "primary-input",
            Self::PrimaryOutput => "primary-output",
            Self::SecondaryInput => "secondary-input",
            Self::SecondaryOutput => "secondary-output",
            Self::Tertiary => "tertiary",
            Self::Solar => "solar",
            Self::Lamp => "lamp",
        })
    }
}

/// <https://wiki.factorio.com/Types/SmokeSource>
#[derive(Debug)]
pub struct SmokeSource {
    name: String, // Name of Prototype/TrivialSmoke
    frequency: f64, // Can't be negative, NaN or infinite
    offset: f64, // Default: 0
    position: Option<Factorio2DVector>,
    north_position: Option<Factorio2DVector>,
    east_position: Option<Factorio2DVector>,
    south_position: Option<Factorio2DVector>,
    west_position: Option<Factorio2DVector>,
    deviation: Option<Position>,
    starting_frame_speed: u16, // Default: 0
    starting_frame_speed_deviation: f64, // Default: 0
    starting_frame: u16, // Default: 0
    starting_frame_deviation: f64, // Default: 0
    slow_down_factor: u8, // Default: 1
    height: f32, // Default: 0
    height_deviation: f32, // Default: 0
    starting_vertical_speed: f32, // Default: 0
    starting_vertical_speed_deviation: f32, // Default: 0
    vertical_speed_slowdown: f32 // Default: 0.965
}

/// <https://wiki.factorio.com/Types/HeatConnection>
#[derive(Debug)]
pub struct HeatConnection {
    position: Position,
    direction: Direction
}

/// <https://wiki.factorio.com/Types/FluidBox>
#[derive(Debug)]
pub struct FluidBox {
    pipe_connections: Vec<PipeConnectionDefinition>, // Max: 256
    base_area: f64, // Default: 1 // Must be > 0
    base_level: f64, // Default: 0
    height: f64, // Default: 1 // Must be > 0
    filter: Option<String>, // Name of Prototype/Fluid
    render_layer: RenderLayer, // Default: "object"
    pipe_covers: Option<Sprite4Way>,
    minimum_temperature: Option<f64>,
    maximum_temperature: Option<f64>,
    production_type: Option<ProductionType>, // Default: None
    //secondary_draw_order: u8, // Default: 1 // Converted to secondary_draw_orders
    secondary_draw_orders: SecondaryDrawOrders // Default: (north = 1, east = 1, south = 1, west = 1)
}

/// <https://wiki.factorio.com/Types/PipeConnectionDefinition>
#[derive(Debug)]
pub struct PipeConnectionDefinition {
    positions: Vec<Factorio2DVector>, // `position` takes priority and gets converted to this
    max_underground_distance: u32, // Default: 0
    r#type: ProductionType, // Default: "input-output"
}

/// <https://wiki.factorio.com/Types/Direction>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Direction(u32);

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Into<u32> for Direction {
    fn into(self) -> u32 {
        self.0
    }
}

/// <https://wiki.factorio.com/Types/FluidBox#production_type>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ProductionType {
    Input,
    InputOutput,
    Output,
}

impl FromStr for ProductionType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "input" => Ok(Self::Input),
            "input-output" => Ok(Self::InputOutput),
            "output" => Ok(Self::Output),
            _ => Err(PrototypesErr::InvalidTypeStr("ProductionType".into(), s.into()))
        }
    }
}

impl fmt::Display for ProductionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Input => "input",
            Self::InputOutput => "input-output",
            Self::Output => "output",
        })
    }
}

/// <https://wiki.factorio.com/Types/WireConnectionPoint>
#[derive(Debug)]
pub struct WireConnectionPoint {
    wire: WirePosition,
    shadow: WirePosition
}

/// <https://wiki.factorio.com/Types/WirePosition>
#[derive(Debug)]
pub struct WirePosition {
    copper: Option<Factorio2DVector>,
    red: Option<Factorio2DVector>,
    green: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/SignalIDConnector>
#[derive(Debug)]
pub struct SignalIDConnector {
    r#type: SignalIDConnectorType,
    name: String, // Name of a circuit network signal
}

/// <https://wiki.factorio.com/Types/SignalIDConnector#type>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SignalIDConnectorType {
    Virtual,
    Item,
    Fluid,
}

impl FromStr for SignalIDConnectorType {
    type Err = PrototypesErr;
    // All fancy shenanigans are omitted, this program/library behaves like a game

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "virtual" => Ok(Self::Virtual),
            "item" => Ok(Self::Item),
            "fluid" => Ok(Self::Fluid),
            _ => Err(PrototypesErr::InvalidTypeStr("SignalIDConnectorType".into(), s.into()))
        }
    }
}

impl fmt::Display for SignalIDConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Virtual => "virtual",
            Self::Item => "item",
            Self::Fluid => "fluid",
        })
    }
}

/// <https://wiki.factorio.com/Types/ModuleSpecification>
#[derive(Debug)]
pub struct ModuleSpecification {
    module_slots: u16, // Default: 0
    module_info_max_icons_per_row: u8, // Default: width of selection box / 0,75
    module_info_max_icon_rows: u8, // Default: width of selection box / 1.5
    module_info_icon_shift: Factorio2DVector, // Default: (0, 0.7)
    module_info_icon_scale: f32, // Default: 0.5
    module_info_separation_multiplier: f32, // Default: 1.1
    module_info_multi_row_initial_height_modifier: f32 // Default: -0.1
}

/// <https://wiki.factorio.com/Types/EffectTypeLimitation>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct EffectTypeLimitation(u8);

impl EffectTypeLimitation {
    pub const SPEED: Self = Self(1);
    pub const PRODUCTIVITY: Self = Self(1 << 1);
    pub const CONSUMPTION: Self = Self(1 << 2);
    pub const POLLUTION: Self = Self(1 << 3);
}

impl From<Vec<&str>> for EffectTypeLimitation {
    fn from(in_arr: Vec<&str>) -> Self {
        let mut result = Self(0);        for item in in_arr {
            match item {
                "speed" => result |= Self::SPEED,
                "productivity" => result |= Self::PRODUCTIVITY,
                "consumption" => result |= Self::CONSUMPTION,
                "pollution" => result |= Self::POLLUTION,
                _ => {}            }
        }
        result
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
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BoilerMode {
    HeatWaterInside,
    OutputToSeparatePipe,
}

impl FromStr for BoilerMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "heat-water-inside" => Ok(Self::HeatWaterInside),
            "output-to-separate-pipe" => Ok(Self::OutputToSeparatePipe),
            _ => Err(PrototypesErr::InvalidTypeStr("BoilerMode".into(), s.into()))
        }
    }
}

impl fmt::Display for BoilerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::HeatWaterInside => "heat-water-inside",
            Self::OutputToSeparatePipe => "output-to-separate-pipe",
        })
    }
}

/// <https://wiki.factorio.com/Types/FootprintParticle>
#[derive(Debug)]
pub struct FootprintParticle {
    tiles: Vec<String>, // (Names) Name of a tile
    particle_name: Option<String>, // Name of a particle
    use_as_default: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/LogisticContainer#logistic_mode>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum LogisticMode {
    PassiveProvider,
    ActiveProvider,
    Storage,
    Buffer,
    Requester,
}

impl FromStr for LogisticMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "passive-provider" => Ok(Self::PassiveProvider),
            "active-provider" => Ok(Self::ActiveProvider),
            "storage" => Ok(Self::Storage),
            "buffer" => Ok(Self::Buffer),
            "requester" => Ok(Self::Requester),
            _ => Err(PrototypesErr::InvalidTypeStr("LogisticMode".into(), s.into()))
        }
    }
}

impl fmt::Display for LogisticMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::PassiveProvider => "passive-provider",
            Self::ActiveProvider => "active-provider",
            Self::Storage => "storage",
            Self::Buffer => "buffer",
            Self::Requester => "requester",
        })
    }
}

/// Used in many places, specified as string
/// <https://wiki.factorio.com/Prototype/ElectricEnergyInterface#gui_mode>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum GuiMode {
    All,
    None,
    Admins,
}

impl FromStr for GuiMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "admins" => Ok(Self::Admins),
            _ => Err(PrototypesErr::InvalidTypeStr("GuiMode".into(), s.into()))
        }
    }
}

impl fmt::Display for GuiMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::All => "all",
            Self::None => "none",
            Self::Admins => "admins",
        })
    }
}

// Can also be converted from array
/// <https://wiki.factorio.com/Types/UnitSpawnDefinition>
#[derive(Debug)]
pub struct UnitSpawnDefinition {
    unit: String, // Name of Entity
    spawn_points: Vec<SpawnPoint> // `evolution_factor` must be ascending from entry to entry
}

// Can also be converted from array
/// <https://wiki.factorio.com/Types/SpawnPoint>
#[derive(Debug)]
pub struct SpawnPoint {
    evolution_factor: f64,
    spawn_height: f64, // Must be >= 0
}

/// <https://wiki.factorio.com/Types/AmmoType>
#[derive(Debug)]
pub struct AmmoType {
    category: String, // Name of AmmoCategory
    action: Option<Trigger>,
    clamp_position: bool, // Default: false // Forced to be false if `target_type` is "entity"
    energy_consumption: Option<Energy>,
    range_modifier: f64, // Default: 1
    cooldown_modifier: f64, // Default: 1
    consumption_modifier: f64, // Default: 1
    target_type: TargetType
}

/// <https://wiki.factorio.com/Types/AmmoType#target_type>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TargetType {
    Entity,
    Position,
    Direction,
}

impl FromStr for TargetType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "entity" => Ok(Self::Entity),
            "position" => Ok(Self::Position),
            "direction" => Ok(Self::Direction),
            _ => Err(PrototypesErr::InvalidTypeStr("TargetType".into(), s.into()))
        }
    }
}

impl fmt::Display for TargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Entity => "entity",
            Self::Position => "position",
            Self::Direction => "direction",
        })
    }
}

/// <https://wiki.factorio.com/Types/CircularProjectileCreationSpecification>
pub type CircularParticleCreationSpecification = Vec<(RealOrientation, Factorio2DVector)>;

/// <https://wiki.factorio.com/Types/HeatBuffer>
#[derive(Debug)]
pub struct HeatBuffer {
    max_temperature: f64, // Must be >= `default_temperature`
    specific_heat: Energy,
    max_transfer: Energy,
    default_temperature: f64, // Default: 15
    min_temperature_gradient: f64, // Default: 1
    min_working_temperature: f64, // Default: 15
    minimum_glow_temperature: f32, // Default: 1
    pipe_covers: Option<Sprite4Way>,
    heat_pipe_covers: Option<Sprite4Way>,
    heat_picture: Option<Sprite4Way>,
    heat_glow: Option<Sprite4Way>,
    connections: Option<Vec<HeatConnection>> // 32 max
}

/// <https://wiki.factorio.com/Types/SignalColorMapping>
#[derive(Debug)]
pub struct SignalColorMapping {
    r#type: SignalType,
    name: String, // Name of a signal
    color: Color
}

/// <https://wiki.factorio.com/Types/SignalColorMapping#type>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum SignalType {
    Virtual,
    Item,
    Fluid,
}

impl FromStr for SignalType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "virtual" => Ok(Self::Virtual),
            "item" => Ok(Self::Item),
            "fluid" => Ok(Self::Fluid),
            _ => Err(PrototypesErr::InvalidTypeStr("SignalType".into(), s.into()))
        }
    }
}

impl fmt::Display for SignalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Virtual => "virtual",
            Self::Item => "item",
            Self::Fluid => "fluid",
        })
    }
}

/// <https://wiki.factorio.com/Prototype/ProgrammableSpeaker#instruments>
#[derive(Debug)]
pub struct Instrument {
    name: String,
    notes: Vec<Note>
}

/// <https://wiki.factorio.com/Prototype/ProgrammableSpeaker#instruments>
#[derive(Debug)]
pub struct Note {
    name: String,
    sound: Sound
}

/// <https://wiki.factorio.com/Types/AnimatedVector>
#[derive(Debug)]
pub struct AnimatedVector {
    rotations: Vec<AnimatedVectorRotation>,
    //render_layer: Option<RenderLayer>, // Just copied over to all rotations
    direction_shift: Option<AnimatedVectorDirectionShift>
}

/// <https://wiki.factorio.com/Types/AnimatedVector#rotations>
#[derive(Debug)]
pub struct AnimatedVectorRotation {
    frames: Vec<Factorio2DVector>, // Sizes of all arrays must be the same
    render_layer: RenderLayer
}

/// <https://wiki.factorio.com/Types/AnimatedVector#direction_shift>
#[derive(Debug)]
pub struct AnimatedVectorDirectionShift {
    north: Option<Factorio2DVector>,
    east: Option<Factorio2DVector>,
    south: Option<Factorio2DVector>,
    west: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Types/UnitAISettings>
#[derive(Debug)]
pub struct UnitAISettings {
    destroy_when_commands_fail: bool, // Default: false
    allow_try_return_to_spawner: bool, // Default: false
    do_separation: bool, // Default: true
    path_resolution_modifier: i8, // Default: 0 // Must be between -8 and 8
}

/// <https://wiki.factorio.com/Prototype/Unit#alternative_attacking_frame_sequence>
#[derive(Debug)]
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
    back_to_walk_animation_speed: f32
}

/// <https://wiki.factorio.com/Types/SpiderEnginePrototype>
#[derive(Debug)]
pub struct SpiderEnginePrototype {
    military_target: String, // Name of simple entity with force prototype
    legs: Vec<SpiderLegSpecification> // Single leg is converted to Vec with one leg
}

/// <https://wiki.factorio.com/Types/SpiderLegSpecification>
#[derive(Debug)]
pub struct SpiderLegSpecification {
    leg: String, // Name of SpiderLeg
    mount_position: Factorio2DVector,
    ground_position: Factorio2DVector,
    blocking_legs: Vec<u32>,
    leg_hit_the_ground_trigger: Option<TriggerEffect>
}

/// <https://wiki.factorio.com/Prototype/FireFlame#burnt_patch_alpha_variations>
#[derive(Debug)]
pub struct FireFlameBurntPatchAlphaVariation {
    tile: String, // Name of a tile
    alpha: f32
}

/// <https://wiki.factorio.com/Prototype/FlyingText>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl FromStr for TextAlignment {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            _ => Err(PrototypesErr::InvalidTypeStr("TextAlignment".into(), s.into()))
        }
    }
}

impl fmt::Display for TextAlignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        })
    }
}

/// <https://wiki.factorio.com/Types/CursorBoxType>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
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

impl FromStr for CursorBoxType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "entity" => Ok(Self::Entity),
            "not-allowed" => Ok(Self::NotAllowed),
            "electricity" => Ok(Self::Electricity),
            "pair" => Ok(Self::Pair),
            "copy" => Ok(Self::Copy),
            "train-visualization" => Ok(Self::TrainVisualization),
            "logistics" => Ok(Self::Logistics),
            "blueprint-snap-rectangle" => Ok(Self::BlueprintSnapRectangle),
            _ => Err(PrototypesErr::InvalidTypeStr("CursorBoxType".into(), s.into()))
        }
    }
}

impl fmt::Display for CursorBoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Entity => "entity",
            Self::NotAllowed => "not-allowed",
            Self::Electricity => "electricity",
            Self::Pair => "pair",
            Self::Copy => "copy",
            Self::TrainVisualization => "train-visualization",
            Self::Logistics => "logistics",
            Self::BlueprintSnapRectangle => "blueprint-snap-rectangle",
        })
    }
}

/// <https://wiki.factorio.com/Types/EquipmentShape>
#[derive(Debug)]
pub struct EquipmentShape {
    width: u32,
    height: u32,
    shape_type: EquipmentShapeType,
    points: Option<EquipmentShapePoints> // Mandatory if type is manual
}

/// <https://wiki.factorio.com/Types/EquipmentShape#type>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum EquipmentShapeType {
    Full,
    Manual,
}

impl FromStr for EquipmentShapeType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => Ok(Self::Full),
            "manual" => Ok(Self::Manual),
            _ => Err(PrototypesErr::InvalidTypeStr("EquipmentShapeType".into(), s.into()))
        }
    }
}

impl fmt::Display for EquipmentShapeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Full => "full",
            Self::Manual => "manual",
        })
    }
}

// Constructor should accept width and height, as points can't exceed them.
/// <https://wiki.factorio.com/Types/EquipmentShape#points>
#[derive(Debug)]
pub struct EquipmentShapePoints(Vec<Vec<u32>>);

/// <https://wiki.factorio.com/Prototype/NightVisionEquipment>
#[derive(Debug)]
pub struct DaytimeColorLookupTable(Vec<(f64, ColorLookupTable)>);

/// <https://wiki.factorio.com/Types/DaytimeColorLookupTable#Second_member>
#[derive(Debug)]
pub enum ColorLookupTable {
    Identity,
    Filename(FileName)
}

/// <https://wiki.factorio.com/Types/PlaceAsTile>
#[derive(Debug)]
pub struct PlaceAsTile {
    result: String, // Name of Tile
    condition: CollisionMask,
    condition_size: i32
}

/// <https://wiki.factorio.com/Types/ItemPrototypeFlags>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
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

impl From<Vec<&str>> for ItemPrototypeFlags {
    fn from(in_arr: Vec<&str>) -> Self {
        let mut result = Self(0);        for item in in_arr {
            match item {
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
                _ => {}            }
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

