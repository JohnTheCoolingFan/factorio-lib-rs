use std::rc::Rc;
use std::collections::HashMap;
use crate::concepts::LocalisedString;
use thiserror::Error;
use mlua::{Lua, Value, prelude::LuaResult};
use std::fmt;
use factorio_lib_rs_derive::{
    Prototype,
    ModSetting,
    PrototypeBase,
    Entity,
    Corpse,
    EntityWithHealth,
    Combinator,
    CraftingMachine,
    FlyingRobot,
    TransportBeltConnectable,
    Vehicle,
    RollingStock,
    Equipment
};
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
    ItemCountType,
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
    SpriteVariations,
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
    AnimationVariations,
    LightAnimations,
    OrientedCliffPrototypes,
    RotatedAnimationVariation,
    BendingType,
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
    WorkingVisualisation,
    RecipeTint,
    ShiftAnimationWaypoints,
    StatusColors,
    GuiMode,
    CreateDecorativesTriggerEffectItem,
    UnitSpawnDefinition,
    RotatedAnimation,
    AttackParameters,
    SmokeSource,
    HeatBuffer,
    ConnectableEntityGraphics,
    SignalColorMapping,
    GlowRenderMode,
    ForceCondition,
    MiningDrillGraphicsSet,
    OffshorePumpGraphicsSet,
    PipePictures,
    PipeToGroundPictures,
    Instrument,
    PumpConnectorGraphicsFluidWagon,
    RailPictures,
    SimpleEntityVisuals,
    SimpleEntityWithOwnerVisuals,
    SpiderLegGraphicsSet,
    StorageTankPictures,
    TrainStopLight,
    TrainStopDrawingBoxes,
    BeltAnimationSet,
    BeltGraphicsSet,
    BeltStructure,
    BeltStructureWithSideLoading,
    TransportBeltConnectorFrame,
    BeltAnimationSetIndexes,
    TreeVisuals,
    RotatedAnimation4Way,
    AnimatedVector,
    UnitAISettings,
    UnitAlternativeAttackingFrameSequence,
    SpiderVehicleGraphicsSet,
    SpiderEnginePrototype,
    WallPictures,
    FireFlameBurntPatchAlphaVariation,
    DamagePrototype,
    TextAlignment,
    CursorBoxType,
    EquipmentShape,
    DaytimeColorLookupTable
};

// TODO
// Current prototype definitions are based off Factorio wiki, which describe prototypes
// *definitions*, meaning how they are defined in lua code and how they are parsed.
// However, https://lua-api.factorio.com/ has a documentation on Lua*Prototype, which seems to have
// less prototype types than wiki. And as it turns out, runtime prototypes are less diverse than
// data-stage. For example, entire Entity abstract prtototype *and* it's subclasses are boiled down
// to LuaEntityPrototype, which has all the protperties (read-only).
// So, my idea is to implement all Lua*Prototype as legit structs and reuire all its subclasses
// (trait implementors) to be able to convert into it. Also, conversion may be done at `data` table
// parsing, as DataTable won't likely be used for reading specific prototype types (or maybe it
// will, I don't know). This simplifies handling of PrototypeReference and Prototype::find
// (DataTableAccessable::find ATM), as it would require only one HashMap to be checked. There are
// cases of PrototypeReference of type Entity (and iirc, there are no cases of referencing specific
// entity type). Another solution would be for Entity struct to have find() method just like all
// Prototypes that checks all its subclasses. Which isn't very efficient for simple lookup, but
// efficient for organization/structure.

/// Shorthand for prototype category/type, used in [DataTable]
pub type PrototypeCategory<T> = HashMap<String, Rc<T>>;

/// Struct representing global `data` table in lua environment
#[derive(Debug)]
pub struct DataTable {
    references: Vec<Rc<dyn PrototypeReferenceValidate>>,
    // Prototypes
    ambient_sound: PrototypeCategory<AmbientSoundPrototype>,
    animation: PrototypeCategory<Animation>,
    editor_controller: PrototypeCategory<EditorController>,
    font: PrototypeCategory<Font>,
    god_controller: PrototypeCategory<GodController>,
    map_gen_settings: PrototypeCategory<MapGenPresets>,
    map_settings: PrototypeCategory<MapSettings>,
    mouse_cursor: PrototypeCategory<MouseCursor>,
    sound: PrototypeCategory<Sound>,
    spectator_controller: PrototypeCategory<SpectatorController>,
    sprite: PrototypeCategory<Sprite>,
    tile_effect: PrototypeCategory<TileEffect>,
    tips_and_tricks_item_category: PrototypeCategory<TipsAndTricksItemCategory>,
    trigger_target_type: PrototypeCategory<TriggerTargetType>,
    wind_sound: PrototypeCategory<WindSound>,
    achievement: PrototypeCategory<Achievement>,
    build_entity_achievement: PrototypeCategory<BuildEntityAchievement>,
    combat_robot_count: PrototypeCategory<CombatRobotCountAchievement>,
    construct_with_robots_achevement: PrototypeCategory<ConstructWithRobotsAchievement>,
    deconstruct_with_robots_achievement: PrototypeCategory<DeconstructWithRobotsAchievement>,
    deliver_by_robots_achievement: PrototypeCategory<DeliverByRobotsAchievement>,
    dont_build_entity_achievement: PrototypeCategory<DontBuildEntityAchievement>,
    dont_craft_manually_achievement: PrototypeCategory<DontCraftManuallyAchievement>,
    dont_use_entity_in_energy_production_achievement: PrototypeCategory<DontUseEntityInEnergyProductionAchievement>,
    finish_the_game_achievement: PrototypeCategory<FinishTheGameAchievement>,
    group_attack_achievement: PrototypeCategory<GroupAttackAchievement>,
    kill_achievement: PrototypeCategory<KillAchievement>,
    player_damaged_achievement: PrototypeCategory<PlayerDamagedAchievement>,
    produce_achievement: PrototypeCategory<ProduceAchievement>,
    produce_per_hour_achievement: PrototypeCategory<ProducePerHourAchievement>,
    research_achievement: PrototypeCategory<ResearchAchievement>,
    train_path_achievement: PrototypeCategory<TrainPathAchievement>,
    ammo_category: PrototypeCategory<AmmoCategory>,
    autoplace_control: PrototypeCategory<AutoplaceControl>,
    custom_input: PrototypeCategory<CustomInput>,
    damage_type: PrototypeCategory<DamageType>,
    optimized_decorative: PrototypeCategory<Decorative>,
    arrow: PrototypeCategory<Arrow>,
    artillery_flare: PrototypeCategory<ArtilleryFlare>,
    artillery_projectile: PrototypeCategory<ArtilleryProjectile>,
    beam: PrototypeCategory<Beam>,
    character_corpse: PrototypeCategory<CharacterCorpse>,
    cliff: PrototypeCategory<Cliff>,
    corpse: PrototypeCategory<CorpsePrototype>,
    rail_remnants: PrototypeCategory<RailRemnants>,
    deconstructible_tile_proxy: PrototypeCategory<DeconstructibleTileProxy>,
    entity_ghost: PrototypeCategory<EntityGhost>,
    accumulator: PrototypeCategory<Accumulator>,
    artillery_turret: PrototypeCategory<ArtilleryTurret>,
    beacon: PrototypeCategory<Beacon>,
    boiler: PrototypeCategory<Boiler>,
    burner_generator: PrototypeCategory<BurnerGenerator>,
    character: PrototypeCategory<Character>,
    arithmetic_combinator: PrototypeCategory<ArithmeticCombinator>,
    decider_combinator: PrototypeCategory<DeciderCombinator>,
    constant_combinator: PrototypeCategory<ConstantCombinator>,
    container: PrototypeCategory<Container>,
    logistic_container: PrototypeCategory<LogisticContainer>,
    infinity_container: PrototypeCategory<InfinityContainer>,
    assembling_machine: PrototypeCategory<AssemblingMachine>,
    rocket_silo: PrototypeCategory<RocketSilo>,
    furnace: PrototypeCategory<Furnace>,
    electric_energy_interface: PrototypeCategory<ElectricEnergyInterface>,
    electric_pole: PrototypeCategory<ElectricPole>,
    unit_spawner: PrototypeCategory<EnemySpawner>,
    fish: PrototypeCategory<Fish>,
    combat_robot: PrototypeCategory<CombatRobot>,
    construction_robot: PrototypeCategory<ConstructionRobot>,
    logistic_robot: PrototypeCategory<LogisticRobot>,
    gate: PrototypeCategory<Gate>,
    generator: PrototypeCategory<Generator>,
    heat_interface: PrototypeCategory<HeatInterface>,
    heat_pipe: PrototypeCategory<HeatPipe>,
    inserter: PrototypeCategory<Inserter>,
    lab: PrototypeCategory<Lab>,
    lamp: PrototypeCategory<Lamp>,
    land_mine: PrototypeCategory<LandMine>,
    linked_container: PrototypeCategory<LinkedContainer>,
    market: PrototypeCategory<Market>,
    mining_drill: PrototypeCategory<MiningDrill>,
    offshore_pump: PrototypeCategory<OffshorePump>,
    pipe: PrototypeCategory<Pipe>,
    infinity_pipe: PrototypeCategory<InfinityPipe>,
    pipe_to_ground: PrototypeCategory<PipeToGround>,
    player_port: PrototypeCategory<PlayerPort>,
    power_switch: PrototypeCategory<PowerSwitch>,
    programmable_speaker: PrototypeCategory<ProgrammableSpeaker>,
    pump: PrototypeCategory<Pump>,
    radar: PrototypeCategory<Radar>,
    curved_rail: PrototypeCategory<CurvedRail>,
    straight_rail: PrototypeCategory<StraightRail>,
    rail_chain_signal: PrototypeCategory<RailChainSignal>,
    rail_signal: PrototypeCategory<RailSignal>,
    reactor: PrototypeCategory<Reactor>,
    roboport: PrototypeCategory<Roboport>,
    simple_entity: PrototypeCategory<SimpleEntity>,
    simple_entity_with_owner: PrototypeCategory<SimpleEntityWithOwner>,
    simple_entity_with_force: PrototypeCategory<SimpleEntityWithForce>,
    solar_panel: PrototypeCategory<SolarPanel>,
    spider_leg: PrototypeCategory<SpiderLeg>,
    storage_tank: PrototypeCategory<StorageTank>,
    train_stop: PrototypeCategory<TrainStop>,
    linked_belt: PrototypeCategory<LinkedBelt>,
    loader_1x1: PrototypeCategory<Loader1x1>,
    loader: PrototypeCategory<Loader1x2>,
    splitter: PrototypeCategory<Splitter>,
    transport_belt: PrototypeCategory<TransportBelt>,
    underground_belt: PrototypeCategory<UndergroundBelt>,
    tree: PrototypeCategory<Tree>,
    turret: PrototypeCategory<TurretPrototype>,
    ammo_turret: PrototypeCategory<AmmoTurret>,
    electric_turret: PrototypeCategory<ElectricTurret>,
    fluid_turret: PrototypeCategory<FluidTurret>,
    unit: PrototypeCategory<Unit>,
    car: PrototypeCategory<Car>,
    artillery_wagon: PrototypeCategory<ArtilleryWagon>,
    cargo_wagon: PrototypeCategory<CargoWagon>,
    fluid_wagon: PrototypeCategory<FluidWagon>,
    locomotive: PrototypeCategory<Locomotive>,
    spider_vehicle: PrototypeCategory<SpiderVehicle>,
    wall: PrototypeCategory<Wall>,
    explosion: PrototypeCategory<Explosion>,
    flame_thrower_explosion: PrototypeCategory<FlameThrowerExplosion>,
    fire: PrototypeCategory<FireFlame>,
    stream: PrototypeCategory<FluidStream>,
    flying_text: PrototypeCategory<FlyingText>,
    higlight_box: PrototypeCategory<HighlightBoxEntity>,
    item_entity: PrototypeCategory<ItemEntity>,
    item_request_proxy: PrototypeCategory<ItemRequestProxy>,
    particle_source: PrototypeCategory<ParticleSource>,
    projectile: PrototypeCategory<Projectile>,
    resource: PrototypeCategory<ResourceEntity>,
    rocket_silo_rocket: PrototypeCategory<RocketSiloRocket>,
    rocket_silo_rocket_shadow: PrototypeCategory<RocketSiloRocketShadow>,
    smoke_with_trigger: PrototypeCategory<SmokeWithTrigger>,
    speech_bubble: PrototypeCategory<SpeechBubble>,
    sticker: PrototypeCategory<Sticker>,
    tile_ghost: PrototypeCategory<TileGhost>,
    active_defense_equipment: PrototypeCategory<ActiveDefenseEquipment>,
    battery_equipment: PrototypeCategory<BatteryEquipment>,
    belt_immunity_equipment: PrototypeCategory<BeltImmunityEquipment>,
    energy_shield_equipment: PrototypeCategory<EnergyShieldEquipment>,
    generator_equipment: PrototypeCategory<GeneratorEquipment>,
    movement_bonus_equipment: PrototypeCategory<MovementBonusEquipment>,
    night_vision_equipment: PrototypeCategory<NightVisionEquipment>,
    roboport_equipment: PrototypeCategory<RoboportEquipment>,
    solar_panel_equipment: PrototypeCategory<SolarPanelEquipment>,
    /* Commented out until implemented
    equipment_category: PrototypeCategory<EquipmentCategory>,
    equipment_grid: PrototypeCategory<EquipmentGrid>,
    fluid: PrototypeCategory<Fluid>,
    fuel_category: PrototypeCategory<FuelCategory>,
    gui_style: PrototypeCategory<GuiStyle>,
    item: PrototypeCategory<Item>,
    ammo: PrototypeCategory<AmmoItem>,
    capsule: PrototypeCategory<Capsule>,
    gun: PrototypeCategory<Gun>,
    item_with_entity_data: PrototypeCategory<ItemWithEntityData>,
    entity_with_label: PrototypeCategory<ItemWithLabel>,
    item_with_inventory: PrototypeCategory<ItemWithInventory>,
    blueprint_book: PrototypeCategory<BlueprintBook>,
    item_with_tags: PrototypeCategory<ItemWithTags>,
    selection_tool: PrototypeCategory<SelectionTool>,
    blueprint: PrototypeCategory<BlueprintItem>,
    copy_paste_tool: PrototypeCategory<CopyPasteTool>,
    deconstruction_item: PrototypeCategory<DeconstructionItem>,
    upgrade_item: PrototypeCategory<UpgradeItem>,
    module: PrototypeCategory<Module>,
    rail_planner: PrototypeCategory<RailPlanner>,
    spidertron_remote: PrototypeCategory<SpidertronRemote>,
    tool: PrototypeCategory<Tool>,
    armor: PrototypeCategory<Armor>,
    repair_tool: PrototypeCategory<RepairTool>,
    item_group: PrototypeCategory<ItemGroup>,
    item_subgroup: PrototypeCategory<ItemSubGroup>,
    module_category: PrototypeCategory<ModuleCategory>,
    noise_expression: PrototypeCategory<NamedNoiseExpression>,
    noise_layer: PrototypeCategory<NoiseLayer>,
    optimized_particle: PrototypeCategory<Particle>,
    recipe: PrototypeCategory<Recipe>,
    recipe_category: PrototypeCategory<RecipeCategory>,
    resource_category: PrototypeCategory<ResourceCategory>,
    shortcut: PrototypeCategory<Shortcut>,
    technology: PrototypeCategory<Technology>,
    tile: PrototypeCategory<Tile>,
    tip_and_tricks_item: PrototypeCategory<TipsAndTricksItem>,
    trivial_smoke: PrototypeCategory<TrivialSmoke>,
    tutorial: PrototypeCategory<Tutorial>,
    utility_constants: PrototypeCategory<UnilityConstants>,
    utility_sounds: PrototypeCategory<UtilitySounds>,
    utility_sprites: PrototypeCategory<UtilitySprites>,
    virtual_signal: PrototypeCategory<VirtualSignal>,
    */
    bool_setting: PrototypeCategory<BoolModSetting>,
    int_setting: PrototypeCategory<IntModSetting>,
    double_setting: PrototypeCategory<DoubleModSetting>,
    string_setting: PrototypeCategory<StringModSetting>,
}

impl DataTable {
    /// Shorthand for [DataTableAccessable::find]
    pub fn find<T: DataTableAccessable>(&self, name: &String) -> Result<&Rc<T>, PrototypesErr> {
        T::find(self, name)
    }

    /// Shorthand for [DataTableAccessable::extend]
    pub fn extend<T: DataTableAccessable>(&self, prototype: T) -> Result<(), PrototypesErr> {
        prototype.extend(self)
    }

    /// Creates new reference and keeps track of it to later be validated through [Self::validate_references]
    pub fn new_reference<T: 'static + DataTableAccessable>(&mut self, name: String) -> Rc<PrototypeReference<T>> {
        let prot_reference = Rc::new(PrototypeReference::<T>::new(name));
        self.references.push(Rc::clone(&prot_reference) as Rc<dyn PrototypeReferenceValidate>);
        prot_reference
    }

    /// Validates all tracked references.
    pub fn validate_references(&self) -> Result<(), PrototypesErr> {
        for prot_reference in &self.references {
            prot_reference.validate(self)?
        }
        Ok(())
    }
}

/// [mlua::FromLua] alternative with [DataTable] reference being passed
pub trait PrototypeFromLua<'lua>: Sized {
    fn prrototype_from_lua(lua_value: Value<'lua>, lua: &'lua Lua, data_table: &DataTable) -> LuaResult<Self>;
}

/// Validate PrototypeReference. Any type.
trait PrototypeReferenceValidate: fmt::Debug {
    fn validate(&self, data_table: &DataTable) -> Result<(), PrototypesErr>;
}

/// Reference to a prototype by name.
#[derive(Debug)]
pub struct PrototypeReference<T: DataTableAccessable> {
    name: String,
    pub prototype: Option<Rc<T>>
}

impl<T: DataTableAccessable> PrototypeReference<T> {
    /// Creates new unresolved Prototype reference
    pub fn new(name: String) -> Self {
        Self{name, prototype: None}
    }

    /// Tries to resolve prototype and remember it. Errors if prototype is not found
    pub fn resolve(&mut self, data_table: &DataTable) -> Result<(), PrototypesErr> {
        self.prototype = Some(data_table.find::<T>(&self.name)?.clone());
        Ok(())
    }

    /// Checks if reference is valid. Always false if [`resolve`](Self::resolve) was never called
    pub fn is_valid(&self) -> bool {
        self.prototype.is_some()
    }
}

impl<T: DataTableAccessable> PrototypeReferenceValidate for PrototypeReference<T> {
    /// Validates the reference
    fn validate(&self, data_table: &DataTable) -> Result<(), PrototypesErr> {
        data_table.find::<T>(&self.name).map(|_| ())
    }
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

/// Trait for manipulating prototypes in [Data table](DataTable).
/// Primarily used for [`PrototypeReference`]
pub trait DataTableAccessable: Prototype {
    /// Find prototype in [Data table](DataTable) by it's name
    fn find<'a>(data_table: &'a DataTable, name: &String) -> Result<&'a Rc<Self>, PrototypesErr> where Self: Sized;
    /// Extend [Data table](DataTable) with this prototype
    fn extend(self, data_table: &DataTable) -> Result<(), PrototypesErr>;
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

/// <https://wiki.factorio.com/Prototype/AmbientSound>
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

/// <https://wiki.factorio.com/Prototype/Animation>
#[derive(Debug, Prototype)]
pub struct AnimationPrototype {
    name: String,
    layers: Vec<Animation> // If lua table doesn't have layers, use same table for constructing just one
}

/// <https://wiki.factorio.com/Prototype/EditorController>
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

/// <https://wiki.factorio.com/Prototype/Font>
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

/// <https://wiki.factorio.com/Prototype/GodController>
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

/// <https://wiki.factorio.com/Prototype/MapGenPresets>
#[derive(Debug, Prototype)]
pub struct MapGenPresets {
    name: String,
    presets: HashMap<String, MapGenPreset>
}

/// <https://wiki.factorio.com/Prototype/MapSettings>
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

/// <https://wiki.factorio.com/Prototype/MouseCursor>
#[derive(Debug, Prototype)]
pub struct MouseCursor {
    name: String,
    cursor: MouseCursorType
}

/// <https://wiki.factorio.com/Prototype/Sound>
#[derive(Debug, Prototype)]
pub struct SoundPrototype {
    name: String,
    sound: Sound
}

/// <https://wiki.factorio.com/Prototype/SpectatorController>
#[derive(Debug, Prototype)]
pub struct SpectatorController {
    name: String, // Must be "default"
    movement_speed: f64 // Must be >= 0.34375
}

/// <https://wiki.factorio.com/Prototype/Sprite>
#[derive(Debug, Prototype)]
pub struct SpritePrototype {
    name: String,
    sprite: Sprite
}

/// <https://wiki.factorio.com/Prototype/TileEffect>
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

/// <https://wiki.factorio.com/Prototype/TipsAndTricksItemCategory>
#[derive(Debug, Prototype)]
pub struct TipsAndTricksItemCategory {
    name: String,
    order: String
}

/// <https://wiki.factorio.com/Prototype/TriggerTargetType>
#[derive(Debug, Prototype)]
pub struct TriggerTargetType {
    name: String
}

/// <https://wiki.factorio.com/Prototype/WindSound>
#[derive(Debug, Prototype)]
pub struct WindSound {
    name: String,
    sound: Sound
}

// PrototypeBase starts here
/// <https://wiki.factorio.com/PrototypeBase>
#[derive(Debug)]
pub struct PrototypeBaseSpec {
    localised_description: Option<LocalisedString>,
    localised_name: Option<LocalisedString>,
    order: String
}

/// <https://wiki.factorio.com/PrototypeBase>
pub trait PrototypeBase: Prototype {
    fn localised_description(&self) -> &Option<LocalisedString>;
    fn localised_name(&self) -> &Option<LocalisedString>;
    fn order(&self) -> &String; // Default: ""
}

/// Base for Achievement and all inherited types <https://wiki.factorio.com/Prototype/Achievement>
#[derive(Debug)]
pub struct AchievementBase {
    icon: IconSpecification,
    steam_stats_name: String, // Default: "" // Unusable
    allowed_without_fight: bool, // Default: true
    hidden: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/Achievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct Achievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase
}

/// <https://wiki.factorio.com/Prototype/BuildEntityAchievement>
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

/// <https://wiki.factorio.com/Prototype/CombatRobotCountAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct CombatRobotCountAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    count: u32 // Default: 1
}

/// <https://wiki.factorio.com/Prototype/ConstructWithRobotsAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct ConstructWithRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    limited_to_one_game: bool,
    amount: u32, // Default: 0
    more_than_manually: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/DeconstructWithRobotsAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct DeconstructWithRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: u32
}

/// <https://wiki.factorio.com/Prototype/DeliverByRobotsAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct DeliverByRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64
}

/// <https://wiki.factorio.com/Prototype/DontBuildEntityAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct DontBuildEntityAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    dont_buid: Vec<String>, // String is converted to Vec<String> with one element
    amount: u32 // Default: 0
}

/// <https://wiki.factorio.com/Prototype/DontCraftManuallyAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct DontCraftManuallyAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64
}

/// <https://wiki.factorio.com/Prototype/DontUseEntityInEnergyProductionAchievement>
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

/// <https://wiki.factorio.com/Prototype/FinishTheGameAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct FinishTheGameAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    until_second: u32 // Default: 0 (means infinite)
}

/// <https://wiki.factorio.com/Prototype/GroupAttackAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct GroupAttackAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: u32 // Default: 1
}

/// <https://wiki.factorio.com/Prototype/KillAchievement>
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

/// <https://wiki.factorio.com/Prototype/PlayerDamagedAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct PlayerDamagedAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    minimum_damage: f32,
    should_survive: bool,
    type_of_dealer: Option<String> // TODO: another prototype enum?
}

/// <https://wiki.factorio.com/Prototype/ProduceAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct ProduceAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64,
    limited_to_one_game: bool,
    product: ProductType // Type is determined from item_product or fluid_product // Only one can be set!
}

/// <https://wiki.factorio.com/Prototype/ProducePerHourAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct ProducePerHourAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64,
    product: ProductType
}

/// <https://wiki.factorio.com/Prototype/ResearchAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct ResearchAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    target: ResearchTarget // Determined from either `technology` or `research_all` is set
}

/// <https://wiki.factorio.com/Prototype/TrainPathAchievement>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct TrainPathAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    minimum_distance: f64
}

/// <https://wiki.factorio.com/Prototype/AmmoCategory>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct AmmoCategory {
    name: String,
    prototype_base: PrototypeBaseSpec,
    bonus_gui_order: String // Default: ""
}

/// <https://wiki.factorio.com/Prototype/AutoplaceControl>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct AutoplaceControl {
    name: String,
    prototype_base: PrototypeBaseSpec,
    category: AutoplaceControlCategory,
    rechness: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/CustomInput>
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

/// <https://wiki.factorio.com/Prototype/DamageType>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct DamageType {
    name: String,
    prototype_base: PrototypeBaseSpec,
    hidden: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/Decorative>
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

/// <https://wiki.factorio.com/Prototype/Entity>
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

/// <https://wiki.factorio.com/Prototype/Entity>
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

/// <https://wiki.factorio.com/Prototype/Arrow>
#[derive(Debug, Prototype, Entity)]
pub struct Arrow {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    arrow_picture: Sprite,
    circle_picture: Option<Sprite>,
    blinking: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/ArtilleryFlare>
#[derive(Debug, Prototype, Entity)]
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

/// <https://wiki.factorio.com/Prototype/ArtilleryProjectile>
#[derive(Debug, Prototype, Entity)]
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

/// <https://wiki.factorio.com/Prototype/Beam>
#[derive(Debug, Prototype, Entity)]
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

/// <https://wiki.factorio.com/Prototype/CharacterCorpse>
#[derive(Debug, Prototype, Entity)]
pub struct CharacterCorpse {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    time_to_live: u32,
    render_layer: RenderLayer, // Default: "object"
    pictures: Vec<AnimationVariation>, // Mandatory // picture field is converted to this
    armor_picture_mapping: HashMap<String, usize> // Exact type of animation index is unknown, it references index in pictures field
}

/// <https://wiki.factorio.com/Prototype/Cliff>
#[derive(Debug, Prototype, Entity)]
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

/// <https://wiki.factorio.com/Prototype/Corpse>
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

/// <https://wiki.factorio.com/Prototype/Corpse>
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

/// <https://wiki.factorio.com/Prototype/Corpse>
#[derive(Debug, Prototype, Corpse)]
pub struct CorpsePrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    corpse_base: CorpseBase
}

/// <https://wiki.factorio.com/Prototype/RailRemnants>
#[derive(Debug, Prototype, Corpse)]
pub struct RailRemnants {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    corpse_base: CorpseBase,
    bending_type: BendingType,
    pictures: RailPictures
}

/// <https://wiki.factorio.com/Prototype/DeconstructibleTileProxy>
#[derive(Debug, Prototype, Entity)]
pub struct DeconstructibleTileProxy {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
}

/// <https://wiki.factorio.com/Prototype/EntityGhost>
#[derive(Debug, Prototype, Entity)]
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

/// <https://wiki.factorio.com/Prototype/Accumulator>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Accumulator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/ArtilleryTurret>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct ArtilleryTurret {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/Beacon>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Beacon {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/Boiler>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Boiler {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/BurnerGenerator>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct BurnerGenerator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/Character>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Character {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/Combinator>
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

/// <https://wiki.factorio.com/Prototype/Combinator>
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

/// <https://wiki.factorio.com/Prototype/ArithmeticCombinator>
#[derive(Debug, Prototype, Combinator)]
pub struct ArithmeticCombinator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/DeciderCombinator>
#[derive(Debug, Prototype, Combinator)]
pub struct DeciderCombinator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    combinator_base: CombinatorBase,
    equal_symbol_sprites: Sprite4Way,
    greater_symbol_sprites: Sprite4Way,
    less_symbol_sprites: Sprite4Way,
    not_equal_symbol_sprites: Sprite4Way,
    greater_or_equal_symbol_sprites: Sprite4Way,
    less_or_equal_symbol_sprites: Sprite4Way,
}

/// <https://wiki.factorio.com/Prototype/ConstantCombinator>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct ConstantCombinator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/Container>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Container {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/LogisticContainer>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct LogisticContainer {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/InfinityContainer>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct InfinityContainer {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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
    gui_mode: GuiMode // Default: "none"
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine>
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
    default_recipe_tint: Option<RecipeTint>,
    shift_animation_waypoints: Option<ShiftAnimationWaypoints>, // Only loaded if `shift_animation_waypoint_stop_duration` or `shift_animation_transition_duration` is not 0
    shift_animation_waypoint_stop_duration: u16, // Default: 0 // Only loaded if `shift_animation_waypoints` is present
    shift_animation_transition_duration: u16, // Default: 0 // Only loaded if `shift_animation_waypoints` is present
    status_colors: Option<StatusColors>,
    entity_info_icon_shift: Factorio2DVector, // Default: {0, -0.3} for 
    draw_entity_info_icon_background: bool, // Default: true
    match_animation_speed_to_activity: bool, // Default: false
    show_recipe_icon_on_map: bool, // Default: true
    base_productivity: f32, // Default: 0
    module_specification: Option<ModuleSpecification>,
    working_visualisations: Option<Vec<WorkingVisualisation>>,
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine>
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
    fn default_recipe_tint(&self) -> &Option<RecipeTint>;
    fn shift_animation_waypoints(&self) -> &Option<ShiftAnimationWaypoints>;
    fn shift_animation_waypoint_stop_duration(&self) -> u16;
    fn shift_animation_transition_duration(&self) -> u16;
    fn status_colors(&self) -> &Option<StatusColors>;
    fn entity_info_icon_shift(&self) -> Factorio2DVector;
    fn draw_entity_info_icon_background(&self) -> bool;
    fn match_animation_speed_to_activity(&self) -> bool;
    fn show_recipe_icon_on_map(&self) -> bool;
    fn base_productivity(&self) -> f32;
    fn module_specification(&self) -> &Option<ModuleSpecification>;
    fn working_visualisations(&self) -> &Option<Vec<WorkingVisualisation>>;
}

/// <https://wiki.factorio.com/Prototype/AssemblingMachine>
#[derive(Debug, Prototype, CraftingMachine)]
pub struct AssemblingMachine {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    crafting_machine_base: CraftingMachineBase,
    fixed_recipe: String, // Default: "" // Name of Recipe
    gui_title_key: String, // Default: ""
    ingredient_count: u8, // Default: 255
}

/// <https://wiki.factorio.com/Prototype/RocketSilo>
#[derive(Debug, Prototype, CraftingMachine)]
pub struct RocketSilo {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
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

/// <https://wiki.factorio.com/Prototype/Furnace>
#[derive(Debug, Prototype, CraftingMachine)]
pub struct Furnace {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    crafting_machine_base: CraftingMachineBase,
    result_inventory_size: u16,
    source_inventory_size: u16 // Not more than 1
}

/// <https://wiki.factorio.com/Prototype/ElectricEnergyInterface>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct ElectricEnergyInterface {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    energy_source: EnergySource, // Must be electric
    energy_production: Energy, // Default: 0
    energy_usage: Energy, // Default: 0
    gui_mode: GuiMode, // Default: "none"
    continuous_animation: bool, // Default: false
    render_layer: RenderLayer, // Default: "object"
    light: Option<LightDefinition>,
    visuals: ElectricEnergyInterfaceVisuals
}

/// <https://wiki.factorio.com/Prototype/ElectricEnergyInterface#picture>
#[derive(Debug)]
pub enum ElectricEnergyInterfaceVisuals {
    Picture(Sprite),
    Pictures(Sprite4Way),
    Animation(Animation),
    Animations(Animation4Way)
}

/// <https://wiki.factorio.com/Prototype/ElectricPole>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct ElectricPole {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pictures: RotatedSprite,
    supply_area_distance: f64, // Max value: 64
    connection_points: Vec<WireConnectionPoint>,
    radius_visualisation_picture: Option<Sprite>,
    active_picture: Option<Sprite>,
    maximum_wire_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    light: Option<LightDefinition>,
    track_coverage_during_build_by_moving: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/EnemySpawner>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct EnemySpawner {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    animations: Vec<AnimationVariation>,
    max_count_of_owned_units: u32,
    max_friends_around_to_spawn: u32,
    spawning_cooldown: [f64; 2],
    spawning_radius: f64,
    spawning_spacing: f64,
    max_richness_for_spawn_shift: f64,
    max_spawn_shift: f64,
    pollution_absorption_absolute: f64,
    pollution_absorption_proportional: f64,
    call_for_help_radius: f64,
    result_units: Vec<UnitSpawnDefinition>,
    dying_sound: Option<Sound>,
    integration: Vec<SpriteVariation>,
    min_darkness_to_spawn: f32, // Default: 0.0
    max_darkness_to_spawn: f32, // Default: 1.0
    random_animation_offset: bool, // Default: true
    spawn_decorations_on_expansion: bool, // Default: false
    spawn_decoration: Vec<CreateDecorativesTriggerEffectItem>
}

/// <https://wiki.factorio.com/Prototype/Fish>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Fish {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pictures: Vec<SpriteVariation>
}

/// <https://wiki.factorio.com/Prototype/FlyingRobot>
#[derive(Debug)]
pub struct FlyingRobotBase {
    speed: f64,
    max_speed: f64, // Default: max double
    max_energy: Energy, // Default: 0
    energy_per_move: Energy, // Default: 0
    energy_per_tick: Energy, // Default: 0
    min_to_charge: f32, // Default: 0.2
    max_to_charge: f32, // Default: 0.95
    speed_multiplier_when_out_of_energy: f32, // Default: 0
}

/// <https://wiki.factorio.com/Prototype/FlyingRobot>
pub trait FlyingRobot {
    fn speed(&self) -> f64;
    fn max_speed(&self) -> f64;
    fn max_energy(&self) -> Energy;
    fn energy_per_move(&self) -> Energy;
    fn energy_per_tick(&self) -> Energy;
    fn min_to_charge(&self) -> f32;
    fn max_to_charge(&self) -> f32;
    fn speed_multiplier_when_out_of_energy(&self) -> f32;
}

/// <https://wiki.factorio.com/Prototype/CombatRobot>
#[derive(Debug, Prototype, FlyingRobot)]
pub struct CombatRobot {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    flying_robot_base: FlyingRobotBase,
    time_to_live: u32,
    attack_parameters: AttackParameters,
    idle: RotatedAnimation,
    shadow_idle: RotatedAnimation,
    in_motion: RotatedAnimation,
    shadow_in_motion: RotatedAnimation,
    range_from_player: f64, // Default: 0
    friction: f64, // Default: 0
    destroy_action: Option<Trigger>,
    follows_player: bool, // Default: false
    light: Option<LightDefinition>
}

/// <https://wiki.factorio.com/Prototype/ConstructionRobot>
#[derive(Debug, Prototype, FlyingRobot)]
pub struct ConstructionRobot {
    // Must have collision box of zero
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    flying_robot_base: FlyingRobotBase,
    // RobotWithLogisticInterface
    max_payload_size: u32,
    cargo_centered: Factorio2DVector,
    idle: Option<RotatedAnimation>,
    in_motion: Option<RotatedAnimation>,
    shadow_idle: Option<RotatedAnimation>,
    shadow_in_motion: Option<RotatedAnimation>,
    destroy_action: Option<Trigger>,
    draw_cargo: bool, // Default: true
    // ConstructionRobot
    construction_vector: Factorio2DVector,
    working: Option<RotatedAnimation>,
    shadow_working: Option<RotatedAnimation>,
    smoke: Option<Animation>,
    sparks: Option<Vec<AnimationVariation>>,
    repairing_sound: Option<Sound>,
    working_light: Option<LightDefinition>
}

/// <https://wiki.factorio.com/Prototype/LogisticRobot>
#[derive(Debug, Prototype, FlyingRobot)]
pub struct LogisticRobot {
    // Must have collision box of zero
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    flying_robot_base: FlyingRobotBase,
    // RobotWithLogisticInterface
    max_payload_size: u32,
    cargo_centered: Factorio2DVector,
    idle: Option<RotatedAnimation>,
    in_motion: Option<RotatedAnimation>,
    shadow_idle: Option<RotatedAnimation>,
    shadow_in_motion: Option<RotatedAnimation>,
    destroy_action: Option<Trigger>,
    draw_cargo: bool, // Default: true
    // LogisticRobot
    idle_with_cargo: Option<RotatedAnimation>,
    in_motion_with_cargo: Option<RotatedAnimation>,
    shadow_idle_with_cargo: Option<RotatedAnimation>,
    shadow_in_motion_with_cargo: Option<RotatedAnimation>
}

/// <https://wiki.factorio.com/Prototype/Gate>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Gate {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    vertical_animation: Animation,
    horizontal_animation: Animation,
    vertical_rail_animation_left: Animation,
    vertical_rail_animation_right: Animation,
    horizontal_rail_animation_left: Animation,
    horizontal_rail_animation_right: Animation,
    vertical_rail_base: Animation,
    horizontal_rail_base: Animation,
    wall_patch: Animation,
    opening_speed: f32,
    activation_distance: f64,
    timeout_to_close: u32,
    open_sound: Sound,
    close_sound: Sound,
    fadeout_interval: u32, // Default: 0
    opened_collision_mask: CollisionMask // Default: ["object-layer", "item-layer", "floor-layer", "water-tile"]
}

/// <https://wiki.factorio.com/Prototype/Generator>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Generator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    energy_source: EnergySource, // Must be electric
    fluid_box: FluidBox,
    horizontal_animation: Animation,
    vertical_animation: Animation,
    effectivity: f64,
    fluid_usage_per_tick: f64,
    maximum_temperature: f64,
    smoke: Option<Vec<SmokeSource>>, // 1 or more, if specified
    burns_fluid: bool, // Default: false
    scale_fluid_usage: bool, // Default: false
    min_perceived_performance: f64, // Default: 0.25
    performance_to_sound_speedup: f64, // Default: 0.5
    max_power_output: Option<Energy>
}

/// <https://wiki.factorio.com/Prototype/HeatInterface>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct HeatInterface {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    heat_buffer: HeatBuffer,
    picture: Option<Sprite>,
    guid_mode: GuiMode, // Default: "all"
}

/// <https://wiki.factorio.com/Prototype/HeatPipe>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct HeatPipe {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    connection_sprites: ConnectableEntityGraphics,
    heat_glow_sprites: ConnectableEntityGraphics,
    heat_buffer: HeatBuffer
}

/// <https://wiki.factorio.com/Prototype/Inserter>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Inserter {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    extension_speed: f64,
    rotation_speed: f64,
    insert_position: Factorio2DVector,
    pickup_position: Factorio2DVector,
    platform_picture: Sprite4Way,
    hand_base_picture: Sprite,
    hand_open_picture: Sprite,
    hand_closed_picture: Sprite,
    energy_source: EnergySource, // Emissions are ignored
    energy_per_movement: Energy, // Default: 0
    energy_per_rotation: Energy, // Default: 0
    stack: bool, // Default: false
    allow_custom_vectors: bool, // Default: false
    allow_burner_leech: bool, // Default: false
    draw_held_item: bool, // Default: true
    use_easter_egg: bool, // Default: true
    filter_count: u8, // Default: 0
    hand_base_shadow: Option<Sprite>,
    hand_open_shadow: Option<Sprite>,
    hand_closed_shadow: Option<Sprite>,
    hand_size: f64, // Default: 0.75
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    default_stack_control_input_signal: Option<SignalIDConnector>,
    draw_inserter_arrow: bool, // Default: true
    chases_belt_items: bool, // Default: true
    stack_size_bonus: u32, // Default: 0
    circuit_wire_connection_points: Option<Vec<WireConnectionPoint>>,
    circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>
}

/// <https://wiki.factorio.com/Prototype/Lab>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Lab {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    energy_usage: Energy,
    energy_source: EnergySource,
    on_animation: Animation,
    off_animation: Animation,
    inputs: Vec<String>, // (Names) Name of science pack items
    researching_speed: f64, // Default: 1
    allowed_effects: EffectTypeLimitation, // Default: all allowed
    light: Option<LightDefinition>,
    base_productivity: f32, // Default: 0
    entity_info_icon_shift: Factorio2DVector, // Default: (0, 0)
    module_specification: Option<ModuleSpecification>
}

/// <https://wiki.factorio.com/Prototype/Lamp>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Lamp {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    picture_on: Sprite,
    picture_off: Sprite,
    energy_usage_per_tick: Energy,
    energy_source: EnergySource, // Must be electric or void, emissions are ignored
    light: Option<LightDefinition>,
    light_when_colored: Option<LightDefinition>,
    circuit_wire_connection_point: Option<WireConnectionPoint>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_connector_sprites: Option<CircuitConnectorSprites>,
    glow_size: f32, // Default: 0
    glow_color_intensity: f32, // Default: 0
    darkness_for_all_lamps_on: f32, // Default: 0.5
    darkness_for_all_lamps_off: f32, // Default: 0.3
    always_on: bool, // Default: false
    signal_to_color_mapping: Option<Vec<SignalColorMapping>>,
    glow_render_mode: GlowRenderMode // Default: "additive"
}

/// <https://wiki.factorio.com/Prototype/LandMine>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct LandMine {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    picture_safe: Sprite,
    picture_set: Sprite,
    trigger_radius: f64,
    picture_set_enemy: Option<Sprite>,
    timeout: u32, // Default: 120
    action: Option<Trigger>,
    ammo_category: Option<String>, // Name of AmmoCategory
    force_die_on_attack: bool, // Default: true
    trigger_force: ForceCondition // Default: "enemy"
}

/// <https://wiki.factorio.com/Prototype/LinkedContainer>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct LinkedContainer {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    inventory_size: u16, // Must be >0
    picture: Option<Sprite>,
    gui_mode: GuiMode, // Default: "all"
    scale_info_icons: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/Market>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Market {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    picture: Sprite,
    allow_access_to_all_forces: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/MiningDrill>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct MiningDrill {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    vector_to_place_result: Factorio2DVector,
    resource_searching_radius: f64,
    energy_usage: Energy,
    mining_speed: f64,
    energy_source: EnergySource,
    resource_categories: Vec<String>, // (Names) Name of resourceCategory
    output_fluid_box: Option<FluidBox>,
    input_fluid_box: Option<FluidBox>,
    animations: Option<Animation4Way>, // Loaded only if `graphics_set` is not present
    graphics_set: Option<MiningDrillGraphicsSet>,
    wet_mining_graphics_set: Option<MiningDrillGraphicsSet>,
    base_picture: Option<Sprite4Way>,
    allowed_effects: EffectTypeLimitation, // Default: all allowed
    radius_visualisation_picture: Option<Sprite>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    base_render_layer: RenderLayer, // Default: "lower-object"
    base_productivity: f32, // Default: 0
    monitor_visualization_tint: Option<Color>,
    circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory iff `circuit_wire_max_distance` > 0
    module_specification: Option<ModuleSpecification>
}

/// <https://wiki.factorio.com/Prototype/OffshorePump>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct OffshorePump {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    fluid_box: FluidBox,
    pumping_speed: f32, // Must be > 0
    fluid: String, // Name of Fluid
    graphics_set: Option<OffshorePumpGraphicsSet>, // Mandatory if `picture` is not defined
    picture: Option<Sprite4Way>, // Deprecated
    min_perceived_performance: f32, // Default: 0.25
    fluid_box_tile_collision_test: CollisionMask, // Default: "ground-tile"
    adjacent_tile_collision_test: CollisionMask, // Defauylt: "water-tile"
    adjacent_tile_collision_mask: CollisionMask, // Default: none
    center_collision_mask: CollisionMask, // Default: none
    adjacent_tile_collision_box: BoundingBox, // Default: ((-0.05, -0.8), (0.05, -0.7))
    placeable_position_visualization: Option<Sprite>,
    remove_on_tile_collision: bool, // Default: false
    always_draw_fluid: bool, // Default: true
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    circuit_connector_sprites: Vec<CircuitConnectorSprites> // Mandatory if `circuit_wire_max_distance` > 0
}

/// <https://wiki.factorio.com/Prototype/Pipe>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Pipe {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    fluid_box: FluidBox,
    horizontal_window_bounding_box: BoundingBox,
    vertical_window_bounding_box: BoundingBox,
    pictures: PipePictures
}

/// <https://wiki.factorio.com/Prototype/InfinityPipe>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct InfinityPipe {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    fluid_box: FluidBox,
    horizontal_window_bounding_box: BoundingBox,
    vertical_window_bounding_box: BoundingBox,
    pictures: PipePictures,
    gui_mode: GuiMode
}

/// <https://wiki.factorio.com/Prototype/PipeToGround>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct PipeToGround {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    fluid_box: FluidBox,
    pictures: PipeToGroundPictures,
    draw_fluid_icon_override: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/PlayerPort>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct PlayerPort {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    animation: Animation
}

/// <https://wiki.factorio.com/Prototype/PowerSwitch>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct PowerSwitch {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    power_on_animation: Animation,
    overlay_start: Animation,
    overlay_loop: Animation,
    led_on: Sprite,
    led_off: Sprite,
    overlay_start_delay: u8,
    circuit_wire_connection_point: WireConnectionPoint,
    left_wire_connection_point: WireConnectionPoint,
    right_wire_connection_point: WireConnectionPoint,
    wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/ProgrammableSpeaker>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct ProgrammableSpeaker {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    energy_source: EnergySource, // Must be electric
    energy_usage_per_tick: Energy,
    sprite: Sprite,
    maximum_polyphony: u32,
    instruments: Vec<Instrument>,
    audible_distance_modifier: f32, // Default: 1
    circuit_wire_connection_point: Option<WireConnectionPoint>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_connector_sprites: Option<CircuitConnectorSprites>
}

/// <https://wiki.factorio.com/Prototype/Pump>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Pump {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    fluid_box: FluidBox,
    energy_source: EnergySource,
    energy_usage: Energy,
    pumping_speed: f64,
    animations: Animation4Way,
    fluid_wagon_connector_speed: f64, // Default: 1 / 64.0
    fluid_wagon_connector_alignment_tolerance: f64, // Default: 2 / 32.0
    fluid_wagon_connector_frame_count: u8, // Default: 1
    fluid_animation: Option<Animation4Way>,
    glass_pictures: Option<Sprite4Way>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory if `circuit_wire_max_distance` > 0
    fluid_wagon_connector_graphics: PumpConnectorGraphicsFluidWagon
}

/// <https://wiki.factorio.com/Prototype/Radar>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Radar {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    energy_usage: Energy,
    energy_per_sector: Energy,
    energy_per_nearby_scan: Energy,
    energy_source: EnergySource,
    pictures: RotatedSprite,
    max_distance_of_sector_revealed: u32,
    max_distance_of_nearby_sector_revealed: u32,
    radius_minimap_visualisation_color: Option<Color>,
    rotation_speed: f64, // Default: 0.01
}

/// <https://wiki.factorio.com/Prototype/CurvedRail>
/// <https://wiki.factorio.com/Prototype/Rail>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct CurvedRail {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pictures: RailPictures,
    walking_sound: Option<Sound>,
    bending_type: BendingType // Must be "turn"
}

/// <https://wiki.factorio.com/Prototype/StraightRail>
/// <https://wiki.factorio.com/Prototype/Rail>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct StraightRail {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pictures: RailPictures,
    walking_sound: Option<Sound>,
    bending_type: BendingType // Must be "straight"
}

/// `collision_box` is hardcoded to ((-0.2, -0.2), (0.2, 0.2))
/// "placeable-off-grid" flag is ignored
/// Rail signals must collide with each other
/// <https://wiki.factorio.com/Prototype/RailChainSignal>
/// <https://wiki.factorio.com/Prototype/RailSignalBase>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct RailChainSignal {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    animation: RotatedAnimation,
    rail_piece: Option<Animation>,
    green_light: Option<LightDefinition>,
    orange_light: Option<LightDefinition>,
    red_light: Option<LightDefinition>,
    default_red_output_signal: Option<SignalIDConnector>,
    default_orange_output_signal: Option<SignalIDConnector>,
    default_green_output_signal: Option<SignalIDConnector>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory if `circuit_wire_max_distance` > 0
    selection_box_offsets: [Factorio2DVector; 8],
    blue_light: Option<LightDefinition>,
    default_blue_output_signal: Option<SignalIDConnector>
}

/// `collision_box` is hardcoded to ((-0.2, -0.2), (0.2, 0.2))
/// "placeable-off-grid" flag is ignored
/// Rail signals must collide with each other
/// <https://wiki.factorio.com/Prototype/RailSignal>
/// <https://wiki.factorio.com/Prototype/RailSignalBase>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct RailSignal {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    animation: RotatedAnimation,
    rail_piece: Option<Animation>,
    green_light: Option<LightDefinition>,
    orange_light: Option<LightDefinition>,
    red_light: Option<LightDefinition>,
    default_red_output_signal: Option<SignalIDConnector>,
    default_orange_output_signal: Option<SignalIDConnector>,
    default_green_output_signal: Option<SignalIDConnector>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory if `circuit_wire_max_distance` > 0
}

/// <https://wiki.factorio.com/Prototype/Reactor>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Reactor {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    working_light_picture: Sprite,
    heat_buffer: HeatBuffer,
    energy_source: EnergySource,
    consumption: Energy,
    // If defined, Number of variations must be >= count of connections defined in `heat_buffer`
    connection_patches_connected: Option<SpriteVariations>,
    connection_patches_disconnected: Option<SpriteVariations>,
    heat_connection_patches_connected: Option<SpriteVariations>,
    heat_connection_patches_disconnected: Option<SpriteVariations>,
    lower_layer_picture: Option<Sprite>,
    heat_lower_layer_picture: Option<Sprite>,
    picture: Option<Sprite>,
    light: Option<LightDefinition>,
    meltdown_action: Option<Trigger>,
    neighbour_bonus: f64, // Default: 1
    neighbour_collision_increase: f64, // Default: 0.25 // Can't be negative
    scale_energy_usage: bool, // Default: false
    use_fuel_glow_color: bool, // Default: false
    default_fuel_glow_color: Color, // Default: (1, 1, 1, 1)
}

/// <https://wiki.factorio.com/Prototype/Roboport>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Roboport {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    energy_source: EnergySource, // Must be electric or void
    energy_usage: Energy,
    recharge_minimum: Energy,
    robot_slots_count: ItemStackIndex,
    material_slots_count: ItemStackIndex,
    base: Sprite,
    base_patch: Sprite,
    base_animation: Animation,
    door_animation_up: Animation,
    door_animation_down: Animation,
    request_to_open_door_timeout: u32,
    recharging_animation: Animation,
    spawn_and_station_height: f32,
    charge_approach_distance: f32,
    logistics_radius: f32, // Can't be negative
    construction_radius: f32, // Can'be negative
    charging_energy: Energy,
    open_door_trigger_effect: Option<TriggerEffect>,
    close_door_trigger_effect: Option<TriggerEffect>,
    default_available_logistic_output_signal: Option<SignalIDConnector>,
    default_total_logistic_output_signal: Option<SignalIDConnector>,
    default_available_construction_output_signal: Option<SignalIDConnector>,
    default_total_construction_output_signal: Option<SignalIDConnector>,
    circuit_wire_connection_point: Option<WireConnectionPoint>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_connector_sprites: Option<CircuitConnectorSprites>,
    spawn_and_station_shadow_height_offset: f32, // Default: 0
    draw_logistic_radius_visualization: bool, // Default: true
    draw_construction_radius_visualization: bool, // Default: true
    recharging_light: Option<LightDefinition>,
    charging_station_count: u32, // Default: 0
    charging_distance: f32, // Default: 0
    charging_station_shift: Option<Factorio2DVector>,
    charging_threshold_distance: f32, // Default: 1
    robot_vertical_acceleration: f32, // Default: 0.01
    stationing_offset: Option<Factorio2DVector>,
    robot_limit: ItemCountType, // Default: u32::MAX
    robots_shrink_when_entering_and_exiting: bool, // Default: false
    charging_offsets: Option<Vec<Factorio2DVector>>,
    logistics_connection_distance: Option<f32> // Must be >= `logistics_radius`
}

/// <https://wiki.factorio.com/Prototype/SimpleEntity>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct SimpleEntity {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    count_as_rock_for_filtered_deconstruction: bool, // Default: false
    render_layer: RenderLayer, // Default: "object"
    secondary_draw_order: i8, // Default: 0
    random_animation_offset: bool, // Default: false
    random_variation_on_create: bool, // Default: true
    visuals: SimpleEntityVisuals
}

/// <https://wiki.factorio.com/Prototype/SimpleEntityWithOwner>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct SimpleEntityWithOwner {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    render_layer: RenderLayer, // default: "object"
    secondary_draw_order: i8, // Default: 0
    random_animation_offset: bool, // Default: false
    random_variation_on_create: bool, // Default: true
    visuals: SimpleEntityWithOwnerVisuals,
    force_visibility: ForceCondition, // Default: "all"
}

/// <https://wiki.factorio.com/Prototype/SimpleEntityWithForce>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct SimpleEntityWithForce {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    render_layer: RenderLayer, // Default: "object"
    secondary_draw_order: i8, // Default: 0
    random_animation_offset: bool, // Default: false
    random_variation_on_create: bool, // Default: true
    visuals: SimpleEntityWithOwnerVisuals
}

/// <https://wiki.factorio.com/Prototype/SolarPanel>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct SolarPanel {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,

    energy_source: EnergySource, // Must be electric
    picture: SpriteVariations,
    production: Energy,
    overlay: Option<SpriteVariations>
}

/// <https://wiki.factorio.com/Prototype/SpiderLeg>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct SpiderLeg {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    part_length: f64, // Must be > 0
    initial_movement_speed: f64,
    movement_acceleration: f64,
    target_position_randomisation_distance: f64,
    minimal_step_size: f64,
    movement_based_position_selection_distance: f64,
    graphics_set: SpiderLegGraphicsSet,
    walking_sound_volume_modifier: f64, // Default: 1
}

/// <https://wiki.factorio.com/Prototype/StorageTank>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct StorageTank {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    fluid_box: FluidBox,
    window_bounding_box: BoundingBox,
    pictures: StorageTankPictures,
    flow_length_in_ticks: u32, // Must be positive
    two_direction_only: bool, // Default: false
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory if `circuit_wire_max_distance` > 0
    scale_info_icons: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/TrainStop>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct TrainStop {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,

    animation_ticks_per_frame: u32,
    rail_overlay_animations: Option<Animation4Way>,
    animations: Option<Animation4Way>,
    top_animations: Option<Animation4Way>,
    default_train_stopped_signal: Option<SignalIDConnector>,
    default_trains_count_signal: Option<SignalIDConnector>,
    default_trains_limit_signal: Option<SignalIDConnector>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    color: Option<Color>,
    chart_name: bool, // Default: true
    light1: Option<TrainStopLight>,
    light2: Option<TrainStopLight>,
    drawing_boxes: Option<TrainStopDrawingBoxes>,
    circuit_wire_connection_points: Vec<WireConnectionPoint>,
    circuit_connector_sprites: Vec<CircuitConnectorSprites>
}

// TODO: Clean up graphics properties
/// <https://wiki.factorio.com/Prototype/TransportBeltConnectable>
#[derive(Debug)]
pub struct TransportBeltConnectableBase {
    speed: f64,
    animation_speed_coefficient: f64, // Default: 1
    /// <https://wiki.factorio.com/Prototype/TransportBeltConnectable#belt_animation_set>
    belt_animation_set: TransportBeltConnectableGraphics,
}

#[derive(Debug)]
pub enum TransportBeltConnectableGraphics {
    AnimationSet(BeltAnimationSet),
    GraphicsSet(BeltGraphicsSet)
}

/// <https://wiki.factorio.com/Prototype/TransportBeltConnectable>
pub trait TransportBeltConnectable {
    fn speed(&self) -> f64;
    fn animation_speed_coefficient(&self) -> f64;
    fn belt_animation_set(&self) -> &TransportBeltConnectableGraphics;
}

/// <https://wiki.factorio.com/Prototype/LinkedBelt>
#[derive(Debug, Prototype, TransportBeltConnectable)]
pub struct LinkedBelt {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    structure: BeltStructureWithSideLoading,
    structure_render_layer: RenderLayer, // Default: "object"
    allow_clone_connection: bool, // Default: true
    allow_blueprint_connection: bool, // Default: true
    allow_side_loading: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/Loader1x1>
#[derive(Debug, Prototype, TransportBeltConnectable)]
pub struct Loader1x1 {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    structure: BeltStructure,
    filter_count: u8,
    structure_render_layer: RenderLayer, // Default: "object"
    container_distance: f64, // Default: 1.5
    belt_length: f64, // Default: 0.5
}

/// <https://wiki.factorio.com/Prototype/Loader1x2>
#[derive(Debug, Prototype, TransportBeltConnectable)]
pub struct Loader1x2 {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    structure: BeltStructure,
    filter_count: u8,
    structure_render_layer: RenderLayer, // Default: "object"
    container_distance: f64, // Default: 1.5
    belt_length: f64, // Default: 0.5
}

/// <https://wiki.factorio.com/Prototype/Splitter>
#[derive(Debug, Prototype, TransportBeltConnectable)]
pub struct Splitter {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    structure: Animation4Way,
    structure_patch: Option<Animation4Way>,
    structure_animation_speed_coefficient: f64, // Default: 1
    structure_animation_movement_cooldown: u32, // Default: 10
}

/// <https://wiki.factorio.com/Prototype/TransportBelt>
#[derive(Debug, Prototype, TransportBeltConnectable)]
pub struct TransportBelt {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    connector_frame_sprites: TransportBeltConnectorFrame,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_wire_connection_point: Option<Vec<WireConnectionPoint>>,
    circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>,
    belt_animation_set_indexes: Option<BeltAnimationSetIndexes>,
    animations: Option<RotatedAnimation>, // Must have 12 animations
    related_underground_belt: Option<String>, // Name of underground belt
}

/// <https://wiki.factorio.com/Prototype/UndergroundBelt>
#[derive(Debug, Prototype, TransportBeltConnectable)]
pub struct UndergroundBelt {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    max_distance: u8,
    structure: BeltStructureWithSideLoading,
    underground_sprite: Sprite,
    underground_remove_belts_sprite: Option<Sprite>
}

/// <https://wiki.factorio.com/Prototype/Tree>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Tree {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    variation_weights: Option<Vec<f64>>,
    darkness_of_burnt_tree: f32, // Default: 0.5
    visuals: TreeVisuals,
    // healing_per_tick: default 0.001666
}

// TODO: Accessors trait
/// <https://wiki.factorio.com/Prototype/Turret>
#[derive(Debug)]
pub struct TurretBase {
    attack_parameters: AttackParameters,
    folded_animation: RotatedAnimation4Way,
    call_for_help_radius: f64,
    corpse: Option<String>, // Name of corpse entity
    attack_target_mask: Option<TriggerTargetMask>, // Default: all
    ignore_target_mask: Option<TriggerTargetMask>, // Default: no
    shoot_in_prepare_state: bool, // Default: false
    turret_base_has_direction: bool, // Default: false
    random_animation_offset: bool, // Default: false
    secondary_animation: bool, // Default: false
    attack_from_start_frame: bool, // Default: false
    allow_turning_when_starting_attack: bool, // Default: false
    base_picture_secondary_draw_order: u8, // Default: 0
    gun_animation_secondary_draw_order: u8, // Default: 0
    base_picture_render_layer: RenderLayer, // Default: "lower-obejct"
    gun_animation_render_layer: RenderLayer, // Default: "object"
    base_picture: Option<Animation4Way>,
    preparing_animation: Option<RotatedAnimation4Way>,
    prepared_animation: Option<RotatedAnimation4Way>,
    prepared_alternative_animation: Option<RotatedAnimation4Way>,
    starting_attack_animation: Option<RotatedAnimation4Way>,
    attacking_animation: Option<RotatedAnimation4Way>,
    energy_glow_animation: Option<RotatedAnimation4Way>,
    ending_attack_animation: Option<RotatedAnimation4Way>,
    folding_animation: Option<RotatedAnimation4Way>,
    integration: Option<Sprite>,
    glow_light_intensity: f32, // Default: 0
    starting_attack_sound: Option<Sound>,
    dying_sound: Option<Sound>,
    preparing_sound: Option<Sound>,
    folding_sound: Option<Sound>,
    prepared_sound: Option<Sound>,
    prepared_alternative_sound: Option<Sound>,
    rotation_speed: f32, // Default: 1
    preparing_speed: f32, // Default: 1
    folded_speed: f32, // Default: 1
    folded_speed_secondary: f32, // Default: 1
    prepared_speed: f32, // Default: 1
    prepared_speed_secondary: f32, // Default: 1
    prepared_alternative_speed: f32, // Default: 1
    prepared_alternative_speed_secondary: f32, // Default: 1
    prepared_alternative_chance: f32, // Default: 0
    starting_attack_speed: f32, // Default: 1
    attacking_speed: f32, // Default: 1
    ending_attack_speed: f32, // Default: 1
    folding_speed: f32, // Default: 1
    prepare_range: f64, // Default: range defined in `attack_parameters`
    alert_when_attacking: bool, // Default: true
    spawn_decorations_on_expansion: bool, // Default: false,
    spawn_decoration: Option<Vec<CreateDecorativesTriggerEffectItem>>
}

/// <https://wiki.factorio.com/Prototype/Turret>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct TurretPrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    turret_base: TurretBase
}

/// <https://wiki.factorio.com/Prototype/AmmoTurret>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct AmmoTurret {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    turret_base: TurretBase,
    inventory_size: ItemStackIndex,
    animated_ammo_count: ItemCountType,
    entity_info_icon_shift: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Prototype/ElectricTurret>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct ElectricTurret {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    turret_base: TurretBase,
    energy_source: EnergySource
}

// `turret_base_has_direction` must = true
/// <https://wiki.factorio.com/Prototype/FluidTurret>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct FluidTurret {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    turret_base: TurretBase,
    fluid_buffer_size: f32,
    fluid_buffer_input_flow: f32,
    activation_buffer_ratio: f32,
    fluid_box: FluidBox,
    muzzle_light: Option<LightDefinition>,
    enough_fuel_indicator_light: Option<LightDefinition>,
    not_enough_fuel_indicator_light: Option<LightDefinition>,
    muzzle_animation: Option<Animation>,
    folded_muzzle_animation_shift: Option<AnimatedVector>,
    preparing_muzzle_animation_shift: Option<AnimatedVector>,
    prepared_muzzle_animation_shift: Option<AnimatedVector>,
    starting_attack_muzzle_animation_shift: Option<AnimatedVector>,
    attacking_muzzle_animation_shift: Option<AnimatedVector>,
    ending_attack_muzzle_animation_shift: Option<AnimatedVector>,
    folding_muzzle_animation_shift: Option<AnimatedVector>,
    enough_fuel_indicator_picture: Option<Sprite4Way>,
    not_enough_fuel_indicator_picture: Option<Sprite4Way>,
    out_of_ammo_alert_icon: Option<Sprite>
}

/// <https://wiki.factorio.com/Prototype/Unit>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Unit {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    run_animation: RotatedAnimation,
    attack_parameters: AttackParameters, // Requires animation in attack_paramaters. Requires ammo_type in attack_paramaters
    movement_speed: f32, // Must be >= 0
    distance_per_frame: f32,
    pollution_to_join_attack: f32,
    distraction_cooldown: u32,
    vision_distance: f64, // 100 max
    rotation_speed: f32, // Default: 0.025
    dying_sound: Option<Sound>,
    min_pursue_time: u32, // Default: 600
    has_belt_immunity: bool, // Default: false
    spawning_time_modifier: f64, // Default: 1
    max_pursue_distance: f64, // Default: 50
    radar_range: u32, // Default: 0
    ai_settings: Option<UnitAISettings>,
    move_while_shooting: bool, // Default: false
    can_open_gates: bool, // Default: false
    affected_by_tiles: bool, // Default: false
    render_layer: RenderLayer, // Default: "object"
    light: Option<LightDefinition>,
    walking_sound: Option<Sound>,
    alternative_attacking_frame_sequence: Option<UnitAlternativeAttackingFrameSequence>,
    running_sound_animation_positions: Option<Vec<f32>>, // Ignored if `walking_sound` is not defined
}

/// <https://wiki.factorio.com/Prototype/Vehicle>
#[derive(Debug)]
pub struct VehicleBase {
    weight: f64, // Mus be positive
    braking_force: f64, // Must be positive // braking_power is converted to this
    friction_force: f64, // Must be posotove // friction is converted to this
    energy_per_hit_point: f64,
    terrain_friction_modifier: f32, // Default: 1 // Must be [0, 1]
    sound_minimum_speed: f64, // Default: 1 / 60.0
    sound_scaling_ratio: f64, // Default: 1
    stop_trigger_speed: f64, // Default: 0
    crash_trigger: Option<TriggerEffect>,
    stop_trigger: Option<TriggerEffect>,
    equipment_grid: Option<String>, // Name of equipment grid
    minimap_representation: Option<Sprite>,
    selected_minimap_representation: Option<Sprite>,
    allow_passengers: bool // Default: true
}

/// <https://wiki.factorio.com/Prototype/Vehicle>
pub trait Vehicle {
    fn weight(&self) -> f64;
    fn braking_force(&self) -> f64;
    fn friction_force(&self) -> f64;
    fn energy_per_hit_point(&self) -> f64;
    fn terrain_friction_modifier(&self) -> f32;
    fn sound_minimum_speed(&self) -> f64;
    fn sound_scaling_ratio(&self) -> f64;
    fn stop_trigger_speed(&self) -> f64;
    fn crash_trigger(&self) -> &Option<TriggerEffect>;
    fn stop_trigger(&self) -> &Option<TriggerEffect>;
    fn equipment_grid(&self) -> &Option<String>;
    fn minimap_representation(&self) -> &Option<Sprite>;
    fn selected_minimap_representation(&self) -> &Option<Sprite>;
    fn allow_passengers(&self) -> bool;
}

/// <https://wiki.factorio.com/Prototype/Car>
#[derive(Debug, Prototype, Vehicle)]
pub struct Car {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    vehicle_base: VehicleBase,
    animation: RotatedAnimation,
    effectivity: f64,
    consumption: Energy,
    rotation_speed: f64,
    energy_source: EnergySource, // If used from `burner`, must be a burner energy source // Otherwise can also be a void energy source
    inventory_size: ItemStackIndex,
    turret_animation: Option<RotatedAnimation>,
    light_animation: Option<RotatedAnimation>, // Must have the same frame count as `animation`
    render_layer: RenderLayer, // Default: "object"
    tank_driving: bool, // Default: false
    has_belt_immunity: bool, // Default: false
    immune_to_tree_impacts: bool, // Default: false
    immune_to_rock_impacts: bool, // Default: false
    turret_rotation_speed: f64, // Default: 0.01
    turret_return_timeout: u32, // Default: 60
    light: Option<LightDefinition>,
    sound_no_fuel: Option<Sound>,
    darkness_to_render_light_animation: f32, // Default: 0.3
    track_particle_triggers: Option<FootstepTriggerEffectList>,
    guns: Vec<String>, // (Names) Name of gun prototypes
}

/// <https://wiki.factorio.com/Prototype/RollingStock>
#[derive(Debug)]
pub struct RollingStockBase {
    max_speed: f64,
    air_resistance: f64,
    joint_distance: f64,
    connection_distance: f64,
    pictures: RotatedSprite,
    vertical_selection_shift: f64,
    drive_over_tie_trigger: Option<TriggerEffect>,
    tie_distance: f64, // Default: 10.0
    back_light: Option<LightDefinition>,
    stand_by_light: Option<LightDefinition>,
    wheels: Option<RotatedSprite>,
    horizontal_doors: Option<Animation>,
    vertical_doors: Option<Animation>,
    color: Option<Color>,
    allow_manual_color: bool, // Default: true
    allow_robot_dispatch_in_automatic_mode: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/RollingStock>
pub trait RollingStock: Vehicle {
    fn max_speed(&self) -> f64;
    fn air_resistance(&self) -> f64;
    fn joint_distance(&self) -> f64;
    fn connection_distance(&self) -> f64;
    fn pictures(&self) -> &RotatedSprite;
    fn vertical_selection_shift(&self) -> f64;
    fn drive_over_tie_trigger(&self) -> &Option<TriggerEffect>;
    fn tie_distance(&self) -> f64;
    fn back_light(&self) -> &Option<LightDefinition>;
    fn stand_by_light(&self) -> &Option<LightDefinition>;
    fn wheels(&self) -> &Option<RotatedSprite>;
    fn horizontal_doors(&self) -> &Option<Animation>;
    fn vertical_doors(&self) -> &Option<Animation>;
    fn color(&self) -> &Option<Color>;
    fn allow_manual_color(&self) -> bool;
    fn allow_robot_dispatch_in_automatic_mode(&self) -> bool;
}

/// <https://wiki.factorio.com/Prototype/ArtilleryWagon>
#[derive(Debug, Prototype, RollingStock)]
pub struct ArtilleryWagon {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    vehicle_base: VehicleBase,
    rolling_stock_base: RollingStockBase,
    gun: String, // Name of Prototype/Gun
    inventory_size: ItemStackIndex, // Must be > 0
    ammo_stack_limit: ItemCountType, // Must be > 0
    turret_rotation_speed: f64,
    manual_range_modifier: f64, // Must be > 0
    disable_automatic_firing: bool, // Default: false
    cannon_base_pictures: Option<RotatedSprite>,
    cannon_barrel_pictures: Option<RotatedSprite>,
    rotating_sound: Option<InterruptibleSound>,
    rotating_stopped_sound: Option<Sound>,
    turn_after_shooting_cooldown: u16, // Default: 0
    cannon_parking_frame_count: u16, // Default: 0
    cannon_parking_speed: f32, // Default: 1
    cannon_base_shiftings: Option<Vec<Factorio2DVector>>, // Must match `cannon_base_pictures` frame count
    cannon_barrel_recoil_shiftings: Option<Vec<Factorio3DVector>>,
    cannon_barrel_recoil_shiftings_load_correction_matrix: Option<Vec<Factorio3DVector>>, // Only loaded if `cannon_barrel_recoil_shiftings` is loaded
    cannon_barrel_light_direction: Option<Factorio3DVector>, // Only loaded if `cannon_barrel_recoil_shiftings` is loaded
}

/// <https://wiki.factorio.com/Prototype/CargoWagon>
#[derive(Debug, Prototype, RollingStock)]
pub struct CargoWagon {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    vehicle_base: VehicleBase,
    rolling_stock_base: RollingStockBase,
    inventory_size: ItemStackIndex
}

/// <https://wiki.factorio.com/Prototype/FluidWagon>
#[derive(Debug, Prototype, RollingStock)]
pub struct FluidWagon {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    vehicle_base: VehicleBase,
    rolling_stock_base: RollingStockBase,
    capacity: f64,
    tank_count: u8, // Default: 3 // Must be one of: 1, 2, 3
}

/// <https://wiki.factorio.com/Prototype/Locomotive>
#[derive(Debug, Prototype, RollingStock)]
pub struct Locomotive {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    vehicle_base: VehicleBase,
    rolling_stock_base: RollingStockBase,
    max_power: Energy,
    reversing_power_modifier: f64,
    energy_source: EnergySource, // Must be burner if used through `burner`, otherwise can also be void
    front_light: Option<LightDefinition>,
    front_light_pictures: Option<RotatedSprite>,
    darkness_to_render_light_animation: f32, // Default: 0.3
}

/// <https://wiki.factorio.com/Prototype/SpiderVehicle>
#[derive(Debug, Prototype, Vehicle)]
pub struct SpiderVehicle {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    vehicle_base: VehicleBase,
    energy_source: EnergySource, // Must be burner if used through `burner`, otherwise can also be void
    inventory_size: ItemStackIndex,
    graphics_set: SpiderVehicleGraphicsSet,
    spider_engine: SpiderEnginePrototype,
    height: f32,
    chunk_exploration_radius: u32,
    movement_energy_consumption: Energy,
    automatic_weapon_cycling: bool,
    chain_shooting_cooldown_modifier: f32,
    torso_rotation_speed: f32, // Default: 1
    trash_inventory_size: ItemStackIndex, // Default: 0
    guns: Vec<String>, // (Names) Name of gun
}

/// <https://wiki.factorio.com/Prototype/Wall>
#[derive(Debug, Prototype, EntityWithHealth)]
pub struct Wall {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pictures: WallPictures,
    visual_merge_group: u32, // Default: 0
    circuit_wire_connection_point: Option<WireConnectionPoint>,
    circuit_wire_max_distance: f64, // Default: 0
    draw_copper_wires: bool, // Default: true
    draw_circuit_wires: bool, // Default: true
    circuit_connector_sprites: Option<CircuitConnectorSprites>,
    default_output_signal: Option<SignalIDConnector>,
    wall_diode_green: Option<Sprite4Way>,
    wall_diode_red: Option<Sprite4Way>,
    wall_diode_green_light_top: Option<LightDefinition>,
    wall_diode_green_light_right: Option<LightDefinition>,
    wall_diode_green_light_bottom: Option<LightDefinition>,
    wall_diode_green_light_left: Option<LightDefinition>,
    wall_diode_red_light_top: Option<LightDefinition>,
    wall_diode_red_light_right: Option<LightDefinition>,
    wall_diode_red_light_bottom: Option<LightDefinition>,
    wall_diode_red_light_left: Option<LightDefinition>,
    connected_gate_visualization: Option<Sprite>
}

/// <https://wiki.factorio.com/Prototype/Explosion>
#[derive(Debug, Prototype, Entity)]
pub struct Explosion {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    animations: AnimationVariations,
    sound: Option<Sound>,
    smoke: Option<String>, // Name of trivial-smoke prototype // Mandatory if `smoke_count` > 0
    height: f32, // Default: 1
    smoke_slow_down_factor: f32, // Default: 0
    smoke_count: u16, // Default: 0
    rotate: bool, // Default: false
    beam: bool, // Default: false
    correct_rotation: bool, // Default: false
    scale_animation_speed: bool, // Default: false
    fade_in_duration: u8, // Default: 0
    fade_out_duration: u8, // Default: 0
    render_layer: RenderLayer, // Default: "explosion"
    scale_in_duration: u8, // Default: 0
    scale_out_duration: u8, // Default: 0
    scale_end: f32, // Default: 1
    scale_increment_per_tick: f32, // Default: 0
    light_intensity_factor_initial: f32, // Default: 0
    light_intensity_factor_final: f32, // Default: 0
    light_size_factor_initial: f32, // Default: 0.05
    light_size_factor_final: f32, // Default: 0.1
    light: Option<LightDefinition>,
    light_intensity_peak_start_progress: f32, // Default: 0
    light_intensity_peak_end_progress: f32, // Default: 0.9
    light_size_peak_start_progress: f32, // Default: 0.1
    light_size_peak_end_progress: f32, // Default: 0.5
    scale_initial: f32, // Default: 1
    scale_initial_deviation: f32, // Default: 0
    scale: f32, // Default: 1
    scale_deviation: f32, // Default: 0
}

/// <https://wiki.factorio.com/Prototype/FlameThrowerExplosion>
#[derive(Debug, Prototype, Entity)]
pub struct FlameThrowerExplosion {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    animations: AnimationVariations,
    sound: Option<Sound>,
    smoke: Option<String>, // Name of trivial-smoke prototype // Mandatory if `smoke_count` > 0
    height: f32, // Default: 1
    smoke_slow_down_factor: f32, // Default: 0
    smoke_count: u16, // Default: 0
    rotate: bool, // Default: false
    beam: bool, // Default: false
    correct_rotation: bool, // Default: false
    scale_animation_speed: bool, // Default: false
    fade_in_duration: u8, // Default: 0
    fade_out_duration: u8, // Default: 0
    render_layer: RenderLayer, // Default: "explosion"
    scale_in_duration: u8, // Default: 0
    scale_out_duration: u8, // Default: 0
    scale_end: f32, // Default: 1
    scale_increment_per_tick: f32, // Default: 0
    light_intensity_factor_initial: f32, // Default: 0
    light_intensity_factor_final: f32, // Default: 0
    light_size_factor_initial: f32, // Default: 0.05
    light_size_factor_final: f32, // Default: 0.1
    light: Option<LightDefinition>,
    light_intensity_peak_start_progress: f32, // Default: 0
    light_intensity_peak_end_progress: f32, // Default: 0.9
    light_size_peak_start_progress: f32, // Default: 0.1
    light_size_peak_end_progress: f32, // Default: 0.5
    scale_initial: f32, // Default: 1
    scale_initial_deviation: f32, // Default: 0
    scale: f32, // Default: 1
    scale_deviation: f32, // Default: 0
}

/// <https://wiki.factorio.com/Prototype/FireFlame>
#[derive(Debug, Prototype, Entity)]
pub struct FireFlame {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    damage_per_tick: DamagePrototype,
    spread_delay: u32,
    spread_delay_deviation: u32,
    render_layer: RenderLayer, // Default: "object"
    initial_render_layer: RenderLayer, // Default: "object"
    secondary_render_layer: RenderLayer, // Default: "object"
    small_tree_fire_pictures: Option<AnimationVariations>,
    pictures: Option<AnimationVariations>,
    smoke_source_pictures: Option<AnimationVariations>,
    secondary_pictures: Option<AnimationVariations>,
    burnt_patch_pictures: Option<SpriteVariations>,
    secondary_picture_fade_out_start: u32, // Default: 0
    secondary_picture_fade_out_duration: u32, // Default: 30
    spawn_entity: Option<String>, // Name of entity
    smoke: Option<Vec<SmokeSource>>,
    maximum_spread_count: u16, // Default: 200
    initial_flame_count: u8, // Default: 0
    uses_alternative_behavior: bool, // Default: false
    limit_overlapping_particles: bool, // Default: false
    tree_dying_factor: f32, // Default: 0
    fade_in_duration: u32, // Default: 30
    fade_out_duration: u32, // Default: 30
    initial_lifetime: u32, // Default: 300
    damage_multiplier_decrease_per_tick: f32, // Default: 0
    damage_multiplier_increase_per_added_fuel: f32, // Default: 0
    maximum_damage_multiplier: f32, // default: 1
    lifetime_increase_by: u32, // Default: 20
    lifetime_increase_cooldown: u32, // Default: 10
    maximum_lifetime: u32, // Default: u32::MAX
    add_fuel_cooldown: u32, // Default: 10
    delay_between_initial_flames: u32, // Default: 10
    smoke_fade_in_duration: u32, // Default: 30
    smoke_fade_out_duration: u32, // Default: 30
    on_fuel_added_action: Option<Trigger>,
    on_damage_tick_effect: Option<Trigger>,
    light: Option<LightDefinition>,
    particle_alpha_blend_duration: u16, // Default: 0
    burnt_patch_lifetime: u32, // Default: 1800
    burnt_patch_alpha_default: f32, // Default: 1
    // Only loaded if `uses_alternative_behavior` is false
    particle_alpha: f32, // Default: 1
    particle_alpha_deviation: f32, // Default: 0
    flame_alpha: f32, // Default: 1
    flame_alpha_deviation: f32, // Default: 0
    //
    burnt_patch_alpha_variations: Option<Vec<FireFlameBurntPatchAlphaVariation>>
}

/// <https://wiki.factorio.com/Prototype/FluidStream>
#[derive(Debug, Prototype, Entity)]
pub struct FluidStream {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    particle_spawn_interval: u16,
    particle_horizontal_speed: f64, // Must be higher than 0 // Must be greater than `particle_horizontal_speed_deviation`
    particle_horizontal_speed_deviation: f64,
    particle_vertical_acceleration: f64,
    initial_action: Option<Trigger>,
    action: Option<Trigger>,
    special_neutral_target_damage: Option<DamagePrototype>,
    width: f32, // Default: 0.5
    particle_buffer_size: u32, // Default: 20 // Must be less than 256 // So u8?
    particle_spawn_timeout: u16, // Default: 4 * `particle_spawn_interval`
    particle_start_alpha: f32, // Default: 1
    particle_end_alpha: f32, // Default: 1
    particle_start_scale: f32, // Default: 1
    particle_alpha_per_part: f32, // Default: 1
    particle_scale_per_part: f32, // Default: 1
    particle_fade_out_threshold: f32, // Defayklt: 1, // Between 0 and 1
    particle_loop_exit_threshold: f32, // Default: 0 // Between 0 and 1
    particle_loop_frame_count: u16, // Default: 1 // If less than 1, force 1
    particle_fade_out_duration: u16, // Default: u16::MAX // If less than 1, force 1
    spine_animation: Option<Animation>,
    particle: Option<Animation>,
    shadow: Option<Animation>,
    smoke_sources: Option<Vec<SmokeSource>>,
    progress_to_create_smoke: f32, // Default: 0.5
    stream_light: Option<LightDefinition>,
    ground_light: Option<LightDefinition>,
    target_position_deviation: f64, // Default: 0
    oriented_particle: bool, // Default: false
    shadow_scale_enabled: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/FlyingText>
#[derive(Debug, Prototype, Entity)]
pub struct FlyingText {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    speed: f32,
    time_to_live: u32,
    text_alignment: TextAlignment // Default: "left"
}

/// <https://wiki.factorio.com/Prototype/HighlightBoxEntity>
#[derive(Debug, Prototype, Entity)]
pub struct HighlightBoxEntity {
    // Bruh
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
}

/// <https://wiki.factorio.com/Prototype/ItemEntity>
#[derive(Debug, Prototype, Entity)]
pub struct ItemEntity {
    // Bruh
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
}

/// <https://wiki.factorio.com/Prototype/ItemRequestProxy>
#[derive(Debug, Prototype, Entity)]
pub struct ItemRequestProxy {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    picture: Sprite,
    use_target_entity_alert_icon_shift: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/ParticleSource>
#[derive(Debug, Prototype, Entity)]
pub struct ParticleSource {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    time_to_live: f32,
    time_before_start: f32,
    height: f32,
    vertical_speed: f32,
    horizontal_speed: f32,
    particle_or_smoke: ParticleSourceParticleOrSmoke,
    time_to_live_deviation: f32, // Default: 0
    time_before_start_deviation: f32, // Default: 0
    height_deviation: f32, // Default: 0
    vertical_speed_deviation: f32, // Default: 0
    horizontal_speed_deviation: f32, // Default: 0
}

/// <https://wiki.factorio.com/Prototype/ParticleSource#particle>
#[derive(Debug)]
pub enum ParticleSourceParticleOrSmoke {
    Particle(String), // Name of Particle prototype
    Smoke(Vec<SmokeSource>) // 1 or more
}

/// <https://wiki.factorio.com/Prototype/Projectile>
#[derive(Debug, Prototype, Entity)]
pub struct Projectile {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    acceleration: f64, // Must be != 0 if `turning_speed_increases_exponentially_with_projectile_spee` is true
    animation: Option<Animation>,
    rotatable: bool, // Default: true
    enable_drawing_with_mask: bool, // Default: false
    direction_only: bool, // Default: false
    hit_at_collision_position: bool, // Default: false
    force_condition: ForceCondition, // Default: "all"
    piercing_damage: f32, // Default: 0
    max_speed: f64, // Default: f64::MAX
    turn_speed: Factorio2DVector, // Default: (1, 1)
    speed_modifier: f32, // Default: 1
    height: f64, // Default: 1
    action: Option<Trigger>,
    final_action: Option<Trigger>,
    light: Option<LightDefinition>,
    smoke: Vec<SmokeSource>,
    hit_collision_mask: CollisionMask, // Default: ["player-layer", "train-layer"]
    // This property name is insanely verbose
    turning_speed_increases_exponentially_with_projectile_speed: bool, // Default: false
    shadow: Option<Animation>
}

/// <https://wiki.factorio.com/Prototype/ResourceEntity>
#[derive(Debug, Prototype, Entity)]
pub struct ResourceEntity {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    stages: AnimationVariations,
    stage_counts: Vec<u32>,
    infinite: bool, // Default: false
    highlight: bool, // Default: false
    randomize_visual_position: bool, // Default: true
    map_grid: bool, // Default: true
    minimum: u32, // Must be != 0 if `infinite` is true
    normal: u32, // Must be != 0 if `infinite` is true
    infinite_depletion_amount: u32, // Default: 1
    resource_patch_search_radius: u32, // Default: 3
    category: String, // Default: "basic-solid"
    walking_sound: Option<Sound>,
    stages_effect: Option<AnimationVariations>,
    effect_animation_period: f32, // Default: 0
    effect_animation_period_deviation: f32, // Default: 0
    effect_darkness_multiplier: f32, // Default; 1
    min_effect_alpha: f32, // Default: 0
    max_effect_alpha: f32, // Default: 1
    tree_removal_probability: f64, // Default: 0 // Must be positive
    tree_removal_max_distance: f64, // Default: 0 // Must be positive when `tree_removal_probability` is set
    mining_visualisation_tint: Color, // Default: resource map color OR white if both unset
}

/// <https://wiki.factorio.com/Prototype/RocketSiloRocket>
#[derive(Debug, Prototype, Entity)]
pub struct RocketSiloRocket {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    rocket_sprite: Sprite,
    rocket_shadow_sprite: Sprite,
    rocket_glare_overlay_sprite: Sprite,
    rocket_smoke_bottom1_animation: Animation,
    rocket_smoke_bottom2_animation: Animation,
    rocket_smoke_top1_animation: Animation,
    rocket_smoke_top2_animation: Animation,
    rocket_smoke_top3_animation: Animation,
    rocket_flame_animation: Animation,
    rocket_flame_left_animation: Animation,
    rocket_flame_right_animation: Animation,
    rocket_rise_offset: Factorio2DVector,
    rocket_flame_left_rotation: f32,
    rocket_flame_right_rotation: f32,
    rocket_render_layer_switch_distance: f64,
    full_render_layer_switch_distance: f64,
    rocket_launch_offset: Factorio2DVector,
    effects_fade_in_start_distance: f64,
    effects_fade_in_end_distance: f64,
    shadow_fade_out_start_ratio: f64,
    shadow_fade_out_end_ratio: f64,
    rocket_visible_distance_from_center: f64,
    rising_speed: f64,
    engine_starting_speed: f64,
    flying_speed: f64,
    flying_acceleration: f64,
    inventory_size: ItemStackIndex,
    shadow_slave_entity: Option<String>, // Name of an entity
    dying_explosion: Option<String>, // Name of an entity
    glow_light: Option<LightDefinition>,
    rocket_initial_offset: Factorio2DVector,
    rocket_above_wires_slice_offset_from_center: f64, // Default: -3
    rocket_air_object_slice_offset_from_center: f64, // Default: -5.5
    flying_trigger: Option<TriggerEffect>
}

/// <https://wiki.factorio.com/Prototype/RocketSiloRocketShadow>
#[derive(Debug, Prototype, Entity)]
pub struct RocketSiloRocketShadow {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase
}

/// <https://wiki.factorio.com/Prototype/SmokeWithTrigger>
#[derive(Debug, Prototype, Entity)]
pub struct SmokeWithTrigger {
    // Collision box must be zero
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    animation: Animation,
    cyclic: bool, // Default: false
    duration: u32, // Default: 0 // May not be 0 if `cyclic` is true
    spread_duration: u32, // Default: 0
    // `fade_in_duration` + `fade_away_duration` must be <= `duration`
    fade_away_duration: u32, // Default: 0
    fade_in_duration: u32, // Default: 0
    start_scale: f64, // Default: 1
    end_scale: f64, // Default: 1
    color: Color, // Default: (0.375, 0.375, 0.375, 0.375) [rgba]
    affected_by_wind: bool, // Default: true
    show_when_smoke_off: bool, // Default: false
    render_layer: RenderLayer, // Default: "smoke"
    movement_slow_down_factor: f64, // Default: 0.995 // Must be [0; 1]
    glow_fade_away_duration: u32, // Default: `fade_away_duration`
    glow_animation: Option<Animation>,
    action: Option<Trigger>,
    action_cooldown: u32, // Default: 0
    particle_count: u8, // Default: 1
    particle_distance_scale_factor: f32, // Default: 0
    spread_duration_variation: u32, // Default: 0
    particle_duration_variation: u32, // Default: 0
    particle_spread: Option<Factorio2DVector>,
    particle_scale_factor: Option<Factorio2DVector>,
    wave_distance: Option<Factorio2DVector>,
    wave_speed: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Prototype/SpeechBubble>
#[derive(Debug, Prototype, Entity)]
pub struct SpeechBubble {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    style: String, // Needs a style of the type "speech_bubble_style", defined inside the gui styles.
    wrapper_flow_style: String, // Default: "flow_style" // Needs a style of the type "flow_style", defined inside the gui styles.
    y_offset: f64, // Default: 0
    fade_in_out_ticks: u32, // Default: 60
}

/// <https://wiki.factorio.com/Prototype/Sticker>
#[derive(Debug, Prototype, Entity)]
pub struct Sticker {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    duration_in_ticks: u32, // Must be > 0
    animation: Option<Animation>,
    damage_interval: u32, // Default: 1
    spread_fire_entity: Option<String>, // Name of an entity
    fire_spread_cooldown: u8, // Default: 30
    fire_spread_radius: f32, // Default: 1
    stickers_per_square_meter: f32, // Default: 15
    force_visibility: ForceCondition, // Default: "all"
    single_particle: bool, // Default: false
    damage_per_tick: Option<DamagePrototype>,
    target_movement_modifier: f32, // Default: 1
    target_movement_modifier_from: f32, // Default: `target_movement_modifier`
    target_movement_modifier_to: f32, // Default: `target_movement_modifier`
    vehicle_speed_modifier: f32, // Default: 1
    vehicle_speed_modifier_from: f32, // Default: `vehicle_speed_modifier`
    vehicle_speed_modifier_to: f32, // Default: `vehicle_speed_modifier`
    vehicle_friction_modifier: f32, // Default: 1
    vehicle_friction_modifier_from: f32, // Default: `vehicle_friction_modifier`
    vehicle_friction_modifier_to: f32, // Default: `vehicle_friction_modifier`
    selection_box_type: CursorBoxType, // Default: "entity"
}

/// <https://wiki.factorio.com/Prototype/TileGhost>
#[derive(Debug, Prototype, Entity)]
pub struct TileGhost {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase
}

/// <https://wiki.factorio.com/Prototype/Equipment>
#[derive(Debug)]
pub struct EquipmentBase {
    sprite: Sprite,
    shape: EquipmentShape,
    categories: Vec<String>, // (Names) Name of EquipmentCategory
    energy_source: EnergySource, // Must be electric
    take_result: String, // Default: name of this prototype
    background_color: Color, // Default: value of equipment_default_background_color in the utility constants
    background_border_color: Color, // Default: value of equipment_default_background_border_color in the utility constants
    grabbed_background_color: Color, // Default: value of equipment_default_grabbed_background_color in the utility constants
}

/// <https://wiki.factorio.com/Prototype/Equipment>
pub trait Equipment: PrototypeBase {
    fn sprite(&self) -> &Sprite;
    fn shape(&self) -> &EquipmentShape;
    fn categories(&self) -> &Vec<String>;
    fn energy_source(&self) -> &EnergySource;
    fn take_result(&self) -> &String;
    fn background_color(&self) -> &Color;
    fn background_border_color(&self) -> &Color;
    fn grabbed_background_color(&self) -> &Color;
}

/// <https://wiki.factorio.com/Prototype/ActiveDefenseEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct ActiveDefenseEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    automatic: bool,
    attack_parameters: AttackParameters
}

/// <https://wiki.factorio.com/Prototype/BatteryEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct BatteryEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
}

/// <https://wiki.factorio.com/Prototype/BeltImmunityEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct BeltImmunityEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    energy_consumption: Energy
}

/// <https://wiki.factorio.com/Prototype/EnergyShieldEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct EnergyShieldEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    max_shield_value: f32,
    energy_per_shield: Energy
}

/// <https://wiki.factorio.com/Prototype/GeneratorEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct GeneratorEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    power: Energy,
    burner: Option<EnergySource> // Must be a burner
}

/// <https://wiki.factorio.com/Prototype/MovementBonusEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct MovementBonusEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    energy_consumption: Energy,
    movement_bonus: f64
}

/// <https://wiki.factorio.com/Prototype/NightVisionEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct NightVisionEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    energy_input: Energy,
    color_lookup: DaytimeColorLookupTable,
    darkness_to_turn_on: f32, // Default: 0.5 // Must be >= 0 and <= 1
    activate_sound: Option<Sound>,
    deactivate_sound: Option<Sound>
}

/// <https://wiki.factorio.com/Prototype/RoboportEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct RoboportEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    recharging_animation: Animation,
    spawn_and_station_height: f32,
    charge_approach_distance: f32,
    construction_radius: f32,
    charging_energy: Energy,
    spawn_and_station_shadow_height_offset: f32, // Default: 0
    draw_logistic_radius_visualization: bool, // Default: true
    draw_construction_radius_visualization: bool, // Default: true
    recharging_light: Option<LightDefinition>,
    charging_station_count: u32, // Default: 0
    charging_distance: f32, // Default: 0
    charging_station_shift: Option<Factorio2DVector>,
    charging_threshold_distance: f32, // Default: 1
    robot_vertical_acceleration: f32, // Default: 0.01
    stationing_offset: Option<Factorio2DVector>,
    robot_limit: ItemCountType, // Default: u32::MAX
    robots_shrink_when_entering_and_exiting: bool, // Default: false
    charging_offsets: Vec<Factorio2DVector>,
    spawn_minimum: Energy, // Default: 0.2 * `energy_source.buffer_capacity`
    burner: Option<EnergySource>, // Must be a burner
    power: Option<Energy>, // Mandatory if `burner` is present
}

/// <https://wiki.factorio.com/Prototype/SolarPanelEquipment>
#[derive(Debug, Prototype, Equipment)]
pub struct SolarPanelEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    power: Energy
}

/// <https://wiki.factorio.com/Prototype/EquipmentCategory>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct EquipmentCategory {
    name: String,
    prototype_base: PrototypeBaseSpec,
}

/// <https://wiki.factorio.com/Prototype/EquipmentGrid>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct EquipmentGrid {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_categories: Vec<String>, // (Names) Name of Equipment category // HashSet::intersection can be used here if I were to implement it runtime
    width: u32,
    height: u32,
    locked: bool // Default: false
}

#[derive(Clone, Debug, Error)]
pub enum PrototypesErr {
    #[error("Invalid prototype type: {0}")]
    InvalidPrototypeType(String),
    #[error("Invalid mod setting type: {0}")]
    InvalidModSettingType(String),
    #[error("Invalid string for type {0}: {1}")]
    InvalidTypeStr(String, String),
    #[error("Prototype \"{0}\" not found")]
    PrototypeNotFound(String)
}
