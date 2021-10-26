pub mod concepts;
pub mod mod_structs;
pub mod prototypes;
pub mod prototype_type;
pub mod types;

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::fmt;
use std::marker::PhantomData;
use mlua::{Value, Lua, prelude::LuaResult, Integer, Table};
use thiserror::Error;
use prototypes::*;
use crate::types::SpriteSizeType;

/// Shorthand for prototype category/type, used in [DataTable]
pub type PrototypeCategory<T> = HashMap<String, T>;

/// Struct representing global `data` table in lua environment
#[derive(Debug)]
pub struct DataTable {
    references: Vec<Weak<dyn PrototypeReferenceValidate>>,
    resource_records: Vec<ResourceRecord>,
    // Prototypes
    ambient_sound: PrototypeCategory<AmbientSoundPrototype>,
    animation: PrototypeCategory<AnimationPrototype>,
    editor_controller: PrototypeCategory<EditorController>,
    font: PrototypeCategory<Font>,
    god_controller: PrototypeCategory<GodController>,
    map_gen_settings: PrototypeCategory<MapGenPresets>,
    map_settings: PrototypeCategory<MapSettings>,
    mouse_cursor: PrototypeCategory<MouseCursor>,
    sound: PrototypeCategory<SoundPrototype>,
    spectator_controller: PrototypeCategory<SpectatorController>,
    sprite: PrototypeCategory<SpritePrototype>,
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
    equipment_category: PrototypeCategory<EquipmentCategory>,
    equipment_grid: PrototypeCategory<EquipmentGrid>,
    fluid: PrototypeCategory<Fluid>,
    fuel_category: PrototypeCategory<FuelCategory>,
    gui_style: PrototypeCategory<GuiStyle>,
    item: PrototypeCategory<ItemPrototype>,
    ammo: PrototypeCategory<AmmoItem>,
    capsule: PrototypeCategory<Capsule>,
    gun: PrototypeCategory<Gun>,
    item_with_entity_data: PrototypeCategory<ItemWithEntityData>,
    item_with_label: PrototypeCategory<ItemWithLabel>,
    item_with_inventory: PrototypeCategory<ItemWithInventory>,
    blueprint_book: PrototypeCategory<BlueprintBook>,
    item_with_tags: PrototypeCategory<ItemWithTags>,
    selection_tool: PrototypeCategory<SelectionToolPrototype>,
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
    utility_constants: PrototypeCategory<UtilityConstants>,
    utility_sounds: PrototypeCategory<UtilitySounds>,
    utility_sprites: PrototypeCategory<UtilitySprites>,
    virtual_signal: PrototypeCategory<VirtualSignal>,
    bool_setting: PrototypeCategory<BoolModSetting>,
    int_setting: PrototypeCategory<IntModSetting>,
    double_setting: PrototypeCategory<DoubleModSetting>,
    string_setting: PrototypeCategory<StringModSetting>,
}

impl DataTable {
    /// Shorthand for [DataTableAccessable::find]
    pub fn find<T: DataTableAccessable>(&self, name: &String) -> Result<&T, PrototypesErr> {
        T::find(self, name)
    }

    /// Shorthand for [DataTableAccessable::extend]
    pub fn extend<T: DataTableAccessable>(&mut self, prototype: T) -> Result<(), PrototypesErr> {
        prototype.extend(self)
    }

    /// Creates new reference and keeps track of it to later be validated through [Self::validate_references]
    pub fn new_reference<T: 'static + DataTableAccessable>(&mut self, name: String) -> Rc<PrototypeReference<T>> {
        let prot_reference = Rc::new(PrototypeReference::<T>::new(name));
        self.references.push(Rc::downgrade(&(prot_reference.clone() as Rc<dyn PrototypeReferenceValidate>)));
        prot_reference
    }

    /// Validates all tracked references.
    pub fn validate_references(&self) -> Result<(), PrototypesErr> {
        for prot_reference in &self.references {
            match prot_reference.upgrade() {
                Some(pref) => pref.validate(self)?,
                None => {}
            }
        }
        Ok(())
    }

    /// Create new resource record
    pub fn new_resource_record(&mut self, resource_record: ResourceRecord) -> () {
        self.resource_records.push(resource_record);
    }

    // Probably should be done at prototype definition load
    /// Validate resources
    /// callback is a function that should find the file and perform necessary checks, returning
    /// the Result of the check.
    pub fn validate_resources<F: Fn(&ResourceRecord) -> Result<(), ResourceError>>(&self, callback: F) -> Result<(), ResourceError> {
        for resource_record in &self.resource_records {
            callback(&resource_record)?;
        }
        Ok(())
    }
}

/// [mlua::FromLua] alternative with [DataTable] reference being passed
pub trait PrototypeFromLua<'lua>: Sized {
    fn prototype_from_lua(lua_value: Value<'lua>, lua: &'lua Lua, data_table: &DataTable) -> LuaResult<Self>;
}

/// Validate PrototypeReference. Any type.
trait PrototypeReferenceValidate: fmt::Debug {
    fn validate(&self, data_table: &DataTable) -> Result<(), PrototypesErr>;
}

/// Reference to a prototype by name.
#[derive(Debug)]
pub struct PrototypeReference<T: DataTableAccessable> {
    pub name: String,
    prot: PhantomData<T>
}

impl<T: DataTableAccessable> PrototypeReference<T> {
    /// Creates new unresolved Prototype reference
    pub fn new(name: String) -> Self {
        Self{name, prot: PhantomData}
    }

    pub fn find<'a>(&self, data_table: &'a DataTable) -> Result<&'a T, PrototypesErr> {
        data_table.find::<T>(&self.name)
    }

    /// Checks if reference is valid.
    pub fn is_valid(&self, data_table: &DataTable) -> bool {
        match self.find(data_table) as Result<&T, PrototypesErr> {
            Ok(_) => true,
            _ => false
        }
    }
}

impl<T: DataTableAccessable> PrototypeReferenceValidate for PrototypeReference<T> {
    /// Validates the reference
    fn validate(&self, data_table: &DataTable) -> Result<(), PrototypesErr> {
        data_table.find::<T>(&self.name).map(|_| ())
    }
}

/// Trait for manipulating prototypes in [Data table](DataTable).
/// Primarily used for [`PrototypeReference`]
pub trait DataTableAccessable: Prototype {
    /// Find prototype in [Data table](DataTable) by it's name
    fn find<'a>(data_table: &'a DataTable, name: &String) -> Result<&'a Self, PrototypesErr> where Self: Sized;
    /// Extend [Data table](DataTable) with this prototype
    fn extend(self, data_table: &mut DataTable) -> Result<(), PrototypesErr>;
}

/// Struct for recording resources (images, sound files)
#[derive(Debug)]
pub struct ResourceRecord {
    pub path: String,
    pub resource_type: ResourceType
}

/// Respirce type with additional info if neded
#[derive(Debug)]
pub enum ResourceType {
    Image(SpriteSizeType, SpriteSizeType), // x and y dimensions of an image
    Sound
}

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("File not found: \"{0}\"")]
    FileNotFound(String),
    #[error("Image size incorrect: Expected at least {0}x{1}, got {2}x{3}")]
    ImageSizeIncorrect(SpriteSizeType, SpriteSizeType, SpriteSizeType, SpriteSizeType)
}

// TODO: add more features
/// Unfinished wrapper around [mlua::Lua::new] that sets some global variables
/// Adds `table_size` global function into environment
pub fn new_lua_instance() -> LuaResult<Lua> {
    let lua = Lua::new();

    {
        let globals = lua.globals();

        fn tablesize<'lua>(_lua: &'lua Lua, table: Table) -> LuaResult<Integer> {
            Ok(table.table_size(true))
        }

        let tablesize_luaf = lua.create_function(tablesize)?;

        globals.set("table_size", tablesize_luaf)?;
    }

    Ok(lua)
}
