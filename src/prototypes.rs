use std::collections::HashMap;
use crate::concepts::LocalisedString;
use thiserror::Error;
use std::fmt;
use factorio_lib_rs_derive::{Prototype, ModSetting, PrototypeBase, Entity, Corpse, EntityWithHealth, Combinator, CraftingMachine};
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
    Animation,
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
    Factorio3DVector,
    RemoveDecoratives,
    CreateTrivialSmokeEffectItem,
    WorkingSound,
    Trigger,
    RadiusVisualizationSpecification,
    ItemToPlace,
    WaterReflectionDefinition,
    AnimationVariation,
    LightAnimations,
    OrientedCliffPrototypes,
    RotatedAnimationVariation,
    BendingType,
    RailRemnantsPictures,
    Sprite4Way,
    ExplosionDefinition,
    Resistance,
    Loot,
    AttackReactionItem,
    EnergySource,
    LightDefinition,
    WireConnectionPoint,
    CircuitConnectorSprites,
    SignalIDConnector,
    Animation4Way,
    RotatedSprite,
    InterruptibleSound,
    ModuleSpecification,
    BeaconGraphicsSet,
    EffectTypeLimitation,
    FluidBox,
    BoilerMode,
    CharacterArmorAnimation,
    FootstepTriggerEffectList,
    FootprintParticle,
    LogisticMode,
    InfinityContainerGuiMode,
    WorkingVisualization,
    CraftingMachineDefaultRecipeTint,
    CraftingMachineShiftAnimationWaypoints,
    CraftingMachineStatusColors
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
    layers: Vec<Animation> // If lua table doesn't have layers, use same table for constructing just one
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

#[derive(Debug)]
pub struct PrototypeBaseSpec {
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String
}

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
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct BuildEntityAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    to_build: String,
    amount: u32, // Default: 1
    limited_to_one_game: bool, // Default: false
    until_second: u32 // Default: 0 (means infinite)
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct CombatRobotCountAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    count: u32 // Default: 1
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct ConstructWithRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    limited_to_one_game: bool,
    amount: u32, // Default: 0
    more_than_manually: bool // Default: false
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DeconstructWithRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: u32
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DeliverByRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DontBuildEntityAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    dont_buid: Vec<String>, // String is converted to Vec<String> with one element
    amount: u32 // Default: 0
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DontCraftManuallyAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct DontUseEntityInEnergyProductionAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    excluded: Vec<String>, // String is converted to Vec<String> with one element
    included: Vec<String>, // Same as `excluded`
    last_hour_only: bool, // Default: false
    minimum_energy_produced: Energy // Default: 0W
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct FinishTheGameAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    until_second: u32 // Default: 0 (means infinite)
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct GroupAttackAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: u32 // Default: 1
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct KillAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
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
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    minimum_damage: f32,
    should_survive: bool,
    type_of_dealer: Option<String> // TODO: another prototype enum?
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct ProduceAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64,
    limited_to_one_game: bool,
    product: ProductType // Type is determined from item_product or fluid_product // Only one can be set!
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct ProducePerHourAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64,
    product: ProductType
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct ResearchAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    target: ResearchTarget // Determined from either `technology` or `research_all` is set
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct TrainPathAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    minimum_distance: f64
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct AmmoCategory {
    name: String,
    prototype_base: PrototypeBaseSpec,
    bonus_gui_order: String // Default: ""
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct AutoplaceControl {
    name: String,
    prototype_base: PrototypeBaseSpec,
    category: AutoplaceControlCategory,
    rechness: bool // Default: false
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct CustomInput {
    name: String,
    prototype_base: PrototypeBaseSpec,
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
    prototype_base: PrototypeBaseSpec,
    hidden: bool // Default: false
}

#[derive(Debug, Prototype, PrototypeBase)]
pub struct Decorative {
    name: String,
    prototype_base: PrototypeBaseSpec,
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
    fn fast_replaceable_group(&self) -> &String;
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
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    arrow_picture: Sprite,
    circle_picture: Option<Sprite>,
    blinking: bool, // Default: false
}

#[derive(Debug, Prototype, PrototypeBase, Entity)]
pub struct ArtilleryFlare {
    // map_color is mandatory
    // selection_priority default: 48
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    pictures: Vec<AnimationVariation>,
    life_time: u16,
    shadows: Option<Vec<AnimationVariation>>,
    render_layer: RenderLayer, // Default: "object"
    render_layer_when_on_ground: RenderLayer, // Default: "lower-object"
    regular_trigger_effect: Option<TriggerEffect>,
    regular_trigger_effect_frequency: u32, // Default: 0
    ended_in_water_trigger_effect: Option<TriggerEffect>,
    movement_modifier_when_on_ground: f64, // Default: 0.8
    creation_shift: Option<Factorio2DVector>,
    initial_speed: Option<Factorio2DVector>,
    initial_height: f32, // Default: 0
    initial_vertical_speed: f32, // Default: 0
    initial_frame_speed: f32, // Default: 1
    shots_per_flare: u32, // Default: 1
    early_death_ticks: u32, // Default: 3 * 60 (180)
    shot_category: String, // Name of Prototype/AmmoCategory
}

#[derive(Debug, Prototype, PrototypeBase, Entity)]
pub struct ArtilleryProjectile {
    // Bounding box must be zero
    // map_color is mandatory
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    reveal_map: bool,
    pcture: Option<Sprite>,
    shadow: Option<Sprite>,
    chart_picture: Option<Sprite>,
    action: Option<Trigger>,
    final_action: Option<Trigger>,
    height_from_ground: f32, // Default: 1
    rotatable: bool, // Default: true
}

#[derive(Debug, Prototype, PrototypeBase, Entity)]
pub struct Beam {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    width: f64,
    damage_interval: u32, // Can't be 0
    head: Animation,
    tail: Animation,
    body: Vec<AnimationVariation>, // Must have at least 1 variation
    action: Option<Trigger>,
    target_offset: Option<Factorio2DVector>,
    random_target_offset: bool, // Default: false
    action_triggered_automatically: bool, // Default: false
    random_end_animation_rotation: bool, // Default: true
    transparent_start_end_animations: bool, // Default: true
    start: Option<Animation>,
    ending: Option<Animation>,
    light_animations: Option<LightAnimations>,
    ground_light_animations: Option<LightAnimations>,
    // These values are considered deprecated.
    // If present, converted to light_animations, other *_animations properties are ignored
    // start_light: Option<Animation>
    // ending_light: Option<Animation>
    // head_light: Option<Animation>
    // tail_light: Option<Animation>
    // body_light: Option<Vec<AnimationVariation>>
}

#[derive(Debug, Prototype, PrototypeBase, Entity)]
pub struct CharacterCorpse {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    time_to_live: u32,
    render_layer: RenderLayer, // Default: "object"
    pictures: Vec<AnimationVariation>, // Mandatory // picture field is converted to this
    armor_picture_mapping: HashMap<String, usize> // Exact type of animation index is unknown, it references index in pictures field
}

#[derive(Debug, Prototype, PrototypeBase, Entity)]
pub struct Cliff {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    orientations: OrientedCliffPrototypes,
    grid_size: Factorio2DVector,
    grid_offset: Factorio2DVector,
    cliff_height: f32, // Default: 4
    cliff_explosive: String, // Name of capsule that has a robot_action to explode cliffs
}

#[derive(Debug)]
pub struct CorpseBase {
    dying_speed: f32, // Default: 1
    splash_speed: f32, // Default: 1
    time_before_shading_off: i32, // Default: 60 * 15
    time_before_removed: i32, // Default: 60 * 120
    remove_on_entity_placemen: bool, // Default: true
    remove_on_tile_placement: bool, // Default: true
    final_render_layer: RenderLayer, // Default: "corpse"
    gound_patch_render_layer: RenderLayer, // Default: "ground-patch"
    animation_render_layer: RenderLayer, // Default: "object"
    splash_render_layer: RenderLayer, // Default: "object"
    animation_overlay_render_layer: RenderLayer, // Default: "object"
    animation_overlay_final_render_layer: RenderLayer, // Default: "corpse"
    shuffle_directions_at_frame: u8, // Default: 1
    use_tile_color_for_ground_patch_tint: bool, // Default: false
    ground_patch_fade_in_delay: f32, // Default: 0
    ground_patch_fade_in_speed: f32, // Default: 0
    ground_patch_fade_out_start: f32, // Default: 0
    animation: Option<Vec<RotatedAnimationVariation>>,
    animation_overlay: Option<Vec<RotatedAnimationVariation>>,
    splash: Option<Vec<AnimationVariation>>,
    ground_patch: Option<Vec<AnimationVariation>>,
    ground_patch_higher: Option<Vec<AnimationVariation>>,
    ground_patch_fade_out_duration: f32, // Default: 0
    direction_shuffle: Option<Vec<Vec<u16>>> // Inner Vecs should be the same size
}

pub trait Corpse: Entity {
    fn dying_speed(&self) -> f32;
    fn splash_speed(&self) -> f32;
    fn time_before_shading_off(&self) -> i32;
    fn time_before_removed(&self) -> i32;
    fn remove_on_entity_placemen(&self) -> bool;
    fn remove_on_tile_placement(&self) -> bool;
    fn final_render_layer(&self) -> RenderLayer;
    fn gound_patch_render_layer(&self) -> RenderLayer;
    fn animation_render_layer(&self) -> RenderLayer;
    fn splash_render_layer(&self) -> RenderLayer;
    fn animation_overlay_render_layer(&self) -> RenderLayer;
    fn animation_overlay_final_render_layer(&self) -> RenderLayer;
    fn shuffle_directions_at_frame(&self) -> u8;
    fn use_tile_color_for_ground_patch_tint(&self) -> bool;
    fn ground_patch_fade_in_delay(&self) -> f32;
    fn ground_patch_fade_in_speed(&self) -> f32;
    fn ground_patch_fade_out_start(&self) -> f32;
    fn animation(&self) -> &Option<Vec<RotatedAnimationVariation>>;
    fn animation_overlay(&self) -> &Option<Vec<RotatedAnimationVariation>>;
    fn splash(&self) -> &Option<Vec<AnimationVariation>>;
    fn ground_patch(&self) -> &Option<Vec<AnimationVariation>>;
    fn ground_patch_higher(&self) -> &Option<Vec<AnimationVariation>>;
    fn ground_patch_fade_out_duration(&self) -> f32;
    fn direction_shuffle(&self) -> &Option<Vec<Vec<u16>>>;
}

#[derive(Debug, Prototype, PrototypeBase, Entity, Corpse)]
pub struct CorpsePrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    corpse_base: CorpseBase
}

#[derive(Debug, Prototype, PrototypeBase, Entity, Corpse)]
pub struct RailRemnants {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    corpse_base: CorpseBase,
    bending_type: BendingType,
    pictures: RailRemnantsPictures
}

#[derive(Debug, Prototype, PrototypeBase, Entity)]
pub struct DeconstructibleTileProxy {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
}

#[derive(Debug, Prototype, PrototypeBase, Entity)]
pub struct EntityGhost {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    medium_build_sound: Option<Sound>,
    large_build_sound: Option<Sound>
}

/// <https://wiki.factorio.com/Prototype/EntityWithHealth>
#[derive(Debug)]
pub struct EntityWithHealthBase {
    // Yes, this one includes PrototypeBase and Entity
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    max_health: f32, // Default: 10
    healing_per_tick: f32, // Default: 0.001666 for Prototype/Tree, 0 for the rest
    repair_speed_multiplier: f32, // Default: 1
    dying_explosion: Option<Vec<ExplosionDefinition>>,
    drying_trigger_effect: Option<TriggerEffect>,
    damaged_trigger_effect: Option<TriggerEffect>,
    loot: Option<Vec<Loot>>,
    resistances: Option<Vec<Resistance>>,
    attack_reaction: Vec<AttackReactionItem>, // Default: Empty
    repair_sound: Sound, // Default: Utility Sound (defaultManualRepair)
    alert_when_damaged: bool, // Default: true
    hide_resistances: bool, // Default: true
    create_ghost_on_death: bool, // Default: true
    random_corpse_variation: bool, // Default: false
    integration_patch_render_layer: RenderLayer, // Default: "lower-object"
    corpse: Vec<String>, // Default: Empty // (Names) Name of Prototype/Corpse
    integration_patch: Sprite4Way
}

/// <https://wiki.factorio.com/Prototype/EntityWithHealth>
pub trait EntityWithHealth: Entity {
    fn max_health(&self) -> f32;
    fn healing_per_tick(&self) -> f32;
    fn repair_speed_multiplier(&self) -> f32;
    fn dying_explosion(&self) -> &Option<Vec<ExplosionDefinition>>;
    fn drying_trigger_effect(&self) -> &Option<TriggerEffect>;
    fn damaged_trigger_effect(&self) -> &Option<TriggerEffect>;
    fn loot(&self) -> &Option<Vec<Loot>>;
    fn resistances(&self) -> &Option<Vec<Resistance>>;
    fn attack_reaction(&self) -> &Vec<AttackReactionItem>;
    fn repair_sound(&self) -> &Sound;
    fn alert_when_damaged(&self) -> bool;
    fn hide_resistances(&self) -> bool;
    fn create_ghost_on_death(&self) -> bool;
    fn random_corpse_variation(&self) -> bool;
    fn integration_patch_render_layer(&self) -> RenderLayer;
    fn corpse(&self) -> &Vec<String>;
    fn integration_patch(&self) -> &Sprite4Way;
}

#[derive(Debug, EntityWithHealth)]
pub struct Accumulator {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    energy_source: EnergySource,
    picture: Sprite,
    charge_cooldown: u16,
    discharge_cooldown: u16,
    charge_animation: Option<Animation>,
    charge_light: Option<LightDefinition>,
    discharge_animation: Option<Animation>,
    discharge_light: Option<LightDefinition>,
    circuit_wire_connection_point: Option<WireConnectionPoint>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_connector_sprites: Option<CircuitConnectorSprites>,
    default_output_signal: Option<SignalIDConnector>
}

#[derive(Debug, EntityWithHealth)]
pub struct ArtilleryTurret {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    gun: String, // Name of a gun item
    inventory_size: u16, // Must be > 0
    ammo_stack_limit: u32, // Must be > 0
    automated_ammo_count: u32,
    turret_rotation_speed: f64,
    manual_range_modifier: f64, // Must be positive
    alert_when_attacking: bool, // Default: true
    disable_automatic_firing: bool, // Default: false
    base_picture_secondary_draw_order: u8, // Default: 0
    base_picture_render_layer: RenderLayer, // Default: "lower-object"
    base_shift: Option<Factorio2DVector>,
    base_picture: Option<Animation4Way>,
    cannon_base_pictures: Option<RotatedSprite>,
    cannon_barrel_pictures: Option<RotatedSprite>,
    rotating_sound: Option<InterruptibleSound>,
    rotating_stopped_sound: Option<Sound>,
    turn_after_shooting_cooldown: u16, // Default: 0
    cannon_parking_frame_count: u16, // Default: 0
    cannon_parking_speed: u16, // Default: 1
    cannon_barrel_recoil_shiftings: Option<Vec<Factorio3DVector>>,
    cannon_barrel_recoil_shiftings_load_correction_matrix: Option<Vec<Factorio3DVector>>, // Only loaded if cannon_barrel_recoil_shiftings is loaded
    cannon_barrel_light_direction: Option<Factorio3DVector> // Only loaded if cannon_barrel_recoil_shiftings is loaded
}

#[derive(Debug, EntityWithHealth)]
pub struct Beacon {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    energy_usage: Energy,
    energy_source: EnergySource,
    supply_area_distance: f64,
    distribution_effectivity: f64,
    module_specification: ModuleSpecification,
    graphics_set: Option<BeaconGraphicsSet>,
    animation: Option<Animation>, // Loaded only if `graphics_set` is not present
    base_picture: Option<Sprite>, // Loaded only if `graphics_set` is not present
    radius_visualization_picture: Option<Sprite>,
    allowed_effects: Option<EffectTypeLimitation>
}

#[derive(Debug, EntityWithHealth)]
pub struct Boiler {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    energy_source: EnergySource,
    fluid_box: FluidBox,
    output_fluid_box: FluidBox,
    energy_consumption: Energy,
    burning_cooldown: u32,
    target_temperature: f64,
    structure: Animation4Way,
    fire: Animation4Way, // Can be empty
    fire_glow: Animation4Way, // Can be empty
    fire_glow_flicker_enabled: bool, // Default: false
    fire_flicker_enabled: bool, // Default: false
    mode: BoilerMode, // Default: "heat-water-inside"
    patch: Option<Sprite4Way>,
}

#[derive(Debug, EntityWithHealth)]
pub struct BurnerGenerator {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    energy_source: EnergySource, // Emissions are ignored
    burner: EnergySource, // Must be a burner energy source
    animation: Animation4Way,
    max_power_output: Energy,
    idle_animation: Option<Animation4Way>,
    always_draw_idle_animation: bool, // Default: false
    min_perceived_performance: f64, // Default: 0.25
    performance_to_sound_speedup: f64, // Default: 0.5
}

#[derive(Debug, EntityWithHealth)]
pub struct Character {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    mining_speed: f64,
    running_speed: f64,
    distance_per_frame: f64,
    maximum_corner_sliding_distance: f64,
    heartbeat: Sound,
    eat: Sound,
    inventory_size: ItemStackIndex,
    build_distance: u32,
    drop_item_distance: u32,
    reach_distance: u32,
    reach_resource_distance: f64,
    item_pickup_distance: f64,
    loot_pickup_distance: f64,
    ticks_to_keep_gun: u32,
    ticks_to_keep_aiming_direction: u32,
    ticks_to_stay_in_combat: u32,
    damage_hit_tint: Color,
    running_sound_animation_positions: Vec<f32>,
    mining_with_tool_particles_animation_positions: Vec<f32>,
    animations: Vec<CharacterArmorAnimation>,
    crafting_categories: Option<Vec<String>>, // (Names) Name of crafting category
    mining_categories: Option<Vec<String>>, // (Names) Name of mining category
    light: Option<LightDefinition>,
    enter_vehicle_distance: f64, // Default: 3.0
    tool_attack_distance: f64, // Default: 1.5
    respawn_time: u32, // Default: 10
    has_belt_immunity: bool, // Default: false
    character_corpse: Option<String>,
    footstep_particle_triggers: Option<FootstepTriggerEffectList>,
    synced_footstep_particle_triggers: Option<FootstepTriggerEffectList>,
    footprint_particles: Option<Vec<FootprintParticle>>,
    left_footprint_offset: Option<Factorio2DVector>,
    right_footprint_offset: Option<Factorio2DVector>,
    right_footprint_frames: Option<Factorio2DVector>,
    left_footprint_frames: Option<Factorio2DVector>,
    tool_attack_result: Option<Trigger>,
}

#[derive(Debug)]
pub struct CombinatorBase {
    energy_source: EnergySource, // Must be an electric void energy source
    active_energy_usage: Energy,
    sprites: Sprite4Way,
    activity_led_sprites: Sprite4Way,
    input_connection_bounding_box: BoundingBox,
    output_connection_bounding_box: BoundingBox,
    activity_led_light_offsets: [Factorio2DVector; 4],
    screen_light_offsets: [Factorio2DVector; 4],
    input_connection_points: [WireConnectionPoint; 4],
    output_connection_points: [WireConnectionPoint; 4],
    activity_led_light: Option<LightDefinition>,
    screen_light: Option<LightDefinition>,
    activity_led_hold_time: u8, // Default: 5
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
}

pub trait Combinator {
    fn energy_source(&self) -> &EnergySource;
    fn active_energy_usage(&self) -> Energy;
    fn sprites(&self) -> &Sprite4Way;
    fn activity_led_sprites(&self) -> &Sprite4Way;
    fn input_connection_bounding_box(&self) -> BoundingBox;
    fn output_connection_bounding_box(&self) -> BoundingBox;
    fn activity_led_light_offsets(&self) -> [Factorio2DVector; 4];
    fn screen_light_offsets(&self) -> [Factorio2DVector; 4];
    fn input_connection_points(&self) -> &[WireConnectionPoint; 4];
    fn output_connection_points(&self) -> &[WireConnectionPoint; 4];
    fn activity_led_light(&self) -> &Option<LightDefinition>;
    fn screen_light(&self) -> &Option<LightDefinition>;
    fn activity_led_hold_time(&self) -> u8;
    fn circuit_wire_max_distance(&self) -> f64;
    fn draw_copper_wires(&self) -> bool;
    fn draw_circuit_wires(&self) -> bool;
}

#[derive(Debug, EntityWithHealth, Combinator)]
pub struct ArithmeticCombinator {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    combinator_base: CombinatorBase,
    plus_symbol_sprites: Sprite4Way,
    minus_symbol_sprites: Sprite4Way,
    multiply_symbol_sprites: Sprite4Way,
    divide_symbol_sprites: Sprite4Way,
    modulo_symbol_sprites: Sprite4Way,
    power_symbol_sprites: Sprite4Way,
    left_shift_symbol_sprites: Sprite4Way,
    right_shift_symbol_sprites: Sprite4Way,
    and_symbol_sprites: Sprite4Way,
    or_symbol_sprites: Sprite4Way,
    xor_symbol_sprites: Sprite4Way,
}

#[derive(Debug, EntityWithHealth, Combinator)]
pub struct DeciderCombinator {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    combinator_base: CombinatorBase,
    equal_symbol_sprites: Sprite4Way,
    greater_symbol_sprites: Sprite4Way,
    less_symbol_sprites: Sprite4Way,
    not_equal_symbol_sprites: Sprite4Way,
    greater_or_equal_symbol_sprites: Sprite4Way,
    less_or_equal_symbol_sprites: Sprite4Way,
}

#[derive(Debug, EntityWithHealth)]
pub struct ConstantCombinator {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    item_slot_count: u32,
    sprites: Sprite4Way,
    activity_led_sprites: Sprite4Way,
    activity_led_light_offsets: [Factorio2DVector; 4],
    circuit_wire_connection_points: [WireConnectionPoint; 4],
    activity_led_light: Option<LightDefinition>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
}

#[derive(Debug, EntityWithHealth)]
pub struct Container {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    inventory_size: u16,
    picture: Sprite,
    enable_inventory_bar: bool, // Default: true
    scale_info_icons: bool, // Default: false
    circuit_wire_connection_point: Option<WireConnectionPoint>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_connector_sprites: Option<CircuitConnectorSprites>
}

#[derive(Debug, EntityWithHealth)]
pub struct LogisticContainer {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    inventory_size: u16,
    picture: Option<Sprite>,
    logistic_mode: LogisticMode,
    enable_inventory_bar: bool, // Default: true
    scale_info_icons: bool, // Default: false
    circuit_wire_connection_point: Option<WireConnectionPoint>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_connector_sprites: Option<CircuitConnectorSprites>,
    max_logistic_slots: Option<u16>, // requester-type must have > 0 and <= 1000 // Storage type must have <= 1
    render_not_in_network_icon: bool, // Default: true
    opened_duration: u8, // Default: 0
    animation: Option<Animation>,
    landing_location_offset: Option<Factorio2DVector>,
    animation_sound: Option<Sound>
}

#[derive(Debug, EntityWithHealth)]
pub struct InfinityContainer {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    inventory_size: u16, // Can't be 0
    picture: Option<Sprite>,
    logistic_mode: Option<LogisticMode>,
    erase_contents_when_mined: bool,
    enable_inventory_bar: bool, // Default: true
    scale_info_icons: bool, // Default: false
    circuit_wire_connection_point: Option<WireConnectionPoint>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_connector_sprites: Option<CircuitConnectorSprites>,
    max_logistic_slots: Option<u16>, // requester-type must have > 0 and <= 1000 // Storage type must have <= 1
    render_not_in_network_icon: bool, // Default: false
    opened_duration: u8, // Default: 0
    animation: Option<Animation>,
    landing_location_offset: Option<Factorio2DVector>,
    animation_sound: Option<Sound>,
    gui_mode: InfinityContainerGuiMode // Default: "none"
}

#[derive(Debug)]
pub struct CraftingMachineBase {
    energy_usage: Energy, // Must be positive
    crafting_speed: f64, // Must be positive
    crafting_categories: Vec<String>, // (Names) Name of crafting category
    energy_source: EnergySource, // if drain is not specified, automatically set to energy_usage / 30
    fluid_boxes: Option<Vec<FluidBox>>,
    allowed_effects: Option<EffectTypeLimitation>,
    scale_entity_info_icon: bool, // Default: false
    show_recipe_icon: bool, // Default: true
    return_ingredients_on_change: bool, // Default: true
    animation: Option<Animation4Way>,
    idle_animation: Option<Animation4Way>,
    always_draw_idle_animation: bool, // Default: false
    default_recipe_tint: Option<CraftingMachineDefaultRecipeTint>,
    shift_animation_waypoints: Option<CraftingMachineShiftAnimationWaypoints>, // Only loaded if `shift_animation_waypoint_stop_duration` or `shift_animation_transition_duration` is not 0
    shift_animation_waypoint_stop_duration: u16, // Default: 0 // Only loaded if `shift_animation_waypoints` is present
    shift_animation_transition_duration: u16, // Default: 0 // Only loaded if `shift_animation_waypoints` is present
    status_colors: Option<CraftingMachineStatusColors>,
    entity_info_icon_shift: Factorio2DVector, // Default: {0, -0.3} for 
    draw_entity_info_icon_background: bool, // Default: true
    match_animation_speed_to_activity: bool, // Default: false
    show_recipe_icon_on_map: bool, // Default: true
    base_productivity: f32, // Default: 0
    module_specification: Option<ModuleSpecification>,
    working_visualisations: Option<Vec<WorkingVisualization>>,
}

pub trait CraftingMachine {
    fn energy_usage(&self) -> Energy;
    fn crafting_speed(&self) -> f64;
    fn crafting_categories(&self) -> &Vec<String>;
    fn energy_source(&self) -> &EnergySource;
    fn fluid_boxes(&self) -> &Option<Vec<FluidBox>>;
    fn allowed_effects(&self) -> &Option<EffectTypeLimitation>;
    fn scale_entity_info_icon(&self) -> bool;
    fn show_recipe_icon(&self) -> bool;
    fn return_ingredients_on_change(&self) -> bool;
    fn animation(&self) -> &Option<Animation4Way>;
    fn idle_animation(&self) -> &Option<Animation4Way>;
    fn always_draw_idle_animation(&self) -> bool;
    fn default_recipe_tint(&self) -> &Option<CraftingMachineDefaultRecipeTint>;
    fn shift_animation_waypoints(&self) -> &Option<CraftingMachineShiftAnimationWaypoints>;
    fn shift_animation_waypoint_stop_duration(&self) -> u16;
    fn shift_animation_transition_duration(&self) -> u16;
    fn status_colors(&self) -> &Option<CraftingMachineStatusColors>;
    fn entity_info_icon_shift(&self) -> Factorio2DVector;
    fn draw_entity_info_icon_background(&self) -> bool;
    fn match_animation_speed_to_activity(&self) -> bool;
    fn show_recipe_icon_on_map(&self) -> bool;
    fn base_productivity(&self) -> f32;
    fn module_specification(&self) -> &Option<ModuleSpecification>;
    fn working_visualisations(&self) -> &Option<Vec<WorkingVisualization>>;
}

#[derive(Debug, EntityWithHealth, CraftingMachine)]
pub struct AssemblingMachine {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    crafting_machine_base: CraftingMachineBase,
    fixed_recipe: String, // Default: "" // Name of Recipe
    gui_title_key: String, // Default: ""
    ingredient_count: u8, // Default: 255
}

#[derive(Debug, EntityWithHealth, CraftingMachine)]
pub struct RocketSilo {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    crafting_machine_base: CraftingMachineBase,
    fixed_recipe: String, // Default: "" // Name of Recipe
    gui_title_key: String, // Default: ""
    ingredient_count: u8, // Default: 255
    active_energy_usage: Energy,
    idle_energy_usage: Energy,
    lamp_energy_usage: Energy,
    rocket_entity: String, // Name of RocketSiloRocket
    satellite_animation: Animation,
    satellite_shadow_animation: Animation,
    arm_01_back_animation: Animation,
    arm_02_right_animation: Animation,
    arm_03_front_animation: Animation,
    shadow_sprite: Sprite,
    hole_sprite: Sprite,
    hole_light_sprite: Sprite,
    rocket_shadow_overlay_sprite: Sprite,
    rocket_glow_overlay_sprite: Sprite,
    door_back_sprite: Sprite,
    door_front_sprite: Sprite,
    base_day_sprite: Sprite,
    base_front_sprite: Sprite,
    red_lights_back_sprites: Sprite,
    red_lights_front_sprites: Sprite,
    hole_clipping_box: BoundingBox,
    door_back_open_offset: Factorio2DVector,
    door_front_open_offset: Factorio2DVector,
    silo_fade_out_start_distance: f64,
    silo_fade_out_end_distance: f64,
    times_to_blink: u8,
    light_blinking_speed: f64,
    door_opening_speed: f64,
    rocket_parts_required: u32,
    base_night_sprite: Option<Sprite>,
    base_light: Option<LightDefinition>,
    base_engine_light: Option<LightDefinition>,
    alarm_trigger: Option<TriggerEffect>,
    clamps_on_trigger: Option<TriggerEffect>,
    clamps_off_trigger: Option<TriggerEffect>,
    doors_trigger: Option<TriggerEffect>,
    raise_rocket_trigger: Option<TriggerEffect>,
    alarm_sound: Option<Sound>,
    clamps_on_sound: Option<Sound>,
    clamps_off_sound: Option<Sound>,
    doors_sound: Option<Sound>,
    raise_rocket_sound: Option<Sound>,
    flying_sound: Option<Sound>,
    rocket_result_inventory_size: u16 // Default: 0
}

#[derive(Debug, EntityWithHealth, CraftingMachine)]
pub struct Furnace {
    name: String,
    entity_with_health_base: EntityWithHealthBase,
    crafting_machine_base: CraftingMachineBase,
    result_inventory_size: u16,
    source_inventory_size: u16 // Not more than 1
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
    ArtilleryFlare(ArtilleryFlare),
    ArtilleryProjectile(ArtilleryProjectile),
    Beam(Beam),
    CharacterCorpse(CharacterCorpse),
    Cliff(Cliff),
    Corpse(CorpsePrototype),
    RailRemnants(RailRemnants),
    DeconstructibleTileProxy(DeconstructibleTileProxy),
    EntityGhost(EntityGhost),
    // For migration, cannot be used
    //EntityParticle,
    //LeafParticle,
    Accumulator(Accumulator),
    ArtilleryTurret(ArtilleryTurret),
    Beacon(Beacon),
    Boiler(Boiler),
    BurnerGenerator(BurnerGenerator),
    Character(Character),
    ArithmeticCombinator(ArithmeticCombinator),
    DeciderCombinator(DeciderCombinator),
    ConstantCombinator(ConstantCombinator),
    Container(Container),
    LogisticContainer(LogisticContainer),
    InfinityContainer(InfinityContainer),
    AssemblingMachine(AssemblingMachine),
    RocketSilo(RocketSilo),
    Furnace(Furnace),
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
    //SimpleSmoke, // note: for migration, cannot be used.
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
    //MiningTool, // note: for migration, cannot be used.
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
