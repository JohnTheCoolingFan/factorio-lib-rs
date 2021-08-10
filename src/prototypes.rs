use std::collections::HashMap;
use crate::concepts::LocalisedString;
use thiserror::Error;
use std::fmt;
use factorio_lib_rs_derive::{Prototype, ModSetting, PrototypeBase, Entity};
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
    ResearchTarget,
    AutoplaceControlCategory,
    KeySequence,
    ConsumingType,
    CustomInputAction,
    SpriteVariation,
    BoundingBox,
    RenderLayer,
    TriggerEffect,
    AutoplaceSpecification,
    CollisionMask,
    TriggerTargetMask,
    EntityPrototypeFlags,
    MinableProperties,
    Factorio2DVector,
    RemoveDecoratives,
    CreateTrivialSmokeEffectItem,
    WorkingSound,
    Trigger,
    RadiusVisualizationSpecification,
    ItemToPlace,
    WaterReflectionDefinition
};

// Struct representing global `data` table in lua environment
#[derive(Debug)]
pub struct DataTable {
    prototypes: Vec<PrototypeGeneral>
}

// Factorio prototypes
// Source info:
// For prototypes: https://wiki.factorio.com/Prototype_definitions
// For settings: https://wiki.factorio.com/Tutorial:Mod_settings

// Prototype
// Contains all values (accessors) for every prototype in the game
pub trait Prototype: fmt::Debug {
    fn name(&self) -> &String;
}

pub trait ModSetting: Prototype {
    fn localised_name(&self) -> &Option<LocalisedString>;
    fn localised_description(&self) -> &Option<LocalisedString>;
    fn order(&self) -> &Option<String>;
    fn hidden(&self) -> bool; // Default: false
    fn setting_type(&self) -> ModSettingType;
}

#[derive(Debug, Prototype, ModSetting)]
pub struct BoolModSetting {
    name: String,
    localised_name: Option<LocalisedString>,
    localised_description: Option<LocalisedString>,
    order: Option<String>,
    hidden: bool,
    setting_type: ModSettingType,
    default_value: bool,
    forced_value: Option<bool>,
}

impl BoolModSetting {
    pub fn default_value(&self) -> bool { self.default_value }
    pub fn forced_value(&self) -> Option<bool> { self.forced_value }
}

#[derive(Debug, Prototype, ModSetting)]
pub struct IntModSetting {
    name: String,
    localised_name: Option<LocalisedString>,
    localised_description: Option<LocalisedString>,
    order: Option<String>,
    hidden: bool,
    setting_type: ModSettingType,
    default_value: i64,
    minimum_value: Option<i64>,
    maximum_value: Option<i64>,
    allowed_values: Option<Vec<i64>>,
}

impl IntModSetting {
    pub fn default_value(&self) -> i64 { self.default_value }
    pub fn minimum_value(&self) -> Option<i64> { self.minimum_value }
    pub fn maximum_value(&self) -> Option<i64> { self.maximum_value }
    pub fn allowed_values(&self) -> Option<Vec<i64>> { self.allowed_values.clone() }
}

#[derive(Debug, Prototype, ModSetting)]
pub struct DoubleModSetting {
    name: String,
    localised_name: Option<LocalisedString>,
    localised_description: Option<LocalisedString>,
    order: Option<String>,
    hidden: bool,
    setting_type: ModSettingType,
    default_value: f64,
    minimum_value: Option<f64>,
    maximum_value: Option<f64>,
    allowed_values: Option<Vec<f64>>,
}

impl DoubleModSetting {
    pub fn default_value(&self) -> f64 { self.default_value }
    pub fn minimum_value(&self) -> Option<f64> { self.minimum_value }
    pub fn maximum_value(&self) -> Option<f64> { self.maximum_value }
    pub fn allowed_values(&self) -> Option<Vec<f64>> { self.allowed_values.clone() }
}

#[derive(Debug, Prototype, ModSetting)]
pub struct StringModSetting {
    name: String,
    localised_name: Option<LocalisedString>,
    localised_description: Option<LocalisedString>,
    order: Option<String>,
    hidden: bool,
    setting_type: ModSettingType,
    default_value: String,
    allow_blank: Option<bool>,
    auto_trim: Option<bool>,
    allowed_values: Option<Vec<String>>
}

impl StringModSetting {
    pub fn default_value(&self) -> String { self.default_value.clone() }
    pub fn allow_blank(&self) -> Option<bool> { self.allow_blank }
    pub fn auto_trim(&self) -> Option<bool> {self.auto_trim }
    pub fn allowed_values(&self) -> Option<Vec<String>> { self.allowed_values.clone() }
}

#[derive(Debug, Prototype)]
pub struct AmbientSoundPrototype {
    name: String,
    sound: Sound,
    track_type: String,
    weight: Option<f64>
}

impl AmbientSoundPrototype {
    pub fn sound(&self) -> &Sound { &self.sound }
    pub fn track_type(&self) -> String { self.track_type.clone() }
    pub fn weight(&self) -> Option<f64> { self.weight }
}

#[derive(Debug, Prototype)]
pub struct AnimationPrototype {
    name: String,
    layers: Vec<AnimationType> // If lua table doesn;t have layers, use same table for constructing just one
}

#[derive(Debug, Prototype)]
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

#[derive(Debug, Prototype)]
pub struct Font {
    name: String,
    size: i32,
    from: String,
    spacing: f32, // Default 0.0
    border: bool, // Default fase
    filtered: bool, // Default false
    border_color: Option<Color>
}

#[derive(Debug, Prototype)]
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

#[derive(Debug, Prototype)]
pub struct MapGenPresets {
    name: String,
    presets: HashMap<String, MapGenPreset>
}

#[derive(Debug, Prototype)]
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

#[derive(Debug, Prototype)]
pub struct MouseCursor {
    name: String,
    cursor: MouseCursorType
}

#[derive(Debug, Prototype)]
pub struct SoundPrototype {
    name: String,
    sound: Sound
}

#[derive(Debug, Prototype)]
pub struct SpectatorController {
    name: String, // Must be "default"
    movement_speed: f64 // Must be >= 0.34375
}

#[derive(Debug, Prototype)]
pub struct SpritePrototype {
    name: String,
    sprite: Sprite
}

#[derive(Debug, Prototype)]
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

#[derive(Debug, Prototype)]
pub struct TipsAndTricksItemCategory {
    name: String,
    order: String
}

#[derive(Debug, Prototype)]
pub struct TriggerTargetType {
    name: String
}

#[derive(Debug, Prototype)]
pub struct WindSound {
    name: String,
    sound: Sound
}

// PrototypeBase starts here

pub trait PrototypeBase: Prototype {
    fn localised_description(&self) -> &Option<LocalisedString>;
    fn localised_name(&self) -> &Option<LocalisedString>;
    fn order(&self) -> &String; // Default: ""
}

// Base for Achievement and all inherited types
#[derive(Debug)]
pub struct AchievementBase {
    icon: IconSpecification,
    steam_stats_name: String, // Default: "" // Unusable
    allowed_without_fight: bool, // Default: true
    hidden: bool // Default: false
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct Achievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct BuildEntityAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    to_build: String,
    amount: u32, // Default: 1
    limited_to_one_game: bool, // Default: false
    until_second: u32 // Default: 0 (means infinite)
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct CombatRobotCountAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    count: u32 // Default: 1
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct ConstructWithRobotsAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    limited_to_one_game: bool,
    amount: u32, // Default: 0
    more_than_manually: bool // Default: false
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DeconstructWithRobotsAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    amount: u32
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DeliverByRobotsAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    amount: f64
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DontBuildEntityAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    dont_buid: Vec<String>, // String is converted to Vec<String> with one element
    amount: u32 // Default: 0
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DontCraftManuallyAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    amount: f64
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DontUseEntityInEnergyProductionAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    excluded: Vec<String>, // String is converted to Vec<String> with one element
    included: Vec<String>, // Same as `excluded`
    last_hour_only: bool, // Default: false
    minimum_energy_produced: Energy // Default: 0W
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct FinishTheGameAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    until_second: u32 // Default: 0 (means infinite)
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct GroupAttackAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    amount: u32 // Default: 1
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct KillAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    to_kill: String, // Default: ""
    type_to_kill: Option<String>, // TODO: another prototype enum?
    damage_type: String, // damage type
    amount: u32, // Default: 1
    in_vehicle: bool, // Default: false
    personally: bool // Default: false
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct PlayerDamagedAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    minimum_damage: f32,
    should_survive: bool,
    type_of_dealer: Option<String> // TODO: another prototype enum?
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct ProduceAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    amount: f64,
    limited_to_one_game: bool,
    product: ProductType // Type is determined from item_product or fluid_product // Only one can be set!
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct ProducePerHourAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    amount: f64,
    product: ProductType
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct ResearchAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    target: ResearchTarget // Determined from either `technology` or `research_all` is set
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct TrainPathAchievement {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    achievement: AchievementBase,
    minimum_distance: f64
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct AmmoCategory {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    bonus_gui_order: String // Default: ""
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct AutoplaceControl {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    category: AutoplaceControlCategory,
    rechness: bool // Default: false
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct CustomInput {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    key_sequence: KeySequence, // TODO?: key_sequence parser and checker. Can be empty, if linked_game_control is set, also empty stands for unassigned
    alternate_key_sequence: Option<KeySequence>,
    linked_game_control: String, // Default: ""
    consumed: ConsumingType, // Default: none
    enabled: bool, // Default: true
    enabled_while_spectating: bool, // Default: false
    enabled_while_in_cutscene: bool, // Default: false
    include_selected_prototype: bool, // Default: false
    item_to_spawn: Option<String>, // Name of Item
    action: CustomInputAction // Default: "lua"
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DamageType {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    hidden: bool // Default: false
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct Decorative {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    pictures: Vec<SpriteVariation>, // At least 1 is required
    collision_box: Option<BoundingBox>,
    render_layer: RenderLayer, // Default: "decorative"
    grows_through_rail_path: bool, // Default: false
    tile_layer: i16, // Default: 0 // Mandatory if render_layer is "decals" // I don't understand how this works
    decal_overdraw_priority: u16, // Default: 0 // Only loaded if render_layer is "decals"
    walking_sound: Option<Sound>,
    trigger_effect: Option<TriggerEffect>,
    autoplace: Option<AutoplaceSpecification>,
    collision_mask: CollisionMask // Default: "doodad-layer"
}

#[derive(Debug)]
pub struct EntityBase {
    icon: Option<IconSpecification>, // Mandatory if one of flags active: "placeable-neutral", "placeable-player", "placeable-enemy"
    collision_box: BoundingBox, // Default: ((0, 0), (0, 0))
    collision_mask: CollisionMask, // Default: ("item-layer", "object-layer", "player-layer", "water-tile") and depends on type
    map_generator_bounding_box: BoundingBox,
    selection_box: BoundingBox, // Default: ((0, 0), (0, 0))
    drawing_box: BoundingBox, // Default: ((0, 0), (0, 0)), selection_box is used instead
    sticker_box: BoundingBox, // Default: collision_box
    hit_visualization_box: BoundingBox, // Default: ((0, 0), (0, 0))
    trigger_target_mask: Option<TriggerTargetMask>,
    flags: Option<EntityPrototypeFlags>,
    minable: MinableProperties, // Default: not minable
    subgroup: Option<String>,
    allow_copy_paste: bool, // Default: true
    selectable_in_game: bool, // Default: true
    selection_priority: u8, // Default: 50
    remove_decoratives: RemoveDecoratives, // Default: "automatic"
    emissions_per_second: f64, // Default: 0
    shooting_cursor_size: Option<f64>,
    created_smoke: CreateTrivialSmokeEffectItem, // Default: "smoke-building"-smoke
    working_sound: Option<WorkingSound>,
    created_effect: Option<Trigger>,
    build_sound: Option<Sound>,
    mined_sound: Option<Sound>,
    mining_sound: Option<Sound>,
    rotated_sound: Option<Sound>,
    vehicle_impact_sound: Option<Sound>,
    open_sound: Option<Sound>,
    close_sound: Option<Sound>,
    radius_visualization_specification: Option<RadiusVisualizationSpecification>,
    build_base_evolution_requirement: f64, // Default: 0
    alert_icon_shift: Option<Factorio2DVector>,
    alert_icon_scale: Option<f32>,
    fast_replaceable_group: String, // Default: ""
    next_upgrade: Option<String>, // Name of the entity // Has limitations, listed on wiki
    placeable_by: Option<Vec<ItemToPlace>>,
    remains_when_mined: Option<Vec<String>>,
    additional_pastable_entities: Option<Vec<String>>,
    tile_width: u32, // Default: Calculated from collision_box
    tile_height: u32, // Default: Calculated from collision_box
    autoplace: Option<AutoplaceSpecification>,
    map_color: Option<Color>,
    friendly_map_color: Option<Color>,
    enemy_map_color: Option<Color>,
    water_reflection: Option<WaterReflectionDefinition>
}

pub trait Entity: PrototypeBase {
    fn icon(&self) -> &Option<IconSpecification>;
    fn collision_box(&self) -> BoundingBox;
    fn collision_mask(&self) -> CollisionMask;
    fn map_generator_bounding_box(&self) -> BoundingBox;
    fn selection_box(&self) -> BoundingBox;
    fn drawing_box(&self) -> BoundingBox;
    fn sticker_box(&self) -> BoundingBox;
    fn hit_visualization_box(&self) -> BoundingBox;
    fn trigger_target_mask(&self) -> &Option<TriggerTargetMask>;
    fn flags(&self) -> Option<EntityPrototypeFlags>;
    fn minable(&self) -> &MinableProperties;
    fn subgroup(&self) -> &Option<String>;
    fn allow_copy_paste(&self) -> bool;
    fn selectable_in_game(&self) -> bool;
    fn selection_priority(&self) -> u8;
    fn remove_decoratives(&self) -> RemoveDecoratives;
    fn emissions_per_second(&self) -> f64;
    fn shooting_cursor_size(&self) -> Option<f64>;
    fn created_smoke(&self) -> &CreateTrivialSmokeEffectItem;
    fn working_sound(&self) -> &Option<WorkingSound>;
    fn created_effect(&self) -> &Option<Trigger>;
    fn build_sound(&self) -> &Option<Sound>;
    fn mined_sound(&self) -> &Option<Sound>;
    fn mining_sound(&self) -> &Option<Sound>;
    fn rotated_sound(&self) -> &Option<Sound>;
    fn vehicle_impact_sound(&self) -> &Option<Sound>;
    fn open_sound(&self) -> &Option<Sound>;
    fn close_sound(&self) -> &Option<Sound>;
    fn radius_visualization_specification(&self) -> &Option<RadiusVisualizationSpecification>;
    fn build_base_evolution_requirement(&self) -> f64;
    fn alert_icon_shift(&self) -> Option<Factorio2DVector>;
    fn alert_icon_scale(&self) -> Option<f32>;
    fn fast_replaceable_group(&self) -> String;
    fn next_upgrade(&self) -> &Option<String>;
    fn placeable_by(&self) -> &Option<Vec<ItemToPlace>>;
    fn remains_when_mined(&self) -> &Option<Vec<String>>;
    fn additional_pastable_entities(&self) -> &Option<Vec<String>>;
    fn tile_width(&self) -> u32;
    fn tile_height(&self) -> u32;
    fn autoplace(&self) -> &Option<AutoplaceSpecification>;
    fn map_color(&self) -> Option<Color>;
    fn friendly_map_color(&self) -> Option<Color>;
    fn enemy_map_color(&self) -> Option<Color>;
    fn water_reflection(&self) -> &Option<WaterReflectionDefinition>;
}

#[derive(Debug, Prototype, PrototypeBase, Entity)]
pub struct Arrow {
    name: String,
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String,
    entity_base: EntityBase,
    arrow_picture: Sprite,
    circle_picture: Option<Sprite>,
    blinking: bool, // Default: false
}

// Enum for all prototypes
#[derive(Debug)]
pub enum PrototypeGeneral {
    // General prototypes
    AmbientSound(AmbientSoundPrototype),
    Animation(AnimationPrototype),
    EditorController(EditorController),
    Font(Font),
    GodController(GodController),
    MapGenPresets(MapGenPresets),
    MapSettings(MapSettings),
    MouseCursor(MouseCursor),
    Sound(SoundPrototype),
    SpectatorController(SpectatorController),
    Sprite(SpritePrototype),
    TileEffect(TileEffect),
    TipsAndTricksItemCategory(TipsAndTricksItemCategory),
    TriggerTargetType(TriggerTargetType),
    WindSound(WindSound),
    Achievement(Achievement),
    BuildEntityAchievement(BuildEntityAchievement),
    CombatRobotCountAchievement(CombatRobotCountAchievement),
    ConstructWithRobotsAchievement(ConstructWithRobotsAchievement),
    DeconstructWithRobotsAchievement(DeconstructWithRobotsAchievement),
    DeliverByRobotsAchievement(DeliverByRobotsAchievement),
    DontBuildEntityAchievement(DontBuildEntityAchievement),
    DontCraftManuallyAchievement(DontCraftManuallyAchievement),
    DontUseEntityInEnergyProductionAchievement(DontUseEntityInEnergyProductionAchievement),
    FinishTheGameAchievement(FinishTheGameAchievement),
    GroupAttackAchievement(GroupAttackAchievement),
    KillAchievement(KillAchievement),
    PlayerDamagedAchievement(PlayerDamagedAchievement),
    ProduceAchievement(ProduceAchievement),
    ProducePerHourAchievement(ProducePerHourAchievement),
    ResearchAchievement(ResearchAchievement),
    TrainPathAchievement(TrainPathAchievement),
    AmmoCategory(AmmoCategory),
    AutoplaceControl(AutoplaceControl),
    CustomInput(CustomInput),
    DamageType(DamageType),
    Decorative(Decorative),
    Arrow(Arrow),
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

#[derive(Clone, Debug, Error)]
pub enum PrototypesErr {
    #[error("Invalid prototype type: {0}")]
    InvalidPrototypeType(String),
    #[error("Invalid mod setting type: {0}")]
    InvalidModSettingType(String),
    #[error("Invalid string for type {0}: {1}")]
    InvalidTypeStr(String, String)
}
