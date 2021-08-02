use std::collections::HashMap;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use std::str::FromStr;
use std::fmt;
use crate::prototypes::PrototypesErr;

pub type FileName = String;
pub type ItemStackIndex = u16;
pub type Factorio2DVector = (f64, f64);
pub type AnimationFrameSequence = Vec<u16>;
pub type SpriteSize = (i16, i16); // sidth, then height
pub type SpritePosition = (i16, i16);
pub type KeySequence = String; // Parser and checker maybe?
pub type BoundingBox = (Position, Position); // Consider adding Option<f32> as specified in https://wiki.factorio.com/Types/BoundingBox?
                                             // It's kinda undocumented

#[derive(Debug)]
pub struct Position(i32, i32);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

#[derive(Debug)]
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CollisionMask(u8);

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
