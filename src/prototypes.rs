use std::collections::HashMap;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use crate::concepts::LocalisedString;
use thiserror::Error;
use std::str::FromStr;
use std::fmt;
use factorio_lib_rs_derive::ModSetting;

// Struct representing global `data` table in lua environment
#[derive(Debug)]
pub struct DataTable {
    prototypes: Vec<Box<dyn Prototype>>
}

// Factorio prototypes
// Source info:
// For prototypes: https://wiki.factorio.com/Prototype_definitions
// For settings: https://wiki.factorio.com/Tutorial:Mod_settings

// TODO: replace optional properties that have default values with non-optional
// TODO: Some prototypes/types have different configuration variations that are documented with
// attribute priority or incompatiblity. This can be done with enums.

// Prototype
// Contains all values (accessors) for every prototype in the game
pub trait Prototype: fmt::Debug {
    fn r#type(&self) -> PrototypeType;
    fn name(&self) -> String;
}

#[derive(Debug, Clone)]
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

pub trait ModSetting: Prototype {
    fn localised_name(&self) -> Option<LocalisedString>;
    fn localised_description(&self) -> Option<LocalisedString>;
    fn order(&self) -> Option<String>;
    fn hidden(&self) -> bool; // Default: false
    fn setting_type(&self) -> ModSettingType;
}

#[derive(Debug, ModSetting)]
pub struct BoolModSetting<'a> {
    name: String,
    localised_name: Option<LocalisedString<'a>>,
    localised_description: Option<LocalisedString<'a>>,
    order: Option<String>,
    hidden: bool,
    setting_type: ModSettingType,
    default_value: bool,
    forced_value: Option<bool>,
}

impl Prototype for BoolModSetting<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::BoolSetting }
    fn name(&self) -> String { self.name.clone() }
}

impl BoolModSetting<'_> {
    pub fn default_value(&self) -> bool { self.default_value }
    pub fn forced_value(&self) -> Option<bool> { self.forced_value }
}

#[derive(Debug, ModSetting)]
pub struct IntModSetting<'a> {
    name: String,
    localised_name: Option<LocalisedString<'a>>,
    localised_description: Option<LocalisedString<'a>>,
    order: Option<String>,
    hidden: bool,
    setting_type: ModSettingType,
    default_value: i64,
    minimum_value: Option<i64>,
    maximum_value: Option<i64>,
    allowed_values: Option<Vec<i64>>,
}

impl Prototype for IntModSetting<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::IntSetting }
    fn name(&self) -> String { self.name.clone() }
}

impl IntModSetting<'_> {
    pub fn default_value(&self) -> i64 { self.default_value }
    pub fn minimum_value(&self) -> Option<i64> { self.minimum_value }
    pub fn maximum_value(&self) -> Option<i64> { self.maximum_value }
    pub fn allowed_values(&self) -> Option<Vec<i64>> { self.allowed_values.clone() }
}

#[derive(Debug, ModSetting)]
pub struct DoubleModSetting<'a> {
    name: String,
    localised_name: Option<LocalisedString<'a>>,
    localised_description: Option<LocalisedString<'a>>,
    order: Option<String>,
    hidden: bool,
    setting_type: ModSettingType,
    default_value: f64,
    minimum_value: Option<f64>,
    maximum_value: Option<f64>,
    allowed_values: Option<Vec<f64>>,
}

impl Prototype for DoubleModSetting<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::DoubleSetting }
    fn name(&self) -> String { self.name.clone() }
}

impl DoubleModSetting<'_> {
    pub fn default_value(&self) -> f64 { self.default_value }
    pub fn minimum_value(&self) -> Option<f64> { self.minimum_value }
    pub fn maximum_value(&self) -> Option<f64> { self.maximum_value }
    pub fn allowed_values(&self) -> Option<Vec<f64>> { self.allowed_values.clone() }
}

#[derive(Debug, ModSetting)]
pub struct StringModSetting<'a> {
    name: String,
    localised_name: Option<LocalisedString<'a>>,
    localised_description: Option<LocalisedString<'a>>,
    order: Option<String>,
    hidden: bool,
    setting_type: ModSettingType,
    default_value: String,
    allow_blank: Option<bool>,
    auto_trim: Option<bool>,
    allowed_values: Option<Vec<String>>
}

impl Prototype for StringModSetting<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::StringSetting }
    fn name(&self) -> String { self.name.clone() }
}

impl StringModSetting<'_> {
    pub fn default_value(&self) -> String { self.default_value.clone() }
    pub fn allow_blank(&self) -> Option<bool> { self.allow_blank }
    pub fn auto_trim(&self) -> Option<bool> {self.auto_trim }
    pub fn allowed_values(&self) -> Option<Vec<String>> { self.allowed_values.clone() }
}

pub type FileName = String;

// https://wiki.factorio.com/Types/Sound
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
pub struct AmbientSoundPrototype {
    name: String,
    sound: Sound,
    track_type: String,
    weight: Option<f64>
}

impl Prototype for AmbientSoundPrototype {
    fn r#type(&self) -> PrototypeType { PrototypeType::AmbientSound }
    fn name(&self) -> String { self.name.clone() }
}

impl AmbientSoundPrototype {
    pub fn sound(&self) -> &Sound { &self.sound }
    pub fn track_type(&self) -> String { self.track_type.clone() }
    pub fn weight(&self) -> Option<f64> { self.weight }
}

#[derive(Debug)]
pub struct AnimationPrototype {
    name: String,
    layers: Vec<AnimationType> // If lua table doesn;t have layers, use same table for constructing just one
}

#[derive(Debug)]
pub enum AnimationType {
    Layers(Vec<AnimationType>),
    Animation(Animation)
}

pub type Factorio2DVector = (f64, f64);

#[derive(Debug)]
pub enum AnimationDrawAs {
    DrawAsShadow,
    DrawAsGlow,
    DrawAsLight
}

impl AnimationDrawAs {
    pub fn new(draw_as_shadow: bool, draw_as_glow: bool, draw_as_light: bool) -> Option<Self> {
        if draw_as_shadow {
            Some(Self::DrawAsShadow)
        } else if draw_as_glow {
            Some(Self::DrawAsGlow)
        } else if draw_as_light {
            Some(Self::DrawAsLight)
        } else {
            None
        }
    }
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
pub struct Animation {
    hr_version: Option<HRAnimation>,
    filename: Option<FileName>, // Mandatory if "stripes" is not specified
    priority: Option<SpritePriority>, // Dfeault Medium
    flags: Option<SpriteFlags>,
    size: Option<SpriteSize>,
    // Automatically converted to size
    // width
    // height
    position: Option<SpritePosition>,
    // Automatically converted to position
    // x
    // y
    shift: Option<Factorio2DVector>,
    scale: Option<f64>,
    draw_as: Option<AnimationDrawAs>, // Aggregates draw_as_* attributes
    mipmap_count: Option<u8>, // Loaded if this is an icon
    apply_runtime_tint: Option<bool>, // false by default
    tint: Option<Color>,
    blend_mode: Option<BlendMode>, // Default is "normal"
    load_in_minimal_mode: Option<bool>, // Default: false
    premul_alpha: bool, // Default: true
    generate_sdf: bool, // Unused, Default: false
    run_mode: RunMode, // Default: "forward"
    frame_count: u32, // Default: 1, can't be 0
    line_length: u32, // Default: 0
    animation_speed: f32, // Default: 1.0
    max_advance: f32, // Default: MAX_FLOAT
    repeat_count: u8, // Default: 1, can't be 0
    // What are these???
    dice: Option<u8>,
    dice_x: Option<u8>,
    dice_y: Option<u8>,
    frame_sequence: Option<AnimationFrameSequence>,
    stripes: Option<Vec<Stripe>>
}

#[derive(Debug)]
pub struct HRAnimation {
    filename: Option<FileName>, // Mandatory if "stripes" is not specified
    priority: Option<SpritePriority>, // Dfeault Medium
    flags: Option<SpriteFlags>,
    size: Option<SpriteSize>,
    // Automatically converted to size
    // width
    // height
    position: Option<SpritePosition>,
    // Automatically converted to position
    // x
    // y
    shift: Option<Factorio2DVector>,
    scale: Option<f64>,
    draw_as: Option<AnimationDrawAs>, // Aggregates draw_as_* attributes
    mipmap_count: Option<u8>, // Loaded if this is an icon
    apply_runtime_tint: Option<bool>, // false by default
    tint: Option<Color>,
    blend_mode: Option<BlendMode>, // Default is "normal"
    load_in_minimal_mode: Option<bool>, // Default: false
    premul_alpha: bool, // Default: true
    generate_sdf: bool, // Unused, Default: false
    run_mode: RunMode, // Default: "forward"
    frame_count: u32, // Default: 1, can't be 0
    line_length: u32, // Default: 0
    animation_speed: f32, // Default: 1.0
    max_advance: f32, // Default: MAX_FLOAT
    repeat_count: u8, // Default: 1, can't be 0
    // What are these???
    dice: Option<u8>,
    dice_x: Option<u8>,
    dice_y: Option<u8>,
    frame_sequence: Option<AnimationFrameSequence>,
    stripes: Option<Vec<Stripe>>
}

pub type AnimationFrameSequence = Vec<u16>;
pub type SpriteSize = (i16, i16);
pub type SpritePosition = (i16, i16);

// TODO: fromstr
#[derive(Debug)]
pub enum BlendMode {
    Normal,
    Additive,
    AdditiveSoft,
    Multiplicative,
    Overwrite
}

// TODO: fromstr
#[derive(Debug)]
pub enum RunMode {
    Forward,
    Backward,
    ForwardThenBackward
}

#[derive(Debug)]
pub struct Stripe {
    width_in_frames: u32,
    height_in_frames: u32,
    filename: FileName,
    x: Option<u32>,
    y: Option<u32>
}

#[derive(Debug)]
pub struct Color(f32, f32, f32, f32);

impl Color {
    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(r, g, b, a)
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self { // r, g, b default is 0
        Self(r, g, b, 1.0)
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

pub type ItemStackIndex = u16;

#[derive(Debug)]
pub struct EditorController {
    name: String, // Must be "default"
    inventory_size: ItemStackIndex,
    gun_inventory_size: ItemStackIndex,
    movement_speed: f64, // Must be >= 0.34375
    item_pickup_distance: f64,
    loot_pickup_distance: f64,
    mining_speed: f64,
    enable_flash_light: bool,
    adjust_speed_based_off_zoom: bool,
    render_as_day: bool,
    instant_blueprint_building: bool,
    instant_deconstruction: bool,
    instant_upgrading: bool,
    instant_rail_planner: bool,
    show_status_icons: bool,
    show_hidden_entities: bool,
    show_entity_tags: bool,
    show_entity_health_bars: bool,
    show_additional_entity_info_gui: bool,
    generate_neighbour_chunks: bool,
    fill_built_entity_energy_buffers: bool,
    show_character_tab_in_controller_gui: bool,
    show_infinity_filter_in_controller_gui: bool,
    placed_corpses_never_expire: bool
}

impl Prototype for EditorController {
    fn r#type(&self) -> PrototypeType { PrototypeType::EditorController }
    fn name(&self) -> String { self.name.clone() }
}

#[derive(Debug)]
pub struct Font {
    name: String,
    size: i32,
    from: String,
    spacing: f32, // Default 0.0
    border: bool, // Default fase
    filtered: bool, // Default false
    border_color: Option<Color>
}

impl Prototype for Font {
    fn r#type(&self) -> PrototypeType { PrototypeType::Font }
    fn name(&self) -> String { self.name.clone() }
}

#[derive(Debug)]
pub struct GodController {
    name: String, // Must be "default"
    inventory_size: ItemStackIndex,
    movement_speed: f64, // Must be >= 0.34375
    item_pickup_distance: f64,
    loot_pickup_distance: f64,
    mining_speed: f64,
    crafting_categories: Option<Vec<String>>,
    mining_categories: Option<Vec<String>>
}

impl Prototype for GodController {
    fn r#type(&self) -> PrototypeType { PrototypeType::GodController }
    fn name(&self) -> String { self.name.clone() }
}

#[derive(Debug)]
pub struct MapGenPresets {
    name: String,
    presets: HashMap<String, MapGenPreset>
}

impl Prototype for MapGenPresets {
    fn r#type(&self) -> PrototypeType { PrototypeType::MapGenPresets }
    fn name(&self) -> String { self.name.clone() }
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
    // Shoukd these be optional or just have defaults? TODO
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
            _ => Err(PrototypesErr::InvalidMapGenSizeStr(String::from(s)))
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
pub struct Position(i32, i32);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
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
    // autoplace_settings // TODO: UNDOCUMENTED // "Types/table", reffuses to elaborate further
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
            _ => Err(PrototypesErr::InvalidDifficultySettingStr(String::from(s)))
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
            _ => Err(PrototypesErr::InvalidResearchQueueSettingStr(String::from(s)))
        }
    }
}

#[derive(Debug)]
pub struct MapSettings {
    name: String, // Must be "map-settings"
    pollution: MapPollutionSettings,
    steering: MapSteering, // ???
    enemy_evolution: MapEnemyEvolution,
    enemy_expansion: MapEnemyExpansion,
    unit_group: MapUnitGroup,
    path_finder: MapPathFinder,
    max_ffailed_behavior_count: u32,
    difficulty_settings: MapDifficultySettings
}

impl Prototype for MapSettings {
    fn r#type(&self) -> PrototypeType { PrototypeType::MapSettings }
    fn name(&self) -> String { self.name.clone() }
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

// Enum for all prototype types
#[derive(Debug)]
pub enum PrototypeType {
    // General prototypes
    AmbientSound,
    Animation,
    EditorController,
    Font,
    GodController,
    MapGenPresets,
    MapSettings,
    MouseCursor,
    Sound,
    SpectatorController,
    Sprite,
    TileEffect,
    TipsAndTricksItemCategory,
    TriggerTargetType,
    WindSound,
    Achievement,
    BuildEntityAchievement,
    CombatRobotCountAchievement,
    ConstructWithRobotsAchievement,
    DeconstructWithRobotsAchievement,
    DeliverByRobotsAchievement,
    DontBuildEntityAchievement,
    DontCraftManuallyAchievement,
    DontUseEntityInEnergyProductionAchievement,
    FinishTheGameAchievement,
    GroupAttackAchievement,
    KillAchievement,
    PlayerDamagedAchievement,
    ProduceAchievement,
    ProducePerHourAchievement,
    ResearchAchievement,
    TrainPathAchievement,
    AmmoCategory,
    AutoplaceControl,
    CustomInput,
    DamageType,
    Decorative,
    Arrow,
    ArtilleryFlare,
    ArtilleryProjectile,
    Beam,
    CharacterCorpse,
    Cliff,
    Corpse,
    RailRemnants,
    DecorativeTileProxy,
    EntityGhost,
    EntityParticle,
    LeafParticle,
    Accumulator,
    ArtilleryTurret,
    Beacon,
    Boiler,
    BurnerGenerator,
    Character,
    ArithmeticCombinator,
    DeciderCombinator,
    ConstantCombinator,
    Container,
    LogisticContainer,
    InfinityContainer,
    AssemblingMachine,
    RocketSilo,
    Furnace,
    ElectricEnergyInterface,
    ElectricPole,
    EnemySpawner,
    Fish,
    CombatRobot,
    ConstructionRobot,
    Gate,
    Generator,
    HeatInterface,
    HeatPipe,
    Inserter,
    Lab,
    Lamp,
    LandMine,
    LinkedContainer,
    Market,
    MiningDrill,
    OffshorePump,
    Pipe,
    InfinityPipe,
    PipeToGround,
    PlayerPort,
    PowerSwitch,
    ProgrammableSpeaker,
    Pump,
    Radar,
    CurvedRail,
    StraightRail,
    RailChainSignal,
    RailSignal,
    Reactor,
    Roboport,
    SimpleEntity,
    SimpleEntityWithOwner,
    SimpleEntityWithForce,
    SolarPanel,
    SpiderLeg,
    StorageTank,
    TrainStop,
    LinkedBelt,
    Loader1x1,
    Loader1x2,
    Splitter,
    TransportBelt,
    UndergroundBelt,
    Tree,
    Turret,
    AmmoTurret,
    ElectricTurret,
    FluidTurret,
    Unit,
    Car,
    ArtilleryWagon,
    CargoWagon,
    FluidWagon,
    Locomotive,
    SpiderVehicle,
    Wall,
    Explosion,
    FlameThrowerExplosion,
    FireFlame,
    FluidStream,
    Flyingtext,
    HighlightBoxEntity,
    ItemEntity,
    ItemRequestProxy,
    ParticleSource,
    Projectile,
    ResourceEntity,
    RocketSiloRocket,
    RocketSiloRocketShadow,
    SimpleSmoke, // note: for migration, cannot be used.
    SmokeWithTrigger,
    SpeechBubble,
    Sticker,
    TileGhost,
    ActiveDefenseEquipment,
    BatteryEquipment,
    BeltImmunityEquipment,
    EnergyShieldEquipment,
    GeneratorEquipment,
    MovementBonusEquipment,
    NightVisionEquipment,
    RoboportEquipment,
    SolarPanelEquipment,
    EquipmentCategory,
    EquipmentGrid,
    Fluid,
    FuelCategory,
    GuiStyle,
    Item,
    AmmoItem,
    Capsule,
    Gun,
    ItemWithEntityData,
    ItemWithLabel,
    ItemWithInventory,
    BlueprintBook,
    ItemWithTags,
    SelectionTool,
    BlueprintItem,
    CopyPasteTool,
    DeconstructionItem,
    UpgradeItem,
    Module,
    RailPlanner,
    SpidertronRemote,
    Tool,
    Armor,
    MiningTool, // note: for migration, cannot be used.
    RepairTool,
    ItemGroup,
    ItemSubGroup,
    ModuleCategory,
    NamedNoiseExpression,
    NoiseLayer,
    Particle,
    Recipe,
    RecipeCategory,
    ResourceCategory,
    Shortcut,
    Technology,
    Tile,
    TipsAndTricksItem,
    TrivialSmoke,
    Tutorial,
    UnilityConstants,
    UtilitySounds,
    UtilitySprites,
    VirtualSignal,
    // Setting types
    BoolSetting,
    IntSetting,
    DoubleSetting,
    StringSetting
}

impl fmt::Display for PrototypeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match &self {
            PrototypeType::AmbientSound => "ambient-sound",
            PrototypeType::Animation    => "animation",
            PrototypeType::EditorController => "editor-controller",
            PrototypeType::Font => "font",
            PrototypeType::GodController => "god-controller",
            PrototypeType::MapGenPresets => "map-gen-settings",
            PrototypeType::MapSettings => "map-settings",
            PrototypeType::MouseCursor => "mouse-cursor",
            PrototypeType::Sound => "sound",
            PrototypeType::SpectatorController => "spectator-controller",
            PrototypeType::Sprite => "sprite",
            PrototypeType::TileEffect => "tile-effect",
            PrototypeType::TipsAndTricksItemCategory => "tips-and-tricks-item-category",
            PrototypeType::TriggerTargetType => "trigger-target-type",
            PrototypeType::WindSound => "wind-sound",
            PrototypeType::Achievement => "achievement",
            PrototypeType::BuildEntityAchievement => "build-entity-achievement",
            PrototypeType::CombatRobotCountAchievement => "combat-robot-count",
            PrototypeType::ConstructWithRobotsAchievement => "construct-with-robots-achevement",
            PrototypeType::DeconstructWithRobotsAchievement => "deconstruct-with-robots-achievement",
            PrototypeType::DeliverByRobotsAchievement => "deliver-by-robots-achievement",
            PrototypeType::DontBuildEntityAchievement => "dont-build-entity-achievement",
            PrototypeType::DontCraftManuallyAchievement => "dont-craft-manually-achievement",
            PrototypeType::DontUseEntityInEnergyProductionAchievement => "dont-use-entity-in-energy-production-achievement",
            PrototypeType::FinishTheGameAchievement => "finish-the-game-achievement",
            PrototypeType::GroupAttackAchievement => "group-attack-achievement",
            PrototypeType::KillAchievement => "kill-achievement",
            PrototypeType::PlayerDamagedAchievement => "player-damaged-achievement",
            PrototypeType::ProduceAchievement => "produce-achievement",
            PrototypeType::ProducePerHourAchievement => "produce-per-hour-achievement",
            PrototypeType::ResearchAchievement => "research-achievement",
            PrototypeType::TrainPathAchievement => "train-path-achievement",
            PrototypeType::AmmoCategory => "ammo-category",
            PrototypeType::AutoplaceControl => "autoplace-control",
            PrototypeType::CustomInput => "custom-input",
            PrototypeType::DamageType => "damage-type",
            PrototypeType::Decorative => "optimized-decorative",
            PrototypeType::Arrow => "arrow",
            PrototypeType::ArtilleryFlare => "artillery-flare",
            PrototypeType::ArtilleryProjectile => "artillery-projectile",
            PrototypeType::Beam => "beam",
            PrototypeType::CharacterCorpse => "character-corpse",
            PrototypeType::Cliff => "cliff",
            PrototypeType::Corpse => "corpse",
            PrototypeType::RailRemnants => "rail-remnants",
            PrototypeType::DecorativeTileProxy => "deconstructible-tile-proxy",
            PrototypeType::EntityGhost => "entity-ghost",
            PrototypeType::EntityParticle => "particle",
            PrototypeType::LeafParticle => "leaf-particle",
            PrototypeType::Accumulator => "accumulator",
            PrototypeType::ArtilleryTurret => "artillery-turret",
            PrototypeType::Beacon => "beacon",
            PrototypeType::Boiler => "boiler",
            PrototypeType::BurnerGenerator => "burner-generator",
            PrototypeType::Character => "character",
            PrototypeType::ArithmeticCombinator => "arithmetic-combinator",
            PrototypeType::DeciderCombinator => "decider-combinator",
            PrototypeType::ConstantCombinator => "constant-combinator",
            PrototypeType::Container => "container",
            PrototypeType::LogisticContainer => "logistic-container",
            PrototypeType::InfinityContainer => "infinity-container",
            PrototypeType::AssemblingMachine => "assembling-machine",
            PrototypeType::RocketSilo => "rocket-silo",
            PrototypeType::Furnace => "furnace",
            PrototypeType::ElectricEnergyInterface => "electric-energy-interface",
            PrototypeType::ElectricPole => "electric-pole",
            PrototypeType::EnemySpawner => "unit-spawner",
            PrototypeType::Fish => "fish",
            PrototypeType::CombatRobot => "combat-robot",
            PrototypeType::ConstructionRobot => "construction-robot",
            PrototypeType::Gate => "gate",
            PrototypeType::Generator => "generator",
            PrototypeType::HeatInterface => "heat-interface",
            PrototypeType::HeatPipe => "heat-pipe",
            PrototypeType::Inserter => "inserter",
            PrototypeType::Lab => "lab",
            PrototypeType::Lamp => "lamp",
            PrototypeType::LandMine => "land-mine",
            PrototypeType::LinkedContainer => "linked-container",
            PrototypeType::Market => "market",
            PrototypeType::MiningDrill => "mining-drill",
            PrototypeType::OffshorePump => "offshore-pump",
            PrototypeType::Pipe => "pipe",
            PrototypeType::InfinityPipe => "infinity-pipe",
            PrototypeType::PipeToGround => "pipe-to-ground",
            PrototypeType::PlayerPort => "player-port",
            PrototypeType::PowerSwitch => "power-switch",
            PrototypeType::ProgrammableSpeaker => "programmable-speaker",
            PrototypeType::Pump => "pump",
            PrototypeType::Radar => "radar",
            PrototypeType::CurvedRail => "curved-rail",
            PrototypeType::StraightRail => "straight-rail",
            PrototypeType::RailChainSignal => "rail-chain-signal",
            PrototypeType::RailSignal => "rail-signal",
            PrototypeType::Reactor => "reactor",
            PrototypeType::Roboport => "roboport",
            PrototypeType::SimpleEntity => "simple-entity",
            PrototypeType::SimpleEntityWithOwner => "simple-entity-with-owner",
            PrototypeType::SimpleEntityWithForce => "simple-entity-with-force",
            PrototypeType::SolarPanel => "solar-panel",
            PrototypeType::SpiderLeg => "spider-leg",
            PrototypeType::StorageTank => "storage-tank",
            PrototypeType::TrainStop => "train-stop",
            PrototypeType::LinkedBelt => "linked-belt",
            PrototypeType::Loader1x1 => "loader-1x1",
            PrototypeType::Loader1x2 => "loader",
            PrototypeType::Splitter => "splitter",
            PrototypeType::TransportBelt => "transport-belt",
            PrototypeType::UndergroundBelt => "underground-belt",
            PrototypeType::Tree => "tree",
            PrototypeType::Turret => "turret",
            PrototypeType::AmmoTurret => "ammo-turret",
            PrototypeType::ElectricTurret => "electric-turret",
            PrototypeType::FluidTurret => "fluid-turret",
            PrototypeType::Unit => "unit",
            PrototypeType::Car => "car",
            PrototypeType::ArtilleryWagon => "artillery-wagon",
            PrototypeType::CargoWagon => "cargo-wagon",
            PrototypeType::FluidWagon => "fluid-wagon",
            PrototypeType::Locomotive => "locomotive",
            PrototypeType::SpiderVehicle => "spider-vehicle",
            PrototypeType::Wall => "wall",
            PrototypeType::Explosion => "explosion",
            PrototypeType::FlameThrowerExplosion => "flame-thrower-explosion",
            PrototypeType::FireFlame => "fire",
            PrototypeType::FluidStream => "stream",
            PrototypeType::Flyingtext => "flying-text",
            PrototypeType::HighlightBoxEntity => "higlight-box",
            PrototypeType::ItemEntity => "item-entity",
            PrototypeType::ItemRequestProxy => "item-request-proxy",
            PrototypeType::ParticleSource => "particle-source",
            PrototypeType::Projectile => "projectile",
            PrototypeType::ResourceEntity => "resource",
            PrototypeType::RocketSiloRocket => "rocket-silo-rocket",
            PrototypeType::RocketSiloRocketShadow => "rocket-silo-rocket-shadow",
            PrototypeType::SimpleSmoke => "smoke",
            PrototypeType::SmokeWithTrigger => "smoke-with-trigger",
            PrototypeType::SpeechBubble => "speech-bubble",
            PrototypeType::Sticker => "sticker",
            PrototypeType::TileGhost => "tile-ghost",
            PrototypeType::ActiveDefenseEquipment => "active-defense-equipment",
            PrototypeType::BatteryEquipment => "battery-equipment",
            PrototypeType::BeltImmunityEquipment => "belt-immunity-equipment",
            PrototypeType::EnergyShieldEquipment => "energy-shield-equipment",
            PrototypeType::GeneratorEquipment => "generator-equipment",
            PrototypeType::MovementBonusEquipment => "movement-bonus-equipment",
            PrototypeType::NightVisionEquipment => "night-vision-equipment",
            PrototypeType::RoboportEquipment => "roboport-equipment",
            PrototypeType::SolarPanelEquipment => "solar-panel-equipment",
            PrototypeType::EquipmentCategory => "equipment-category",
            PrototypeType::EquipmentGrid => "equipment-grid",
            PrototypeType::Fluid => "fluid",
            PrototypeType::FuelCategory => "fuel-category",
            PrototypeType::GuiStyle => "gui-style",
            PrototypeType::Item => "item",
            PrototypeType::AmmoItem => "ammo",
            PrototypeType::Capsule => "capsule",
            PrototypeType::Gun => "gun",
            PrototypeType::ItemWithEntityData => "item-with-entity-data",
            PrototypeType::ItemWithLabel => "entity-with-label",
            PrototypeType::ItemWithInventory => "item-with-inventory",
            PrototypeType::BlueprintBook => "blueprint-book",
            PrototypeType::ItemWithTags => "item-with-tags",
            PrototypeType::SelectionTool => "selection-tool",
            PrototypeType::BlueprintItem => "blueprint",
            PrototypeType::CopyPasteTool => "copy-paste-tool",
            PrototypeType::DeconstructionItem => "deconstruction-item",
            PrototypeType::UpgradeItem => "upgrade-item",
            PrototypeType::Module => "module",
            PrototypeType::RailPlanner => "rail-planner",
            PrototypeType::SpidertronRemote => "spidertron-remote",
            PrototypeType::Tool => "tool",
            PrototypeType::Armor => "armor",
            PrototypeType::MiningTool => "mining-tool",
            PrototypeType::RepairTool => "repair-tool",
            PrototypeType::ItemGroup => "item-group",
            PrototypeType::ItemSubGroup => "item-subgroup",
            PrototypeType::ModuleCategory => "module-category",
            PrototypeType::NamedNoiseExpression => "noise-expression",
            PrototypeType::NoiseLayer => "noise-layer",
            PrototypeType::Particle => "optimized-particle",
            PrototypeType::Recipe => "recipe",
            PrototypeType::RecipeCategory => "recipe-category",
            PrototypeType::ResourceCategory => "resource-category",
            PrototypeType::Shortcut => "shortcut",
            PrototypeType::Technology => "technology",
            PrototypeType::Tile => "tile",
            PrototypeType::TipsAndTricksItem => "tip-and-tricks-item",
            PrototypeType::TrivialSmoke => "trivial-smoke",
            PrototypeType::Tutorial => "tutorial",
            PrototypeType::UnilityConstants => "utility-constants",
            PrototypeType::UtilitySounds => "utility-sounds",
            PrototypeType::UtilitySprites => "utility-sprites",
            PrototypeType::VirtualSignal => "virtual-signal",
            PrototypeType::BoolSetting => "bool-setting",
            PrototypeType::IntSetting => "int-setting",
            PrototypeType::DoubleSetting => "double-setting",
            PrototypeType::StringSetting => "string-setting"
        })
    }
}

impl FromStr for PrototypeType {
    type Err = PrototypesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ambient-sound" => Ok(PrototypeType::AmbientSound),
            "animation" => Ok(PrototypeType::Animation),
            "editor-controller" => Ok(PrototypeType::EditorController),
            "font" => Ok(PrototypeType::Font),
            "god-controller" => Ok(PrototypeType::GodController),
            "map-gen-settings" => Ok(PrototypeType::MapGenPresets),
            "map-settings" => Ok(PrototypeType::MapSettings),
            "mouse-cursor" => Ok(PrototypeType::MouseCursor),
            "sound" => Ok(PrototypeType::Sound),
            "spectator-controller" => Ok(PrototypeType::SpectatorController),
            "sprite" => Ok(PrototypeType::Sprite),
            "tile-effect" => Ok(PrototypeType::TileEffect),
            "tips-and-tricks-item-category" => Ok(PrototypeType::TipsAndTricksItemCategory),
            "trigger-target-type" => Ok(PrototypeType::TriggerTargetType),
            "wind-sound" => Ok(PrototypeType::WindSound),
            "achievement" => Ok(PrototypeType::Achievement),
            "build-entity-achievement" => Ok(PrototypeType::BuildEntityAchievement),
            "combat-robot-count" => Ok(PrototypeType::CombatRobotCountAchievement),
            "construct-with-robots-achevement" => Ok(PrototypeType::ConstructWithRobotsAchievement),
            "deconstruct-with-robots-achievement" => Ok(PrototypeType::DeconstructWithRobotsAchievement),
            "deliver-by-robots-achievement" => Ok(PrototypeType::DeliverByRobotsAchievement),
            "dont-build-entity-achievement" => Ok(PrototypeType::DontBuildEntityAchievement),
            "dont-craft-manually-achievement" => Ok(PrototypeType::DontCraftManuallyAchievement),
            "dont-use-entity-in-energy-production-achievement" => Ok(PrototypeType::DontUseEntityInEnergyProductionAchievement),
            "finish-the-game-achievement" => Ok(PrototypeType::FinishTheGameAchievement),
            "group-attack-achievement" => Ok(PrototypeType::GroupAttackAchievement),
            "kill-achievement" => Ok(PrototypeType::KillAchievement),
            "player-damaged-achievement" => Ok(PrototypeType::PlayerDamagedAchievement),
            "produce-achievement" => Ok(PrototypeType::ProduceAchievement),
            "produce-per-hour-achievement" => Ok(PrototypeType::ProducePerHourAchievement),
            "research-achievement" => Ok(PrototypeType::ResearchAchievement),
            "train-path-achievement" => Ok(PrototypeType::TrainPathAchievement),
            "ammo-category" => Ok(PrototypeType::AmmoCategory),
            "autoplace-control" => Ok(PrototypeType::AutoplaceControl),
            "custom-input" => Ok(PrototypeType::CustomInput),
            "damage-type" => Ok(PrototypeType::DamageType),
            "optimized-decorative" => Ok(PrototypeType::Decorative),
            "arrow" => Ok(PrototypeType::Arrow),
            "artillery-flare" => Ok(PrototypeType::ArtilleryFlare),
            "artillery-projectile" => Ok(PrototypeType::ArtilleryProjectile),
            "beam" => Ok(PrototypeType::Beam),
            "character-corpse" => Ok(PrototypeType::CharacterCorpse),
            "cliff" => Ok(PrototypeType::Cliff),
            "corpse" => Ok(PrototypeType::Corpse),
            "rail-remnants" => Ok(PrototypeType::RailRemnants),
            "deconstructible-tile-proxy" => Ok(PrototypeType::DecorativeTileProxy),
            "entity-ghost" => Ok(PrototypeType::EntityGhost),
            "particle" => Ok(PrototypeType::EntityParticle),
            "leaf-particle" => Ok(PrototypeType::LeafParticle),
            "accumulator" => Ok(PrototypeType::Accumulator),
            "artillery-turret" => Ok(PrototypeType::ArtilleryTurret),
            "beacon" => Ok(PrototypeType::Beacon),
            "boiler" => Ok(PrototypeType::Boiler),
            "burner-generator" => Ok(PrototypeType::BurnerGenerator),
            "character" => Ok(PrototypeType::Character),
            "arithmetic-combinator" => Ok(PrototypeType::ArithmeticCombinator),
            "decider-combinator" => Ok(PrototypeType::DeciderCombinator),
            "constant-combinator" => Ok(PrototypeType::ConstantCombinator),
            "container" => Ok(PrototypeType::Container),
            "logistic-container" => Ok(PrototypeType::LogisticContainer),
            "infinity-container" => Ok(PrototypeType::InfinityContainer),
            "assembling-machine" => Ok(PrototypeType::AssemblingMachine),
            "rocket-silo" => Ok(PrototypeType::RocketSilo),
            "furnace" => Ok(PrototypeType::Furnace),
            "electric-energy-interface" => Ok(PrototypeType::ElectricEnergyInterface),
            "electric-pole" => Ok(PrototypeType::ElectricPole),
            "unit-spawner" => Ok(PrototypeType::EnemySpawner),
            "fish" => Ok(PrototypeType::Fish),
            "combat-robot" => Ok(PrototypeType::CombatRobot),
            "construction-robot" => Ok(PrototypeType::ConstructionRobot),
            "gate" => Ok(PrototypeType::Gate),
            "generator" => Ok(PrototypeType::Generator),
            "heat-interface" => Ok(PrototypeType::HeatInterface),
            "heat-pipe" => Ok(PrototypeType::HeatPipe),
            "inserter" => Ok(PrototypeType::Inserter),
            "lab" => Ok(PrototypeType::Lab),
            "lamp" => Ok(PrototypeType::Lamp),
            "land-mine" => Ok(PrototypeType::LandMine),
            "linked-container" => Ok(PrototypeType::LinkedContainer),
            "market" => Ok(PrototypeType::Market),
            "mining-drill" => Ok(PrototypeType::MiningDrill),
            "offshore-pump" => Ok(PrototypeType::OffshorePump),
            "pipe" => Ok(PrototypeType::Pipe),
            "infinity-pipe" => Ok(PrototypeType::InfinityPipe),
            "pipe-to-ground" => Ok(PrototypeType::PipeToGround),
            "player-port" => Ok(PrototypeType::PlayerPort),
            "power-switch" => Ok(PrototypeType::PowerSwitch),
            "programmable-speaker" => Ok(PrototypeType::ProgrammableSpeaker),
            "pump" => Ok(PrototypeType::Pump),
            "radar" => Ok(PrototypeType::Radar),
            "curved-rail" => Ok(PrototypeType::CurvedRail),
            "straight-rail" => Ok(PrototypeType::StraightRail),
            "rail-chain-signal" => Ok(PrototypeType::RailChainSignal),
            "rail-signal" => Ok(PrototypeType::RailSignal),
            "reactor" => Ok(PrototypeType::Reactor),
            "roboport" => Ok(PrototypeType::Roboport),
            "simple-entity" => Ok(PrototypeType::SimpleEntity),
            "simple-entity-with-owner" => Ok(PrototypeType::SimpleEntityWithOwner),
            "simple-entity-with-force" => Ok(PrototypeType::SimpleEntityWithForce),
            "solar-panel" => Ok(PrototypeType::SolarPanel),
            "spider-leg" => Ok(PrototypeType::SpiderLeg),
            "storage-tank" => Ok(PrototypeType::StorageTank),
            "train-stop" => Ok(PrototypeType::TrainStop),
            "linked-belt" => Ok(PrototypeType::LinkedBelt),
            "loader-1x1" => Ok(PrototypeType::Loader1x1),
            "loader" => Ok(PrototypeType::Loader1x2),
            "splitter" => Ok(PrototypeType::Splitter),
            "transport-belt" => Ok(PrototypeType::TransportBelt),
            "underground-belt" => Ok(PrototypeType::UndergroundBelt),
            "tree" => Ok(PrototypeType::Tree),
            "turret" => Ok(PrototypeType::Turret),
            "ammo-turret" => Ok(PrototypeType::AmmoTurret),
            "electric-turret" => Ok(PrototypeType::ElectricTurret),
            "fluid-turret" => Ok(PrototypeType::FluidTurret),
            "unit" => Ok(PrototypeType::Unit),
            "car" => Ok(PrototypeType::Car),
            "artillery-wagon" => Ok(PrototypeType::ArtilleryWagon),
            "cargo-wagon" => Ok(PrototypeType::CargoWagon),
            "fluid-wagon" => Ok(PrototypeType::FluidWagon),
            "locomotive" => Ok(PrototypeType::Locomotive),
            "spider-vehicle" => Ok(PrototypeType::SpiderVehicle),
            "wall" => Ok(PrototypeType::Wall),
            "explosion" => Ok(PrototypeType::Explosion),
            "flame-thrower-explosion" => Ok(PrototypeType::FlameThrowerExplosion),
            "fire" => Ok(PrototypeType::FireFlame),
            "stream" => Ok(PrototypeType::FluidStream),
            "flying-text" => Ok(PrototypeType::Flyingtext),
            "higlight-box" => Ok(PrototypeType::HighlightBoxEntity),
            "item-entity" => Ok(PrototypeType::ItemEntity),
            "item-request-proxy" => Ok(PrototypeType::ItemRequestProxy),
            "particle-source" => Ok(PrototypeType::ParticleSource),
            "projectile" => Ok(PrototypeType::Projectile),
            "resource" => Ok(PrototypeType::ResourceEntity),
            "rocket-silo-rocket" => Ok(PrototypeType::RocketSiloRocket),
            "rocket-silo-rocket-shadow" => Ok(PrototypeType::RocketSiloRocketShadow),
            "smoke" => Ok(PrototypeType::SimpleSmoke),
            "smoke-with-trigger" => Ok(PrototypeType::SmokeWithTrigger),
            "speech-bubble" => Ok(PrototypeType::SpeechBubble),
            "sticker" => Ok(PrototypeType::Sticker),
            "tile-ghost" => Ok(PrototypeType::TileGhost),
            "active-defense-equipment" => Ok(PrototypeType::ActiveDefenseEquipment),
            "battery-equipment" => Ok(PrototypeType::BatteryEquipment),
            "belt-immunity-equipment" => Ok(PrototypeType::BeltImmunityEquipment),
            "energy-shield-equipment" => Ok(PrototypeType::EnergyShieldEquipment),
            "generator-equipment" => Ok(PrototypeType::GeneratorEquipment),
            "movement-bonus-equipment" => Ok(PrototypeType::MovementBonusEquipment),
            "night-vision-equipment" => Ok(PrototypeType::NightVisionEquipment),
            "roboport-equipment" => Ok(PrototypeType::RoboportEquipment),
            "solar-panel-equipment" => Ok(PrototypeType::SolarPanelEquipment),
            "equipment-category" => Ok(PrototypeType::EquipmentCategory),
            "equipment-grid" => Ok(PrototypeType::EquipmentGrid),
            "fluid" => Ok(PrototypeType::Fluid),
            "fuel-category" => Ok(PrototypeType::FuelCategory),
            "gui-style" => Ok(PrototypeType::GuiStyle),
            "item" => Ok(PrototypeType::Item),
            "ammo" => Ok(PrototypeType::AmmoItem),
            "capsule" => Ok(PrototypeType::Capsule),
            "gun" => Ok(PrototypeType::Gun),
            "item-with-entity-data" => Ok(PrototypeType::ItemWithEntityData),
            "entity-with-label" => Ok(PrototypeType::ItemWithLabel),
            "item-with-inventory" => Ok(PrototypeType::ItemWithInventory),
            "blueprint-book" => Ok(PrototypeType::BlueprintBook),
            "item-with-tags" => Ok(PrototypeType::ItemWithTags),
            "selection-tool" => Ok(PrototypeType::SelectionTool),
            "blueprint" => Ok(PrototypeType::BlueprintItem),
            "copy-paste-tool" => Ok(PrototypeType::CopyPasteTool),
            "deconstruction-item" => Ok(PrototypeType::DeconstructionItem),
            "upgrade-item" => Ok(PrototypeType::UpgradeItem),
            "module" => Ok(PrototypeType::Module),
            "rail-planner" => Ok(PrototypeType::RailPlanner),
            "spidertron-remote" => Ok(PrototypeType::SpidertronRemote),
            "tool" => Ok(PrototypeType::Tool),
            "armor" => Ok(PrototypeType::Armor),
            "mining-tool" => Ok(PrototypeType::MiningTool),
            "repair-tool" => Ok(PrototypeType::RepairTool),
            "item-group" => Ok(PrototypeType::ItemGroup),
            "item-subgroup" => Ok(PrototypeType::ItemSubGroup),
            "module-category" => Ok(PrototypeType::ModuleCategory),
            "noise-expression" => Ok(PrototypeType::NamedNoiseExpression),
            "noise-layer" => Ok(PrototypeType::NoiseLayer),
            "optimized-particle" => Ok(PrototypeType::Particle),
            "recipe" => Ok(PrototypeType::Recipe),
            "recipe-category" => Ok(PrototypeType::RecipeCategory),
            "resource-category" => Ok(PrototypeType::ResourceCategory),
            "shortcut" => Ok(PrototypeType::Shortcut),
            "technology" => Ok(PrototypeType::Technology),
            "tile" => Ok(PrototypeType::Tile),
            "tip-and-tricks-item" => Ok(PrototypeType::TipsAndTricksItem),
            "trivial-smoke" => Ok(PrototypeType::TrivialSmoke),
            "tutorial" => Ok(PrototypeType::Tutorial),
            "utility-constants" => Ok(PrototypeType::UnilityConstants),
            "utility-sounds" => Ok(PrototypeType::UtilitySounds),
            "utility-sprites" => Ok(PrototypeType::UtilitySprites),
            "virtual-signal" => Ok(PrototypeType::VirtualSignal),
            "bool-setting" => Ok(PrototypeType::BoolSetting),
            "int-setting" => Ok(PrototypeType::IntSetting),
            "double-setting" => Ok(PrototypeType::DoubleSetting),
            "string-setting" => Ok(PrototypeType::StringSetting),
            _ => Err(PrototypesErr::InvalidPrototypeType(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum PrototypesErr {
    #[error("Invalid prototype type: {0}")]
    InvalidPrototypeType(String),
    #[error("Invalid mod setting type: {0}")]
    InvalidModSettingType(String),
    #[error("Invalid MapGenSize string: {0}")]
    InvalidMapGenSizeStr(String),
    #[error("Invalid DifficultySetting string: {0}")]
    InvalidDifficultySettingStr(String),
    #[error("Invalid ResearchQueueSetting string: {0}")]
    InvalidResearchQueueSettingStr(String),
}
