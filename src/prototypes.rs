use std::collections::HashMap;
use crate::concepts::LocalisedString;
use thiserror::Error;
use std::str::FromStr;
use std::fmt;
use factorio_lib_rs_derive::{ModSetting, PrototypeBase};
use crate::types::{
    ModSettingType,
    MapDifficultySettings,
    MapPathFinder,
    MapUnitGroup,
    MapEnemyExpansion,
    MapEnemyEvolution,
    MapSteering,
    MapPollutionSettings,
    MapGenPreset,
    Color,
    ItemStackIndex,
    AnimationType,
    Sound,
    MouseCursorType,
    Sprite,
    IconSpecification,
    Energy,
    ProductType,
    ResearchTarget};

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
    fn name(&self) -> &String;
}
pub trait ModSetting: Prototype {
    fn localised_name(&self) -> &Option<LocalisedString>;
    fn localised_description(&self) -> &Option<LocalisedString>;
    fn order(&self) -> &Option<String>;
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
    fn name(&self) -> &String { &self.name }
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
    fn name(&self) -> &String { &self.name }
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
    fn name(&self) -> &String { &self.name }
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
    fn name(&self) -> &String { &self.name }
}

impl StringModSetting<'_> {
    pub fn default_value(&self) -> String { self.default_value.clone() }
    pub fn allow_blank(&self) -> Option<bool> { self.allow_blank }
    pub fn auto_trim(&self) -> Option<bool> {self.auto_trim }
    pub fn allowed_values(&self) -> Option<Vec<String>> { self.allowed_values.clone() }
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
    fn name(&self) -> &String { &self.name }
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
    fn name(&self) -> &String { &self.name }
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
    fn name(&self) -> &String { &self.name }
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
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct MapGenPresets {
    name: String,
    presets: HashMap<String, MapGenPreset>
}

impl Prototype for MapGenPresets {
    fn r#type(&self) -> PrototypeType { PrototypeType::MapGenPresets }
    fn name(&self) -> &String { &self.name }
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
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct MouseCursor {
    name: String,
    cursor: MouseCursorType
}

impl Prototype for MouseCursor {
    fn r#type(&self) -> PrototypeType { PrototypeType::MouseCursor }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct SoundPrototype {
    name: String,
    sound: Sound
}

impl Prototype for SoundPrototype {
    fn r#type(&self) -> PrototypeType { PrototypeType::Sound }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct SpectatorController {
    name: String, // Must be "default"
    movement_speed: f64 // Must be >= 0.34375
}

impl Prototype for SpectatorController {
    fn r#type(&self) -> PrototypeType { PrototypeType::SpectatorController }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct SpritePrototype {
    name: String,
    sprite: Sprite
}

impl Prototype for SpritePrototype {
    fn r#type(&self) -> PrototypeType { PrototypeType::Sprite }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct TileEffect {
    name: String, // Must be "water" // For some reason
    specular_lightness: Color,
    foam_color: Color,
    foam_color_multiplier: f32,
    tick_scale: f32,
    animation_speed: f32,
    animation_scale: Vec<f32>, // One or two members, same for other below
    dark_threshold: Vec<f32>,
    reflection_threshold: Vec<f32>,
    specular_threshold: Vec<f32>,
    texture: Sprite,
    near_zoom: f32, // Default: 2.0
    far_zoom: f32 // Default: 0.5
}

impl Prototype for TileEffect {
    fn r#type(&self) -> PrototypeType { PrototypeType::TileEffect }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct TipsAndTricksItemCategory {
    name: String,
    order: String
}

impl Prototype for TipsAndTricksItemCategory {
    fn r#type(&self) -> PrototypeType { PrototypeType::TipsAndTricksItemCategory }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct TriggerTargetType {
    name: String
}

impl Prototype for TriggerTargetType {
    fn r#type(&self) -> PrototypeType { PrototypeType::TriggerTargetType }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug)]
pub struct WindSound {
    name: String,
    sound: Sound
}

impl Prototype for WindSound {
    fn r#type(&self) -> PrototypeType { PrototypeType::WindSound }
    fn name(&self) -> &String { &self.name }
}

// PrototypeBase starts here

trait PrototypeBase: Prototype {
    fn localised_description(&self) -> &Option<LocalisedString>;
    fn localised_name(&self) -> &Option<LocalisedString>;
    fn order(&self) -> &String; // Default: ""
}

#[derive(Debug)]
pub struct AchievementBase {
    icon: IconSpecification,
    steam_stats_name: String, // Default: "" // Unusable
    allowed_without_fight: bool, // Default: true
    hidden: bool // Default: false
}

#[derive(Debug, PrototypeBase)]
pub struct Achievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase
}

impl Prototype for Achievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::Achievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct BuildEntityAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    to_build: String,
    amount: u32, // Default: 1
    limited_to_one_game: bool, // Default: false
    until_second: u32 // Default: 0 (means infinite)
}

impl Prototype for BuildEntityAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::BuildEntityAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct CombatRobotCountAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    count: u32 // Default: 1
}

impl Prototype for CombatRobotCountAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::CombatRobotCountAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct ConstructWithRobotsAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    limited_to_one_game: bool,
    amount: u32, // Default: 0
    more_than_manually: bool // Default: false
}

impl Prototype for ConstructWithRobotsAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::ConstructWithRobotsAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct DeconstructWithRobotsAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    amount: u32
}

impl Prototype for DeconstructWithRobotsAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::DeconstructWithRobotsAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct DeliverByRobotsAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    amount: f64
}

impl Prototype for DeliverByRobotsAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::DeliverByRobotsAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct DontBuildEntityAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    dont_buid: Vec<String>, // String is converted to Vec<String> with one element
    amount: u32 // Default: 0
}

impl Prototype for DontBuildEntityAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::DontBuildEntityAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct DontCraftManuallyAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    amount: f64
}

impl Prototype for DontCraftManuallyAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::DontCraftManuallyAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct DontUseEntityInEnergyProductionAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    excluded: Vec<String>, // String is converted to Vec<String> with one element
    included: Vec<String>, // Same as `excluded`
    last_hour_only: bool, // Default: false
    minimum_energy_produced: Energy // Default: 0W
}

impl Prototype for DontUseEntityInEnergyProductionAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::DontUseEntityInEnergyProductionAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct FinishTheGameAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    until_second: u32 // Default: 0 (means infinite)
}

impl Prototype for FinishTheGameAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::FinishTheGameAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct GroupAttackAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    amount: u32 // Default: 1
}

impl Prototype for GroupAttackAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::GroupAttackAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct KillAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    to_kill: String, // Default: ""
    type_to_kill: Option<PrototypeType>,
    damage_type: String, // damage type
    amount: u32, // Default: 1
    in_vehicle: bool, // Default: false
    personally: bool // Default: false
}

impl Prototype for KillAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::KillAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct PlayerDamagedAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    minimum_damage: f32,
    should_survive: bool,
    type_of_dealer: Option<PrototypeType>
}

impl Prototype for PlayerDamagedAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::PlayerDamagedAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct ProduceAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    amount: f64,
    limited_to_one_game: bool,
    product: ProductType // Type is determined from item_product or fluid_product // Only one can be set!
}

impl Prototype for ProduceAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::ProduceAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct ProducePerHourAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    amount: f64,
    product: ProductType
}

impl Prototype for ProducePerHourAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::ProducePerHourAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct ResearchAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    target: ResearchTarget // Determined from either `technology` or `research_all` is set
}

impl Prototype for ResearchAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::ResearchAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct TrainPathAchievement<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    achievement: AchievementBase,
    minimum_distance: f64
}

impl Prototype for TrainPathAchievement<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::TrainPathAchievement }
    fn name(&self) -> &String { &self.name }
}

#[derive(Debug, PrototypeBase)]
pub struct AmmoCategory<'a> {
    name: String,
    localised_description: Option<LocalisedString<'a>>,
    localised_name: Option<LocalisedString<'a>>,
    order: String,
    bonus_gui_order: String // Default: ""
}

impl Prototype for AmmoCategory<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::AmmoCategory }
    fn name(&self) -> &String { &self.name }
}

// Enum for all prototype types
#[derive(Debug, Clone, Copy)]
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
    #[error("Invalid BlendMode string: {0}")]
    InvalidBlendModeStr(String),
    #[error("Invalid RunMode string: {0}")]
    InvalidRunModeStr(String),
    #[error("Invalid SystemCursor string: {0}")]
    InvalidSystemCursorStr(String),
    #[error("Invalid Energy string: {0}")]
    InvalidEnergyStr(String),
}
