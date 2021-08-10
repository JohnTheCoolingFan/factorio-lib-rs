use std::collections::HashMap;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use std::str::FromStr;
use std::fmt;
use crate::prototypes::PrototypesErr;
use factorio_lib_rs_derive::{TriggerEffectItemBase, CreateEntityTriggerEffectItemBase, TriggerItemBase};

pub type FileName = String;
pub type ItemStackIndex = u16;
pub type Factorio2DVector = (f64, f64);
pub type AnimationFrameSequence = Vec<u16>;
pub type SpriteSize = (i16, i16); // sidth, then height
pub type SpritePosition = (i16, i16);
pub type KeySequence = String; // Parser and checker maybe?
pub type BoundingBox = (Position, Position); // Consider adding Option<f32> as specified in https://wiki.factorio.com/Types/BoundingBox?
                                             // It's kinda undocumented

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position(i32, i32);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub struct Sound {
    aggregation: Option<SoundAggregation>,
    allow_random_repeat: Option<bool>,
    audible_distance_modifier: Option<f64>,
    variations: Vec<SoundVariation> // If variations table not present, use the same table, but construct single variation.
}

#[derive(Debug)]
pub struct SoundAggregation {
    max_count: u32,
    progress_threshold: Option<f32>,
    remove: bool,
    count_already_playing: Option<bool>
}

#[derive(Debug)]
pub struct SoundVariation {
    filename: FileName,
    volume: Option<f32>,
    preload: Option<bool>,
    speed: Option<f32>,
    min_speed: Option<f32>, // >= 1/64, Ignored if speed is present
    max_speed: Option<f32>  // Mandatory if min_speed is present, >= min_speed
}

#[derive(Debug)]
pub enum AnimationType {
    Layers(Vec<AnimationType>),
    Animation(Animation)
}

#[derive(Debug)]
pub struct Animation {
    regular: AnimationVariant,
    hr_version: Option<AnimationVariant>,
}

#[derive(Debug)]
pub enum AnimationVariant {
    Regular(AnimationSpec),
    Stripes(Vec<Stripe>)
}

#[derive(Debug)]
pub struct AnimationSpec {
    // These types share same fields/values, so I decided to "combine" them
    sprite: SpriteSpec,
    run_mode: RunMode, // Default: "forward"
    frame_count: u32, // Default: 1, can't be 0
    line_length: u32, // Default: 0
    animation_speed: f32, // Default: 1.0
    max_advance: f32, // Default: MAX_FLOAT
    repeat_count: u8, // Default: 1, can't be 0
    frame_sequence: Option<AnimationFrameSequence>,
}

#[derive(Debug)]
pub struct Sprite {
    layers: Vec<SpriteLayer>
}

#[derive(Debug)]
pub struct SpriteLayer {
    regular: SpriteSpec,
    hr_version: Option<SpriteSpec>
}

#[derive(Debug)]
pub struct SpriteSpec {
    filename: FileName,
    dice: Option<Dice>, // AKA slice // _y and _x are converted into this
    priority: SpritePriority, // Default: "medium"
    flags: Option<SpriteFlags>,
    size: Option<SpriteSize>,
    // Automatically converted to size
    // width
    // height
    position: Option<SpritePosition>,
    // Automatically converted to position
    // x
    // y
    shift: Factorio2DVector, // (0, 0) by default
    scale: f64, // 1 by default,
    draw_as: DrawAs, // all false by default
    mipmap_count: u8, // Default: 0
    apply_runtime_tint: bool, // Default: false
    tint: Color, // Default: (1, 1, 1, 1) (white)
    blend_mode: BlendMode, // Default: "normal"
    load_in_minimal_mode: bool, //Default: false
    premul_alpha: bool, // Default: true
    generate_sfd: bool // Default: false // Unused (Then why it is documented?)
}

#[derive(Debug)]
pub struct SpriteVariation {
    layers: Vec<SpriteVariationLayer>
}

#[derive(Debug)]
pub struct SpriteVariationLayer {
    regular: SpriteVariationSpec,
    hr_version: Option<SpriteVariationSpec>
}

// Extension (or side-step?) of SpriteSpec
// Ignores dice and slice
#[derive(Debug)]
pub struct SpriteVariationSpec {
    sprite: SpriteSpec,
    variation_count: u32, // Default: 1
    repeat_count: u32, // Default: 1
    line_length: u32 // Default: value of `variation_count`
}

#[derive(Debug)]
pub struct Dice(i16, i16);

impl Dice {
    pub fn new(n: i16) -> Self {
        Self(n, n)
    }

    pub fn new_xy(x: i16, y: i16) -> Self {
        Self(x, y)
    }
}

#[derive(Debug)]
pub enum DrawAs {
    DrawAsShadow,
    DrawAsGlow,
    DrawAsLight,
    None
}

impl DrawAs {
    pub fn new(draw_as_shadow: bool, draw_as_glow: bool, draw_as_light: bool) -> Self {
        if draw_as_shadow {
            Self::DrawAsShadow
        } else if draw_as_glow {
            Self::DrawAsGlow
        } else if draw_as_light {
            Self::DrawAsLight
        } else {
            Self::None
        }
    }
}

#[derive(Debug)]
pub enum BlendMode {
    Normal,
    Additive,
    AdditiveSoft,
    Multiplicative,
    Overwrite
}

impl FromStr for BlendMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "additive" => Ok(Self::Additive),
            "additive-soft" => Ok(Self::AdditiveSoft),
            "multiplicative" => Ok(Self::Multiplicative),
            "overwrite" => Ok(Self::Overwrite),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("BlendMode"), String::from(s)))
        }
    }
}

#[derive(Debug)]
pub enum RunMode {
    Forward,
    Backward,
    ForwardThenBackward
}

impl FromStr for RunMode {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "backward" => Ok(Self::Backward),
            "forward-then-backward" => Ok(Self::ForwardThenBackward),
            _ => Err(PrototypesErr::InvalidTypeStr(String::from("RunMode"), String::from(s)))
        }
    }
}

#[derive(Debug)]
pub struct Stripe {
    width_in_frames: u32,
    height_in_frames: u32,
    filename: FileName,
    x: Option<u32>,
    y: Option<u32>
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SpriteFlags(u32);

impl SpriteFlags {
    pub const NO_CROP: SpriteFlags = SpriteFlags(1);
    pub const NOT_COMPRESSED: SpriteFlags = SpriteFlags(1 << 1);
    pub const ALWAYS_COMPRESSED: SpriteFlags = SpriteFlags(1 << 2);
    pub const MIPMAP: SpriteFlags = SpriteFlags(1 << 3);
    pub const LINEAR_MINIFICATION: SpriteFlags = SpriteFlags(1 << 4);
    pub const LINEAR_MAGNIFICATION: SpriteFlags = SpriteFlags(1 << 5);
    pub const LINEAR_MIP_LEVEL: SpriteFlags = SpriteFlags(1 << 6);
    pub const ALPHA_MASK: SpriteFlags = SpriteFlags(1 << 7);
    pub const NO_SCALE: SpriteFlags = SpriteFlags(1 << 8);
    pub const MASK: SpriteFlags = SpriteFlags(1 << 9);
    pub const ICON: SpriteFlags = SpriteFlags(1 << 10);
    pub const GUI: SpriteFlags = SpriteFlags(1 << 11);
    pub const GUI_ICON: SpriteFlags = SpriteFlags(1 << 12);
    pub const LIGHT: SpriteFlags = SpriteFlags(1 << 13);
    pub const TERRAIN: SpriteFlags = SpriteFlags(1 << 14);
    pub const TERRAIN_EFFECT_MAP: SpriteFlags = SpriteFlags(1 << 15);
    pub const SHADOW: SpriteFlags = SpriteFlags(1 << 16);
    pub const SMOKE: SpriteFlags = SpriteFlags(1 << 17);
    pub const DECAL: SpriteFlags = SpriteFlags(1 << 18);
    pub const LOW_OBJECT: SpriteFlags = SpriteFlags(1 << 19);
    pub const TRILINEAR_FILTERING: SpriteFlags = SpriteFlags(1 << 20);
    pub const GROUP_NONE: SpriteFlags = SpriteFlags(1 << 21);
    pub const GROUP_TERRAIN: SpriteFlags = SpriteFlags(1 << 22);
    pub const GROUP_TERRAIN_EFFECT_MAP: SpriteFlags = SpriteFlags(1 << 23);
    pub const GROUP_SHADOW: SpriteFlags = SpriteFlags(1 << 24);
    pub const GROUP_SMOKE: SpriteFlags = SpriteFlags(1 << 25);
    pub const GROUP_DECAL: SpriteFlags = SpriteFlags(1 << 26);
    pub const GROUP_LOW_OBJECT: SpriteFlags = SpriteFlags(1 << 27);
    pub const GROUP_GUI: SpriteFlags = SpriteFlags(1 << 28);
    pub const GROUP_ICON: SpriteFlags = SpriteFlags(1 << 29);
    pub const GROUP_ICON_BACKGROUND: SpriteFlags = SpriteFlags(1 << 30);
    pub const COMPRESSED: SpriteFlags = SpriteFlags(1 << 31);
}

impl From<Vec<&str>> for SpriteFlags {
    fn from(flags: Vec<&str>) -> Self {
        let mut result = Self(0);
        for flag in flags {
            match flag {
                "no-crop" => result |= SpriteFlags::NO_CROP,
                "not-compressed" => result |= SpriteFlags::NOT_COMPRESSED,
                "always-compressed" => result |= SpriteFlags::ALWAYS_COMPRESSED,
                "mipmap" => result |= SpriteFlags::MIPMAP,
                "linear-minification" => result |= SpriteFlags::LINEAR_MINIFICATION,
                "linear-maginfication" => result |= SpriteFlags::LINEAR_MAGNIFICATION,
                "linear-mip-level" => result |= SpriteFlags::LINEAR_MIP_LEVEL,
                "alpha-mask" => result |= SpriteFlags::ALPHA_MASK,
                "no-scale" => result |= SpriteFlags::NO_SCALE,
                "mask" => result |= SpriteFlags::MASK | SpriteFlags::GROUP_NONE,
                "icon" => result |= SpriteFlags::ICON |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::NO_SCALE |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::NOT_COMPRESSED |
                    SpriteFlags::GROUP_ICON,
                "gui" => result |= SpriteFlags::GUI |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::NO_SCALE |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::NOT_COMPRESSED |
                    SpriteFlags::GROUP_GUI,
                "gui-icon" => result |= SpriteFlags::GUI_ICON |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::NO_SCALE |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::NOT_COMPRESSED |
                    SpriteFlags::GROUP_ICON,
                "light" => result |= SpriteFlags::LIGHT |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::GROUP_NONE,
                "terrain" => result |= SpriteFlags::TERRAIN |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::GROUP_TERRAIN,
                "terrain-effect-map" => result |= SpriteFlags::TERRAIN_EFFECT_MAP |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MIP_LEVEL |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::NO_CROP |
                    SpriteFlags::GROUP_TERRAIN_EFFECT_MAP,
                "shadow" => result |= SpriteFlags::SHADOW,
                "smoke" => result |= SpriteFlags::SMOKE |
                    SpriteFlags::MIPMAP |
                    SpriteFlags::LINEAR_MINIFICATION |
                    SpriteFlags::LINEAR_MAGNIFICATION |
                    SpriteFlags::GROUP_SMOKE,
                "decal" => result |= SpriteFlags::DECAL |
                    SpriteFlags::GROUP_DECAL,
                "low-object" => result |= SpriteFlags::LOW_OBJECT,
                "trilinear-filtering" => result |= SpriteFlags::TRILINEAR_FILTERING,
                /*
                "group=none" => result |= SpriteFlags::GROUP_NONE,
                "group=terrain" => result |= SpriteFlags::GROUP_TERRAIN,
                "group=terrain-effect-map" => result |= SpriteFlags::GROUP_TERRAIN_EFFECT_MAP,
                "group=shadow" => result |= SpriteFlags::GROUP_SHADOW,
                "group=smoke" => result |= SpriteFlags::GROUP_SMOKE,
                "group=decal" => result |= SpriteFlags::GROUP_DECAL,
                "group=low-object" => result |= SpriteFlags::GROUP_LOW_OBJECT,
                "group=gui" => result |= SpriteFlags::GROUP_GUI,
                "group=icon" => result |= SpriteFlags::GROUP_ICON,
                "group=icon-background" => result |= SpriteFlags::GROUP_ICON_BACKGROUND, */
                "compressed" => result |= SpriteFlags::COMPRESSED,
                _ => {}
            }
        }
        result
    }
}

impl BitAnd for SpriteFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        SpriteFlags(self.0 & rhs.0)
    }
}

impl BitAndAssign for SpriteFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = SpriteFlags(self.0 & rhs.0)
    }
}

impl BitOr for SpriteFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        SpriteFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for SpriteFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = SpriteFlags(self.0 | rhs.0)
    }
}

impl BitXor for SpriteFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        SpriteFlags(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for SpriteFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = SpriteFlags(self.0 ^ rhs.0)
    }
}

#[derive(Debug)]
pub enum SpritePriority {
    ExtraHighNoScale,
    ExtraHigh,
    High,
    Medium,
    Low,
    VeryLow,
    NoAtlas
}

#[derive(Debug)]
pub enum MapGenPreset {
    // Decided by `default` field
    Default(MapGenPresetDefault),
    NonDefault(MapGenPresetNonDefault)
}

#[derive(Debug)]
pub struct MapGenPresetDefault {
    order: String
}

#[derive(Debug)]
pub struct MapGenPresetNonDefault {
    order: String,
    // Should these be optional or just have defaults? TODO
    basic_settings: Option<MapGenPresetBasicSettings>,
    advanced_settings: Option<MapGenPresetAdvancedSettings>
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct MapGenAutoplaceControl {
    frequency: Option<MapGenSize>,
    size: Option<MapGenSize>,
    rechness: Option<MapGenSize>,
}

#[derive(Debug)]
pub struct CliffPlacementSettings {
    name: String, // Name of the cliff prototype
    cliff_elevation_0: f32, // Default 10.0
    cliff_elevation_interval: f32,
    richness: MapGenSize
}

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

#[derive(Debug)]
pub struct MapGenPresetAdvancedSettings {
    // Defaults are not documented too
    pollution: MapGenPollution,
    enemy_evolution: MapGenEnemyEvolution,
    enemy_expansion: MapGenEnemyExpansion,
    difficulty_settings: MapGenDifficultySettings
}

#[derive(Debug)]
pub struct MapGenPollution {
    enabled: bool,
    diffusion_ratio: f64, // Must be <= 0.25
    ageing: f64, // Must be >= 0.5
    enemy_attack_pollution_consumption_modifier: f64,
    min_pollution_to_damage_trees: f64,
    pollution_restored_per_tree_damage: f64
}

#[derive(Debug)]
pub struct MapGenEnemyEvolution {
    enabled: bool,
    time_factor: f64,
    destroy_factor: f64,
    pollution_factor: f64
}

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

#[derive(Debug)]
pub struct MapGenDifficultySettings {
    recipe_difficulty: DifficultySetting,
    technology_difficulty: DifficultySetting,
    technology_price_multiplier: f64,
    research_queue_setting: ResearchQueueSetting
}

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

#[derive(Debug)]
pub struct MapSteering {
    default: MapSteeringSettings,
    moving: MapSteeringSettings
}

#[derive(Debug)]
pub struct MapSteeringSettings {
    radius: f64,
    separation_factor: f64,
    separation_force: f64,
    force_unit_fuzzy_goto_behavior: bool
}

#[derive(Debug)]
pub struct MapEnemyEvolution {
    enabled: bool,
    time_factor: f64,
    destroy_factor: f64,
    pollution_factor: f64
}

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

#[derive(Debug)]
pub struct MapDifficultySettings {
    recipe_difficulty: DifficultySetting,
    technology_difficulty: DifficultySetting,
    technology_price_multiplier: f64, // Default: 1.0 // Must be >= 0.001 and <= 1000.0
    research_queue_setting: Option<ResearchQueueSetting>
}

#[derive(Debug)]
pub enum MouseCursorType {
    SystemCursor(SystemCursor),
    CustomCursor(CustomCursor)
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CustomCursor {
    filename: FileName,
    hot_pixel_x: i16,
    hot_pixel_y: i16
}

#[derive(Debug)]
pub enum IconSpecification {
    Icon(IconSpec),
    Icons(IconsSpec)
}

#[derive(Debug)]
pub struct IconSpec {
    icon: FileName,
    icon_size: i16,
    icon_mipmaps: u8 // Default: 0
}

#[derive(Debug)]
pub struct IconsSpec {
    icons: Vec<IconData>,
    // icon_size omitted here, it will be copied to each IconData
    icon_mipmaps: u8 // Default: 0
}

#[derive(Debug)]
pub struct IconData {
    icon: FileName,
    icon_size: i16, // Copied from `icon_size` from prototype
    tint: Color, // Default: (0, 0, 0 , 1)
    shift: Factorio2DVector, // Default: (0, 0)
    scale: f64, // Default: 1
    icon_mipmaps: u8 // Default: 0
}

#[derive(Debug)]
pub struct Energy(f64); // I don't know which type factorio uses internally, so I will use this

impl FromStr for Energy {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let mut rev_s = s.chars().rev();
        if rev_s.next().ok_or(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))? == 'W' {
            let next_char: char = rev_s.next().ok_or(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))?;
            if next_char.is_ascii_digit() {
                return Ok(Self(s[0..len-1].parse::<f64>().map_err(|_| PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))?))
            } else {
                let value: f64 = f64::from_str(&s[0..len-2]).map_err(|_| PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))?;

                return match next_char {
                    'k' | 'K' => Ok(Self(value * 1000.0)),
                    'M' => Ok(Self(value * 1000000.0)),
                    'G' => Ok(Self(value * 1000000000.0)),
                    'T' => Ok(Self(value * (10.0 as f64).powi(12))),
                    'P' => Ok(Self(value * (10.0 as f64).powi(15))),
                    'E' => Ok(Self(value * (10.0 as f64).powi(18))),
                    'Z' => Ok(Self(value * (10.0 as f64).powi(21))),
                    'Y' => Ok(Self(value * (10.0 as f64).powi(24))),
                    _ => Err(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))
                }
            }
        } else {
            return Err(PrototypesErr::InvalidTypeStr(String::from("Energy"), String::from(s)))
        } 
    }
}

#[derive(Debug)]
pub enum ProductType {
    Item(String),
    Fluid(String)
}

#[derive(Debug)]
pub enum ResearchTarget {
    All,
    Technology(String)
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum CustomInputAction {
    Lua,
    SpawnItem,
    TogglePersonalRoboport,
    TogglePersonalLogisticRequests,
    ToggleEquipmentMovementsBonus
}

impl FromStr for CustomInputAction {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lua" => Ok(Self::Lua),
            "spawn-item" => Ok(Self::SpawnItem),
            "toggle-personal-roboport" => Ok(Self::TogglePersonalRoboport),
            "toggle-personal-logistic-requests" => Ok(Self::TogglePersonalLogisticRequests),
            "toggle-equipment-movement-bonus" => Ok(Self::ToggleEquipmentMovementsBonus),
            _ => Err(PrototypesErr::InvalidTypeStr("CustomInputAction".into(), s.into()))
        }
    }
}

#[derive(Debug)]
pub enum RenderLayer {
    WaterTile,
    GroundTile,
    TileTransition,
    Decals,
    LowerRadiusVisualization,
    RadiusVisualization,
    TransportBeltIntegration,
    Resource,
    BuildingSmoke,
    Decorative,
    GroundPatch, // Love these names
    GroundPatchHigher,
    GroundPatchHigher2,
    Remnants,
    Floor,
    TransportBelt,
    TransportBeltEndings,
    FloorMechanicsUnderCorpse,
    Corpse,
    FloorMechanics,
    Item,
    LowerObject,
    TransportBeltCircuitConnector,
    LowerObjectAboveShadow,
    Object,
    HigherObjectUnder,
    HigherObjectAbove,
    ItemInInserterHand,
    Wires,
    WiresAbove,
    EntityInfoIcon,
    EntityInfoIconAbove,
    Explosion,
    Projectile,
    Smoke,
    AirObject,
    AirEntityInfoIcon,
    LightEffect,
    SelectionBox,
    HigherSelectionBox,
    CollisionSelectionBox,
    Arrow,
    Cursor
}

impl FromStr for RenderLayer {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "water-tile" => Ok(Self::WaterTile),
            "ground-tile" => Ok(Self::GroundTile),
            "tile-transition" => Ok(Self::TileTransition),
            "decals" => Ok(Self::Decals),
            "lower-radius-visualization" => Ok(Self::LowerRadiusVisualization),
            "radius-visualization" => Ok(Self::RadiusVisualization),
            "transport-belt-integration" => Ok(Self::TransportBeltIntegration),
            "resource" => Ok(Self::Resource),
            "building-smoke" => Ok(Self::BuildingSmoke),
            "decorative" => Ok(Self::Decorative),
            "ground-patch" => Ok(Self::GroundPatch),
            "ground-patch-higher" => Ok(Self::GroundPatchHigher),
            "ground-patch-higher2" => Ok(Self::GroundPatchHigher2),
            "remnants" => Ok(Self::Remnants),
            "floor" => Ok(Self::Floor),
            "transport-belt" => Ok(Self::TransportBelt),
            "transport-belt-endings" => Ok(Self::TransportBeltEndings),
            "floor-mechanics-under-corpse" => Ok(Self::FloorMechanicsUnderCorpse),
            "corpse" => Ok(Self::Corpse),
            "floor-mechanics" => Ok(Self::FloorMechanics),
            "item" => Ok(Self::Item),
            "lower-object" => Ok(Self::LowerObject),
            "transport-belt-circuit-connector" => Ok(Self::TransportBeltCircuitConnector),
            "lower-object-above-shadow" => Ok(Self::LowerObjectAboveShadow),
            "object" => Ok(Self::Object),
            "higher-object-under" => Ok(Self::HigherObjectUnder),
            "higher-object-above" => Ok(Self::HigherObjectAbove),
            "item-in-inserter-hand" => Ok(Self::ItemInInserterHand),
            "wires" => Ok(Self::Wires),
            "wires-above" => Ok(Self::WiresAbove),
            "entity-info-icon" => Ok(Self::EntityInfoIcon),
            "entity-info-icon-above" => Ok(Self::EntityInfoIconAbove),
            "explosion" => Ok(Self::Explosion),
            "projectile" => Ok(Self::Projectile),
            "smoke" => Ok(Self::Smoke),
            "air-object" => Ok(Self::AirObject),
            "air-entity-info-icon" => Ok(Self::AirEntityInfoIcon),
            "light-effect" => Ok(Self::LightEffect),
            "selection-box" => Ok(Self::SelectionBox),
            "higher-selection-box" => Ok(Self::HigherSelectionBox),
            "collision-selection-box" => Ok(Self::CollisionSelectionBox),
            "arrow" => Ok(Self::Arrow),
            "cursor" => Ok(Self::Cursor),
            _ => Err(PrototypesErr::InvalidTypeStr("RenderLayer".into(), s.into()))
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
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

// TODO: probably move to a separate file because this takes a lot of lines and is complicated
#[derive(Debug)]
pub enum TriggerEffect {
    Damage(DamageTriggerEffectItem),
    CreateEntity(CreateEntityTriggerEffectItem),
    CreateExplosion(CreateExplosionTriggerEffectItem),
    CreateFire(CreateFireTriggerEffectItem),
    CreateSmoke(CreateSmokeTriggerEffectItem),
    CreateTrivialSmoke(CreateTrivialSmokeEffectItem),
    CreateParticle(CreateParticleTriggerEffectItem),
    CreateSticker(CreateStickerTriggerEffectItem),
    CreateDecorative(CreateDecorativesTriggerEffectItem),
    NestedResult(Box<NestedTriggerEffectItem>),
    PlaySound(PlaySoundTriggerEffectItem),
    PushBack(PushBackTriggerEffectItem),
    DestoryCliffs(DestroyCliffsTriggerEffectItem),
    ShowExplosionOnChart(ShowExplosionOnChartTriggerEffectItem),
    InsertItem(InsertItemTriggerEffectItem),
    Script(ScriptTriggerEffectItem),
    SetTile(SetTileTriggerEffectItem),
    InvokeTileTrigger(InvokeTileEffectTriggerEffectItem),
    DestoryDecoratives(DestroyDecorativesTriggerEffectItem),
    CameraEffect(CameraEffectTriggerEffectItem),
}

#[derive(Debug)]
pub struct DamagePrototype {
    amount: f32,
    r#type: String // Damage type
}

#[derive(Debug)]
pub struct DamageTypeFilters {
    types: Vec<String>, // If String, converted to Vec<String> with one element // Name of DamageType prototype
    whitelist: bool // Default: false
}

#[derive(Debug)]
pub struct TriggerEffectItem {
    repeat_count: u16, // Default: 1
    repeat_count_deviation: u16, // Default: 0
    probability: f32, // Default: 1 // 0 < value <= 1
    affects_target: bool, // Default: false
    show_in_tooltip: bool, // Default: true // Default: false in some cases
    damage_type_filters: Option<DamageTypeFilters>
}

pub trait TriggerEffectItemBase {
    fn repeat_count(&self) -> u16;
    fn repeat_count_deviation(&self) -> u16;
    fn probability(&self) -> f32;
    fn affects_target(&self) -> bool;
    fn show_in_tooltip(&self) -> bool;
    fn damage_type_filters(&self) -> &Option<DamageTypeFilters>;
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct DamageTriggerEffectItem {
    base: TriggerEffectItem,
    damage: DamagePrototype,
    apply_damage_to_trees: bool, // Default: true
    vaporize: bool, // Default: false
    lower_distance_threshold: u16, // Default: u16::MAX
    upper_distance_threshold: u16, // Default: u16::MAX
    lower_damage_modifier: f32, // Default: 1
    upper_damage_modifier: f32  // Default: 1
}

#[derive(Debug)]
pub struct CreateEntityTriggerEffect {
    entity_name: String, // Entity name
    offset_deviation: Option<BoundingBox>,
    trigger_created_entity: bool, // Default: false
    check_buildability: bool, // Default: false
    // Override default in constructor
    show_in_tooltip: bool, // Default: false
    tile_collision_mask: Option<CollisionMask>,
    offsets: Option<Vec<Factorio2DVector>>
}

#[derive(Debug, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase)]
pub struct CreateEntityTriggerEffectItem {
    base: TriggerEffectItem,
    create_entity_base: CreateEntityTriggerEffect
}

pub trait CreateEntityTriggerEffectItemBase {
    fn entity_name(&self) -> &String;
    fn offset_deviation(&self) -> &Option<BoundingBox>;
    fn trigger_created_entity(&self) -> bool;
    fn check_buildability(&self) -> bool;
    fn show_in_tooltip(&self) -> bool;
    fn tile_collision_mask(&self) -> &Option<CollisionMask>;
    fn offsets(&self) -> &Option<Vec<Factorio2DVector>>;
}

#[derive(Debug, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase)]
pub struct CreateExplosionTriggerEffectItem {
    base: TriggerEffectItem,
    create_entity_base: CreateEntityTriggerEffect,
    max_movement_distance: f32, // Default: -1
    max_movement_distance_deviation: f32, // Default: 0
    inherit_movement_distance_from_projectile: bool, // Default: false
    cycle_while_moving: bool // Default: false
}

#[derive(Debug, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase)]
pub struct CreateFireTriggerEffectItem {
    base: TriggerEffectItem,
    create_entity_base: CreateEntityTriggerEffect,
    initial_ground_flame_count: u8 // Default: u8::MAX
}

#[derive(Debug, TriggerEffectItemBase, CreateEntityTriggerEffectItemBase)]
pub struct CreateSmokeTriggerEffectItem {
    base: TriggerEffectItem,
    create_entity_base: CreateEntityTriggerEffect,
    initial_height: f32, // Default: 0
    speed: Option<Factorio2DVector>,
    speed_multiplier: f32, // Default: 0
    speed_multiplier_deviation: f32, // Default: 0
    starting_frame: f32, // Default: 0 // Why is it f32?
    starting_frame_deviation: f32, // Default: 0
    starting_frame_speed: f32, // Default: 0
    starting_frame_speed_deviation: f32, // Default: 0
    speed_from_center: f32, // Default: 0
    speed_from_center_deviation: f32 // Default: 0
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct CreateTrivialSmokeEffectItem {
    base: TriggerEffectItem,
    smoke_name: String, // Name of TrivialSmoke prototype
    offset_deviation: Option<BoundingBox>,
    offsets: Option<Vec<Factorio2DVector>>,
    initial_height: f32, // Default: 0
    max_radius: f32, // Default: 0
    speed: Factorio2DVector, // Default: (0, 0)
    speed_multiplier: f32, // Default: 0
    speed_multiplier_deviation: f32, // Default: 0
    starting_frame: f32, // Default: 0
    starting_frame_deviation: f32, // Default: 0
    starting_frame_speed: f32, // Default: 0
    starting_frame_speed_deviation: f32, // Default: 0
    speed_from_center: f32, // Default: 0
    speed_from_center_deviation: f32 // Default: 0
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct CreateParticleTriggerEffectItem {
    base: TriggerEffectItem,
    particle_name: String, // Name of Particle prototype
    initial_height: f32,
    offset_deviation: Option<BoundingBox>,
    // show_in_tooltip: Default: false // Override in constructor
    tile_collision_mask: Option<CollisionMask>,
    offsets: Option<Vec<Factorio2DVector>>,
    initial_height_deviation: f32, // Default: 0
    initial_vertical_speed: f32, // Default: 0
    initial_vertical_speed_deviation: f32, // Default: 0
    speed_from_center: f32, // Default: 0
    speed_from_center_deviation: f32, // Default: 0
    frame_speed: f32, // Default: 1
    frame_speed_deviation: f32, // Default: 0
    tail_length: u8, // Default: 0 // Silently capped to maximum fo 100
    tail_length_deviation: u8, // Default: 0
    tail_width: f32, // Default: 1
    rotate_offsets: bool // Default: false
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct CreateStickerTriggerEffectItem {
    base: TriggerEffectItem,
    stricker: String, // Name of Sticker prototype
    // show_in_tooltip: Default: false // Override in constructor
    trigger_created_entity: bool // Default: false
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct CreateDecorativesTriggerEffectItem {
    base: TriggerEffectItem,
    decorative: String, // name of Decorative prototype
    spawn_max: u16,
    spawn_min_radius: f32,
    spawn_max_radius: f32, // Limited to < 24
    spawn_min: u16, // Default: u16
    radius_curve: f32, // Default: 0.5
    apply_projection: bool, // Default: false
    spread_evenly: bool // Default: false
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct NestedTriggerEffectItem {
    base: TriggerEffectItem,
    action: Trigger
}

#[derive(Debug)]
pub enum Trigger {
    Direct(DirectTriggerItem),
    Area(AreaTriggerItem),
    Line(LineTriggerItem),
    Cluster(ClusterTriggerItem)
}

#[derive(Debug)]
pub enum TriggerTargetMask {
    Everything,
    Specific(Vec<String>)
}

#[derive(Debug)]
pub struct TriggerItem {
    entity_flags: EntityPrototypeFlags, // Default: all flags
    ignore_collision_condition: bool, // Default: false
    trigger_target_mask: TriggerTargetMask, // Default: all flags
    repeat_count: u32, // Default: 1
    probability: f32, // Default: 1
    collision_mask: CollisionMask, // Default: all
    action_delivery: Option<Vec<TriggerDelivery>>,
    force: ForceCondition // Default: all forces
}

pub trait TriggerItemBase {
    fn entity_flags(&self) -> EntityPrototypeFlags; // Default: all flags
    fn ignore_collision_condition(&self) -> bool; // Default: false
    fn trigger_target_mask(&self) -> &TriggerTargetMask; // Default: all flags
    fn repeat_count(&self) -> u32; // Default: 1
    fn probability(&self) -> f32; // Default: 1
    fn collision_mask(&self) -> CollisionMask; // Default: all
    fn action_delivery(&self) -> &Option<Vec<TriggerDelivery>>;
    fn force(&self) -> ForceCondition; // Default: all forces
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum TriggerDelivery {
    Instant(InstantTriggerDelivery),
    Projectile(ProjectileTriggerDelivery),
    FlameThrowerExplosion(FlameThrowerExplosionTriggerDelivery),
    Beam(BeamTriggerDelivery),
    Stream(StreamTriggerDelivery),
    Artillery(ArtilleryTriggerDelivery)
}

#[derive(Debug)]
pub struct InstantTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>
}

#[derive(Debug)]
pub struct ProjectileTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    projectile: String,
    starting_speed: f32,
    starting_speed_deviation: f32, // Default: 0
    direction_deviation: f32, // Default: 0
    range_deviation: f32, // Default: 0
    max_range: f64, // Default: 1000
    min_range: f64 // Default: 0
}

#[derive(Debug)]
pub struct FlameThrowerExplosionTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    explosion: String,
    starting_distance: f64,
    direction_deviation: f32, // Default: 0
    speed_deviation: f64, // Default: 0
    starting_frame_fraction_deviation: f64, // Default: 0
    projectile_starting_speed: f64 // Default: 1
}

#[derive(Debug)]
pub struct BeamTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    beam: String, // Name of Beam prototype
    add_to_shooter: bool, // Default: true
    max_length: u32, // Default: 0
    duration: u32, // Default: 0
    source_offset: Option<Factorio2DVector>,
}

#[derive(Debug)]
pub struct StreamTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    stream: String, // Name of FluidStream prototype
    source_offset: Option<Factorio2DVector>
}

#[derive(Debug)]
pub struct ArtilleryTriggerDelivery {
    source_effects: Option<TriggerEffect>,
    target_effects: Option<TriggerEffect>,
    projectile: String, // Name of ArtilleryProjectile prototype
    starting_speed: f32,
    starting_speed_deviation: f32, // Default: 0
    direction_deviation: f32, // Default: 0
    range_deviation: f32, // Default: 0
    trigger_fired_artillery: bool // Default: false
}

#[derive(Debug, TriggerItemBase)]
pub struct DirectTriggerItem {
    base: TriggerItem,
    filter_enabled: bool // Default: false
}

#[derive(Debug, TriggerItemBase)]
pub struct AreaTriggerItem {
    base: TriggerItem,
    radius: f64,
    trigger_from_target: bool, // Default: false
    target_entities: bool, // Default: true
    show_in_tooltip: bool, // Default: true
    collision_mode: CollisionMode // Default: "distance-from-collision-box"
}

#[derive(Debug, TriggerItemBase)]
pub struct LineTriggerItem {
    base: TriggerItem,
    range: f64,
    width: f64,
    range_effects: Option<TriggerEffect>
}

#[derive(Debug, TriggerItemBase)]
pub struct ClusterTriggerItem {
    base: TriggerItem,
    cluster_count: f64, // Must be at least 2
    distance: f32,
    distance_deviation: f32 // Default: 0
}

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

#[derive(Debug, TriggerEffectItemBase)]
pub struct PlaySoundTriggerEffectItem {
    base: TriggerEffectItem,
    sound: Sound,
    // Negative values are silently clamped to 0
    min_distance: f32, // Default: 0
    max_distance: f32, // Default: 1e21
    volume_modifier: f32, // Default: 1
    audible_distance_modifier: f32, // Default: 1
    play_on_target_position: bool // Default: false
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct PushBackTriggerEffectItem {
    base: TriggerEffectItem,
    distance: f32
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct DestroyCliffsTriggerEffectItem {
    base: TriggerEffectItem,
    radius: f32,
    explosion: Option<String>, // Name of an entity
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct ShowExplosionOnChartTriggerEffectItem {
    base: TriggerEffectItem,
    scale: f32
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct InsertItemTriggerEffectItem {
    base: TriggerEffectItem,
    item: String, // Name of an item
    count: u32 // Default: 1
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct ScriptTriggerEffectItem {
    base: TriggerEffectItem,
    effect_id: String
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct SetTileTriggerEffectItem {
    base: TriggerEffectItem,
    tile_name: String, // Name of a prototype
    radius: f32,
    apply_projection: bool, // Default: false
    tile_collision_mask: CollisionMask // Default: none
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct InvokeTileEffectTriggerEffectItem {
    base: TriggerEffectItem,
    tile_collision_mask: Option<CollisionMask>
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct DestroyDecorativesTriggerEffectItem {
    base: TriggerEffectItem,
    radius: f32,
    from_render_layer: RenderLayer, // Default: first layer
    to_render_layer: RenderLayer, // Default: last layer
    include_soft_decoratives: bool, // Default: false
    include_decals: bool, // Default: false
    invoke_decorative_trigger: bool, // Default: true
    decoratives_with_trigger_only: bool // Default: false
}

#[derive(Debug, TriggerEffectItemBase)]
pub struct CameraEffectTriggerEffectItem {
    base: TriggerEffectItem,
    effect: String,
    duration: u8,
    ease_in_duration: u8, // Default: 0
    ease_out_duration: u8, // Default: 0
    delay: u8, // Default: 0
    full_strength_max_distance: u16, // Default: 0
    max_distance: u16, // Default: 0
    strength: f32, // Default: 0
}

#[derive(Debug)]
pub struct AutoplaceSpecification {
    control: String, // Default: "" // id of autoplace control
    default_enabled: bool, // Default: true
    force: String, // Default: "neutral"
    order: String, // Default: ""
    placement_density: u32, // Default: 1
    tile_restriction: Vec<String>, // Default: empty // Official docs are not clear about what this actually is, assuming it's a list of String
    base: AutoplaceSpecificationBase,
}

#[derive(Debug)]
pub enum AutoplaceSpecificationBase {
    Expression(ExpressionBasedAutoplaceSpecification),
    Peak(PeakBasedAutoplaceSpecification)
}

#[derive(Debug)]
pub struct ExpressionBasedAutoplaceSpecification {
    probability_expression: NoiseExpression,
    richness_expression: NoiseExpression
}

#[derive(Debug)]
pub struct PeakBasedAutoplaceSpecification {
    sharpness: f64, // Default: 0
    max_probability: f64, // Default: 1
    richness_base: f64, // Default: 0
    richness_multiplier: f64, // Default: 0
    richness_multiplier_distance_bonus: f64, // Default: 0
    random_probability_penalty: f64, // Default: 0
    peaks: Vec<AutoplacePeak>, // If not specified, interpret specification as peak
    coverage: f64, // Default: calculated from existing peaks // What
    starting_area_amount: u32, // Default: 0
    starting_area_size: f64, // Default: 10
}

#[derive(Debug)]
pub struct AutoplacePeak {
    influence: f64, // Default: 1
    min_influence: f64, // Default: f64::MIN
    max_influence: f64, // Default: f64::MAX
    richness_influence: f64, // Default: 0
    noise_layer: String, // Default: ""
    noise_persistence: f64, // Default: 0.5
    noise_octaves_difference: f64, // Default: 0
    noise_scale: f64, // Default: 1
    dimensions: Vec<Dimension> // Default: empty // Only one of each type
}

#[derive(Debug)]
pub enum Dimension {
    StartingAreaWeight(DimensionSpec),
    Elevation(DimensionSpec),
    Water(DimensionSpec),
    Temperature(DimensionSpec),
    Aux(DimensionSpec),
    TierFromStart(DimensionSpec),
    Distance(DimensionSpec),
}

#[derive(Debug)]
pub struct DimensionSpec {
    optimal: f64,
    range: f64, // Default: 0
    max_range: f64, // Default: range * 1.5 // Default value taken from Factorio base mod source code, version 1.1.37, decoratives.lua, lines 11-17
    top_property_limit: f64, // Default: f64::MAX // Seems to be unused
}

#[derive(Debug)]
pub enum NoiseExpression {
    Variable(String), // variable_name
    FunctionApplication(String, String), // function_name and arguments //  FIXME // This does not actually satisfy the api, because arguments make my brain explode
    LiteralBoolean(bool), // literal_value
    LiteralNumber(f32), // literal_value
    LiteralString(String), // literal_value
    LiteralObject(String), // FIXME // I'm not going to implement this properly.
    LiteralExpression(Box<NoiseExpression>), // literal_value // oh god no
    ArrayConstruction(Vec<Box<NoiseExpression>>), // FIXME // Not implemented properly
    ProcedureDelimeter(Box<NoiseExpression>), // expression
    IfElseChain(String), // FIXME // no
}

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

#[derive(Debug)]
pub enum ProductPrototype {
    Item(ItemProductPrototype), // type = "item" // Default
    Fluid(FluidProductPrototype) // type = "fluid"
}

// Either a sequence or a table
// first item stands for name and second for amount
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

#[derive(Debug)]
pub struct RadiusVisualizationSpecification {
    sprite: Option<Sprite>,
    distance: f64, // Default: 0 // Must be > 0
    offset: Option<Factorio2DVector>,
    draw_in_cursor: bool, // Default: true
    draw_on_selection: bool // Default: true
}

#[derive(Debug)]
pub struct ItemToPlace {
    item: String, // Name of Item
    count: u32 // Can't be larger than the stack size of the item
}

#[derive(Debug)]
pub struct WaterReflectionDefinition {
    pictures: Option<Vec<SpriteVariation>>,
    orientation_to_variation: bool, // default: false
    rotate: bool, // Default: false
}
