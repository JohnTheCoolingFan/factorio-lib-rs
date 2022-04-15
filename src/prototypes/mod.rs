mod abstract_prototypes;
mod utility;
pub mod additional_types;
pub mod prototype_type;

pub use abstract_prototypes::*;
pub use utility::*;

use std::{collections::HashMap, rc::{Rc, Weak}, marker::PhantomData, hash::Hash, fmt};
use thiserror::Error;
use mlua::{Value, Lua, prelude::LuaResult, ToLua, FromLua};
use additional_types::*;
use factorio_lib_rs_derive::{
    Prototype,
    ModSetting,
    PrototypeBase,
    Entity,
    Corpse,
    EntityWithHealth,
    EntityWithOwner,
    Combinator,
    CraftingMachine,
    FlyingRobot,
    TransportBeltConnectable,
    Turret,
    Vehicle,
    RollingStock,
    Equipment,
    Item,
    SelectionTool,
    DataTableAccessable,
    PrototypeFromLua,
    prot_from_lua_blanket,
};

#[cfg(feature = "concepts")]
use crate::concepts::LocalisedString;
#[cfg(not(feature = "concepts"))]
type LocalisedString = String;

// TODO: Make fields not acessible through traits pub

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

// Factorio prototypes
// Source info:
// For prototypes: https://wiki.factorio.com/Prototype_definitions
// For settings: https://wiki.factorio.com/Tutorial:Mod_settings

// Prototype
// Contains all values (accessors) for every prototype in the game
pub trait Prototype: fmt::Debug {
    fn name(&self) -> &String;
}

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
    map_gen_presets: PrototypeCategory<MapGenPresets>,
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
    highlight_box: PrototypeCategory<HighlightBoxEntity>,
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
    tips_and_tricks_item: PrototypeCategory<TipsAndTricksItem>,
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
    pub fn find<T: DataTableAccessable>(&self, name: &str) -> Result<&T, PrototypesErr> {
        T::find(self, name)
    }

    /// Shorthand for [DataTableAccessable::extend]
    pub fn extend<T: DataTableAccessable>(&mut self, prototype: T) -> Result<(), PrototypesErr> {
        prototype.extend(self)
    }

    /// Creates new reference and keeps track of it to later be validated through [Self::validate_references]
    pub fn new_reference<T: DataTableAccessable + 'static>(&mut self, name: String) -> Rc<PrototypeReference<T>> {
        let prot_reference = Rc::new(PrototypeReference::<T>::new(name));
        self.references.push(Rc::downgrade(&(prot_reference.clone() as Rc<dyn PrototypeReferenceValidate>)));
        prot_reference
    }

    /// Validates all tracked references.
    pub fn validate_references(&self) -> Result<(), PrototypesErr> {
        for prot_reference in &self.references {
            if let Some(pref) = prot_reference.upgrade() {
                pref.validate(self)?
            }
        }
        Ok(())
    }

    /// Cleanup up Weak references
    pub fn references_cleanup(&mut self) {
        self.references.retain(|r| r.upgrade().is_some())
    }

    /// Create new resource record
    pub fn register_resource(&mut self, resource_record: ResourceRecord) {
        self.resource_records.push(resource_record);
    }

    /// Validate resources
    /// callback is a function that should find the file and perform necessary checks, returning
    /// the Result of the check.
    pub fn validate_resources<F: Fn(&ResourceRecord) -> Result<(), ResourceError>>(&self, callback: F) -> Result<(), ResourceError> {
        for resource_record in &self.resource_records {
            callback(resource_record)?;
        }
        Ok(())
    }
}

/// [mlua::FromLua] alternative with [DataTable] reference being passed
pub trait PrototypeFromLua<'lua>: Sized {
    fn prototype_from_lua(value: Value<'lua>, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<Self>;
}

impl<'lua, T: PrototypeFromLua<'lua>> PrototypeFromLua<'lua> for Vec<T> {
    fn prototype_from_lua(value: Value<'lua>, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<Self> {
        if let Value::Table(value) = value {
            value.sequence_values::<Value>().collect::<LuaResult<Vec<Value>>>()?.into_iter().map(|v| T::prototype_from_lua(v, lua, data_table)).collect()
        } else {
            Err(mlua::Error::FromLuaConversionError{
                from: value.type_name(),
                to: "Vec",
                message: Some("expected table".into())
            })
        }
    }
}

impl<'lua, T: PrototypeFromLua<'lua>> PrototypeFromLua<'lua> for Option<T> {
    #[inline]
    fn prototype_from_lua(value: Value<'lua>, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<Self> {
        match value {
            Value::Nil => Ok(None),
            value => Ok(Some(T::prototype_from_lua(value, lua, data_table)?))
        }
    }
}

impl<'lua> PrototypeFromLua<'lua> for Value<'lua> {
    #[inline]
    fn prototype_from_lua(value: Value<'lua>, _lua: &'lua Lua, _data_table: &mut DataTable) -> LuaResult<Self> {
        Ok(value)
    }
}

impl<'lua, K, V> PrototypeFromLua<'lua> for HashMap<K, V>
where
    K: Eq + Hash + FromLua<'lua>,
    V: PrototypeFromLua<'lua>,
{
    fn prototype_from_lua(value: Value<'lua>, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<Self> {
        HashMap::<K, Value>::from_lua(value, lua)?.into_iter().map(|(k, v)| Ok((k, V::prototype_from_lua(v, lua, data_table)?))).collect()
    }
}

// "Manual" PrototypeFromLua implementations for types that implement FromLua
// The reason I can't do impl<T> PrototypeFromLua for T is because I need special handling for Vec
// and Option to allow to pass through DataTable reference
prot_from_lua_blanket!(String);
prot_from_lua_blanket!(f64);
prot_from_lua_blanket!(f32);
prot_from_lua_blanket!(bool);
prot_from_lua_blanket!(u64);
prot_from_lua_blanket!(u32);
prot_from_lua_blanket!(u16);
prot_from_lua_blanket!(u8);
prot_from_lua_blanket!(i64);
prot_from_lua_blanket!(i32);
prot_from_lua_blanket!(i16);
prot_from_lua_blanket!(i8);
prot_from_lua_blanket!(Color);
prot_from_lua_blanket!(Factorio2DVector);
#[cfg(feature = "concepts")]
prot_from_lua_blanket!(LocalisedString);

fn prot_from_lua_err(cond: bool, type_name: &'static str, message: impl ToString) -> LuaResult<()> {
    if cond {
        Err(mlua::Error::FromLuaConversionError { from: "table", to: type_name, message: Some(message.to_string()) })
    } else {
        Ok(())
    }
}

/// Trait for getting a prototype from table
trait GetPrototype<'lua> {
    fn get_prot<K: ToLua<'lua>, V: PrototypeFromLua<'lua>>(&self, key: K, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<V>;
}

impl<'lua> GetPrototype<'lua> for mlua::Table<'lua> {
    fn get_prot<K: ToLua<'lua>, V: PrototypeFromLua<'lua>>(&self, key: K, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<V> {
        let value = self.get::<K, Value>(key)?;
        V::prototype_from_lua(value, lua, data_table)
    }
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
        matches!(self.find(data_table), Ok(_))
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
    fn find<'a>(data_table: &'a DataTable, name: &str) -> Result<&'a Self, PrototypesErr>;
    /// Extend [Data table](DataTable) with this prototype
    fn extend(self, data_table: &mut DataTable) -> Result<(), PrototypesErr>;
}

/// Struct for recording resources (images, sound files)
#[derive(Debug)]
pub struct ResourceRecord {
    pub path: String,
    pub resource_type: ResourceType
}

/// Resource type with additional info if needed
#[derive(Debug)]
pub enum ResourceType {
    Image(SpriteSizeType, SpriteSizeType), // x and y dimensions of an image
    Sound // Only .ogg, .wav and .voc are accepted
}

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("File not found: \"{0}\"")]
    FileNotFound(String),
    #[error("Image size incorrect: Expected at least {0}x{1}, got {2}x{3}")]
    ImageSizeIncorrect(SpriteSizeType, SpriteSizeType, SpriteSizeType, SpriteSizeType)
}

pub trait ModSetting: Prototype {
    fn localised_name(&self) -> &Option<LocalisedString>;
    fn localised_description(&self) -> &Option<LocalisedString>;
    fn order(&self) -> &Option<String>;
    fn hidden(&self) -> bool; // Default: false
    fn setting_type(&self) -> ModSettingType;
}

#[derive(Debug, Clone, Prototype, ModSetting, DataTableAccessable, PrototypeFromLua)]
#[data_table(bool_setting)]
pub struct BoolModSetting {
    name: String,
    localised_name: Option<LocalisedString>,
    localised_description: Option<LocalisedString>,
    order: Option<String>,
    #[default(false)]
    hidden: bool,
    #[from_str]
    setting_type: ModSettingType,
    pub default_value: bool,
    #[mandatory_if(hidden)]
    pub forced_value: Option<bool>,
}

#[derive(Debug, Clone, Prototype, ModSetting, DataTableAccessable, PrototypeFromLua)]
#[data_table(int_setting)]
pub struct IntModSetting {
    name: String,
    localised_name: Option<LocalisedString>,
    localised_description: Option<LocalisedString>,
    order: Option<String>,
    #[default(false)]
    hidden: bool,
    #[from_str]
    setting_type: ModSettingType,
    pub default_value: i64,
    pub minimum_value: Option<i64>,
    pub maximum_value: Option<i64>,
    pub allowed_values: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Prototype, ModSetting, DataTableAccessable, PrototypeFromLua)]
#[data_table(double_setting)]
pub struct DoubleModSetting {
    name: String,
    localised_name: Option<LocalisedString>,
    localised_description: Option<LocalisedString>,
    order: Option<String>,
    #[default(false)]
    hidden: bool,
    #[from_str]
    setting_type: ModSettingType,
    pub default_value: f64,
    pub minimum_value: Option<f64>,
    pub maximum_value: Option<f64>,
    pub allowed_values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Prototype, ModSetting, DataTableAccessable, PrototypeFromLua)]
#[data_table(string_setting)]
pub struct StringModSetting {
    name: String,
    localised_name: Option<LocalisedString>,
    localised_description: Option<LocalisedString>,
    order: Option<String>,
    #[default(false)]
    hidden: bool,
    #[from_str]
    setting_type: ModSettingType,
    pub default_value: String,
    #[default(false)]
    pub allow_blank: bool,
    #[default(false)]
    pub auto_trim: bool,
    pub allowed_values: Option<Vec<String>>
}

/// <https://wiki.factorio.com/Prototype/AmbientSound>
#[derive(Debug, Clone, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(ambient_sound)]
pub struct AmbientSoundPrototype {
    name: String,
    pub sound: Sound,
    #[from_str]
    pub track_type: TrackType,
    #[default(1.0)]
    pub weight: f64
}

/// <https://wiki.factorio.com/Prototype/Animation>
#[derive(Debug, Clone, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(animation)]
pub struct AnimationPrototype {
    name: String,
    #[use_self_vec]
    pub layers: Vec<Animation> // If lua table doesn't have layers, use same table for constructing just one
}

/// <https://wiki.factorio.com/Prototype/EditorController>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(editor_controller)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct EditorController {
    name: String, // Must be "default"
    pub inventory_size: ItemStackIndex,
    pub gun_inventory_size: ItemStackIndex,
    pub movement_speed: f64, // Must be >= 0.34375
    pub item_pickup_distance: f64,
    pub loot_pickup_distance: f64,
    pub mining_speed: f64,
    pub enable_flash_light: bool,
    pub adjust_speed_based_off_zoom: bool,
    pub render_as_day: bool,
    pub instant_blueprint_building: bool,
    pub instant_deconstruction: bool,
    pub instant_upgrading: bool,
    pub instant_rail_planner: bool,
    pub show_status_icons: bool,
    pub show_hidden_entities: bool,
    pub show_entity_tags: bool,
    pub show_entity_health_bars: bool,
    pub show_additional_entity_info_gui: bool,
    pub generate_neighbour_chunks: bool,
    pub fill_built_entity_energy_buffers: bool,
    pub show_character_tab_in_controller_gui: bool,
    pub show_infinity_filter_in_controller_gui: bool,
    pub placed_corpses_never_expire: bool
}

impl EditorController {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.name != "default" {
            return Err(mlua::Error::FromLuaConversionError { from: "table", to: "EditorController", message: Some("EditorController name should only be \"default\"".into()) })
        }
        if self.movement_speed < 0.34375 {
            return Err(mlua::Error::FromLuaConversionError { from: "table", to: "EditorController", message: Some("movement speed must be >= 0.34375".into()) })
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Prototype/Font>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(font)]
pub struct Font {
    pub name: String,
    pub size: i32,
    pub from: String,
    #[default(0.0_f32)]
    pub spacing: f32, // Default: 0.0
    #[default(false)]
    pub border: bool, // Default: false
    #[default(false)]
    pub filtered: bool, // Default: false
    pub border_color: Option<Color>
}

/// <https://wiki.factorio.com/Prototype/GodController>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(god_controller)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct GodController {
    pub name: String, // Must be "default"
    pub inventory_size: ItemStackIndex,
    pub movement_speed: f64, // Must be >= 0.34375
    pub item_pickup_distance: f64,
    pub loot_pickup_distance: f64,
    pub mining_speed: f64,
    pub crafting_categories: Option<Vec<String>>,
    pub mining_categories: Option<Vec<String>>
}

impl GodController {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.name != "default" {
            return Err(mlua::Error::FromLuaConversionError { from: "table", to: "GodController", message: Some("GodController name should only be \"default\"".into()) })
        }
        if self.movement_speed < 0.34375 {
            return Err(mlua::Error::FromLuaConversionError { from: "table", to: "GodController", message: Some("movement speed must be >= 0.34375".into()) })
        }
        Ok(())
    }

}

/// <https://wiki.factorio.com/Prototype/MapGenPresets>
#[derive(Debug, Prototype, DataTableAccessable)]
#[data_table(map_gen_presets)]
pub struct MapGenPresets {
    pub name: String,
    pub presets: HashMap<String, MapGenPreset>
}

impl<'lua> PrototypeFromLua<'lua> for MapGenPresets {
    fn prototype_from_lua(value: Value<'lua>, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<Self> {
        if let Value::Table(table) = value {
            let name = table.get::<_, String>("name")?;
            let mut result = HashMap::new();
            for (k, v) in table.pairs::<String, Value>().collect::<LuaResult<HashMap<String, Value>>>()? {
                if k != "name" {
                    result.insert(k, MapGenPreset::prototype_from_lua(v, lua, data_table)?);
                }
            }
            Ok(Self{name, presets: result})
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "MapGenPresets", message: Some("expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Prototype/MapSettings>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(map_settings)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct MapSettings {
    pub name: String, // Must be "map-settings"
    pub pollution: MapPollutionSettings,
    pub steering: MapSteering, // ???
    pub enemy_evolution: MapEnemyEvolution,
    pub enemy_expansion: MapEnemyExpansion,
    pub unit_group: MapUnitGroup,
    pub path_finder: MapPathFinder,
    pub max_ffailed_behavior_count: u32,
    pub difficulty_settings: MapDifficultySettings
}

impl MapSettings {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.name != "map-settings" {
            return Err(mlua::Error::FromLuaConversionError { from: "table", to: "MapSettings", message: Some("`name` of MapSettings prototype must be \"map-settings\"".into()) })
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Prototype/MouseCursor>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(mouse_cursor)]
pub struct MouseCursor {
    pub name: String,
    #[use_self_forced]
    pub cursor: MouseCursorType
}

/// <https://wiki.factorio.com/Prototype/Sound>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(sound)]
pub struct SoundPrototype {
    pub name: String,
    #[use_self_forced]
    pub sound: Sound
}

/// <https://wiki.factorio.com/Prototype/SpectatorController>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[post_extr_fn(Self::post_extr_fn)]
#[data_table(spectator_controller)]
pub struct SpectatorController {
    pub name: String, // Must be "default"
    pub movement_speed: f64 // Must be >= 0.34375
}

impl SpectatorController {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        if self.name != "default" {
            return Err(mlua::Error::FromLuaConversionError{from: "table", to: "SpectatorController", message: Some("`name` must be \"default\" since only one instance can be defined".into())})
        }
        if self.movement_speed < 0.34375 {
            return Err(mlua::Error::FromLuaConversionError{from: "table", to: "SpectatorController", message: Some("`movement_speed` must be >= 0.34375".into())})
        }
        Ok(())
    }
}

/// <https://wiki.factorio.com/Prototype/Sprite>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(sprite)]
pub struct SpritePrototype {
    pub name: String,
    #[use_self_forced]
    pub sprite: Sprite
}

/// <https://wiki.factorio.com/Prototype/TileEffect>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(tile_effect)]
#[post_extr_fn(Self::post_extr_fn)]
pub struct TileEffect {
    pub name: String, // Must be "water" // For some reason
    pub specular_lightness: Color,
    pub foam_color: Color,
    pub foam_color_multiplier: f32,
    pub tick_scale: f32,
    pub animation_speed: f32,
    pub animation_scale: Vec<f32>, // One or two members, same for other below
    pub dark_threshold: Vec<f32>,
    pub reflection_threshold: Vec<f32>,
    pub specular_threshold: Vec<f32>,
    pub texture: Sprite, // Size must be 512 x 512
    #[default(2.0)]
    pub near_zoom: f32, // Default: 2.0
    #[default(0.5)]
    pub far_zoom: f32 // Default: 0.5
}

impl TileEffect {
    fn post_extr_fn(&self, _lua: &Lua, _data_table: &DataTable) -> LuaResult<()> {
        prot_from_lua_err(self.name != "water", "TileEffect", "`name` must be \"water\"")?;
        prot_from_lua_err(
            self.animation_scale.is_empty() || self.animation_scale.len() > 2,
            "TileEffect", "`animation_scale` must have one or two elements")?;
        prot_from_lua_err(
            self.dark_threshold.is_empty() || self.animation_scale.len() > 2,
            "TileEffect", "`animation_scale` must have one or two elements")?;
        prot_from_lua_err(
            self.reflection_threshold.is_empty() || self.animation_scale.len() > 2,
            "TileEffect", "`animation_scale` must have one or two elements")?;
        prot_from_lua_err(
            self.specular_threshold.is_empty() || self.animation_scale.len() > 2,
            "TileEffect", "`animation_scale` must have one or two elements")?;
        // TODO: sprite size check
        Ok(())
    }
}

/// <https://wiki.factorio.com/Prototype/TipsAndTricksItemCategory>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(tips_and_tricks_item_category)]
pub struct TipsAndTricksItemCategory {
    pub name: String,
    pub order: String
}

// 56 instances max // weird number
/// <https://wiki.factorio.com/Prototype/TriggerTargetType>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(trigger_target_type)]
pub struct TriggerTargetType {
    pub name: String
}

/// <https://wiki.factorio.com/Prototype/WindSound>
#[derive(Debug, Prototype, DataTableAccessable, PrototypeFromLua)]
#[data_table(wind_sound)]
pub struct WindSound {
    pub name: String,
    pub sound: Sound
}

// PrototypeBase starts here
/// <https://wiki.factorio.com/PrototypeBase>
#[derive(Debug, PrototypeFromLua)]
pub struct PrototypeBaseSpec {
    pub localised_description: Option<LocalisedString>,
    pub localised_name: Option<LocalisedString>,
    pub order: String
}

/// <https://wiki.factorio.com/PrototypeBase>
pub trait PrototypeBase: Prototype {
    fn localised_description(&self) -> &Option<LocalisedString>;
    fn localised_name(&self) -> &Option<LocalisedString>;
    fn order(&self) -> &String; // Default: ""
}

/// Base for Achievement and all inherited types <https://wiki.factorio.com/Prototype/Achievement>
#[derive(Debug, PrototypeFromLua)]
pub struct AchievementBase {
    #[use_self_forced]
    pub icon: IconSpecification,
    #[default("")]
    pub steam_stats_name: String, // Default: "" // Unusable
    #[default(true)]
    pub allowed_without_fight: bool, // Default: true
    #[default(false)]
    pub hidden: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/Achievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable, PrototypeFromLua)]
#[data_table(achievement)]
pub struct Achievement {
    pub name: String,
    #[use_self_forced]
    pub prototype_base: PrototypeBaseSpec,
    #[use_self_forced]
    pub achievement: AchievementBase
}

/// <https://wiki.factorio.com/Prototype/BuildEntityAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable, PrototypeFromLua)]
#[data_table(build_entity_achievement)]
pub struct BuildEntityAchievement {
    pub name: String,
    #[use_self_forced]
    pub prototype_base: PrototypeBaseSpec,
    #[use_self_forced]
    pub achievement: AchievementBase,
    pub to_build: String, // Name of entity
    #[default(1_u32)]
    pub amount: u32, // Default: 1
    #[default(false)]
    pub limited_to_one_game: bool, // Default: false
    #[default(0_u32)]
    pub until_second: u32 // Default: 0 (means infinite)
}

/// <https://wiki.factorio.com/Prototype/CombatRobotCountAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable, PrototypeFromLua)]
#[data_table(combat_robot_count)]
pub struct CombatRobotCountAchievement {
    pub name: String,
    #[use_self_forced]
    pub prototype_base: PrototypeBaseSpec,
    #[use_self_forced]
    pub achievement: AchievementBase,
    #[default(1_u32)]
    pub count: u32 // Default: 1
}

/// <https://wiki.factorio.com/Prototype/ConstructWithRobotsAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(construct_with_robots_achevement)]
pub struct ConstructWithRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    limited_to_one_game: bool,
    amount: u32, // Default: 0
    more_than_manually: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/DeconstructWithRobotsAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(deconstruct_with_robots_achievement)]
pub struct DeconstructWithRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: u32
}

/// <https://wiki.factorio.com/Prototype/DeliverByRobotsAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(deliver_by_robots_achievement)]
pub struct DeliverByRobotsAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64
}

/// <https://wiki.factorio.com/Prototype/DontBuildEntityAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(dont_build_entity_achievement)]
pub struct DontBuildEntityAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    dont_buid: Vec<String>, // String is converted to Vec<String> with one element
    amount: u32 // Default: 0
}

/// <https://wiki.factorio.com/Prototype/DontCraftManuallyAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(dont_craft_manually_achievement)]
pub struct DontCraftManuallyAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64
}

/// <https://wiki.factorio.com/Prototype/DontUseEntityInEnergyProductionAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(dont_use_entity_in_energy_production_achievement)]
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
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(finish_the_game_achievement)]
pub struct FinishTheGameAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    until_second: u32 // Default: 0 (means infinite)
}

/// <https://wiki.factorio.com/Prototype/GroupAttackAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(group_attack_achievement)]
pub struct GroupAttackAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: u32 // Default: 1
}

/// <https://wiki.factorio.com/Prototype/KillAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(kill_achievement)]
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
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(player_damaged_achievement)]
pub struct PlayerDamagedAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    minimum_damage: f32,
    should_survive: bool,
    type_of_dealer: Option<String> // TODO: another prototype enum?
}

/// <https://wiki.factorio.com/Prototype/ProduceAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(produce_achievement)]
pub struct ProduceAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64,
    limited_to_one_game: bool,
    product: ProductType // Type is determined from item_product or fluid_product // Only one can be set!
}

/// <https://wiki.factorio.com/Prototype/ProducePerHourAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(produce_per_hour_achievement)]
pub struct ProducePerHourAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    amount: f64,
    product: ProductType
}

/// <https://wiki.factorio.com/Prototype/ResearchAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(research_achievement)]
pub struct ResearchAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    target: ResearchTarget // Determined from either `technology` or `research_all` is set
}

/// <https://wiki.factorio.com/Prototype/TrainPathAchievement>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(train_path_achievement)]
pub struct TrainPathAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    achievement: AchievementBase,
    minimum_distance: f64
}

/// <https://wiki.factorio.com/Prototype/AmmoCategory>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(ammo_category)]
pub struct AmmoCategory {
    name: String,
    prototype_base: PrototypeBaseSpec,
    bonus_gui_order: String // Default: ""
}

/// <https://wiki.factorio.com/Prototype/AutoplaceControl>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(autoplace_control)]
pub struct AutoplaceControl {
    name: String,
    prototype_base: PrototypeBaseSpec,
    pub category: AutoplaceControlCategory,
    pub can_be_disabled: bool, // Default: true
    pub richness: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/CustomInput>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(custom_input)]
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
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(damage_type)]
pub struct DamageType {
    name: String,
    prototype_base: PrototypeBaseSpec,
    hidden: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/Decorative>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(optimized_decorative)]
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
    water_reflection: Option<WaterReflectionDefinition>,
    protected_from_tile_building: bool, // Default: true
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
    fn map_color(&self) -> &Option<Color>;
    fn friendly_map_color(&self) -> &Option<Color>;
    fn enemy_map_color(&self) -> &Option<Color>;
    fn water_reflection(&self) -> &Option<WaterReflectionDefinition>;
    fn protected_from_tile_building(&self) -> bool;
}

/// <https://wiki.factorio.com/Prototype/Arrow>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(arrow)]
pub struct Arrow {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    arrow_picture: Sprite,
    circle_picture: Option<Sprite>,
    blinking: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/ArtilleryFlare>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(artillery_flare)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(artillery_projectile)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(beam)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(character_corpse)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(cliff)]
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
#[derive(Debug, Prototype, Corpse, DataTableAccessable)]
#[data_table(corpse)]
pub struct CorpsePrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    corpse_base: CorpseBase
}

/// <https://wiki.factorio.com/Prototype/RailRemnants>
#[derive(Debug, Prototype, Corpse, DataTableAccessable)]
#[data_table(rail_remnants)]
pub struct RailRemnants {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    corpse_base: CorpseBase,
    bending_type: BendingType,
    pictures: RailPictures
}

/// <https://wiki.factorio.com/Prototype/DeconstructibleTileProxy>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(deconstructible_tile_proxy)]
pub struct DeconstructibleTileProxy {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
}

/// <https://wiki.factorio.com/Prototype/EntityGhost>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(entity_ghost)]
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

/// <https://wiki.factorio.com/Prototype/EntityWithOwner>
#[derive(Debug)]
pub struct EntityWithOwnerBase {
    is_military_target: bool, // Default: false
    allow_run_time_change_of_is_military_target: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/EntityWithOwner>
pub trait EntityWithOwner {
    fn is_military_target(&self) -> bool;
    fn allow_run_time_change_of_is_military_target(&self) -> bool;
}

/// <https://wiki.factorio.com/Prototype/Accumulator>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(accumulator)]
pub struct Accumulator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_source: EnergySource,
    pub picture: Sprite,
    pub charge_cooldown: u16,
    pub discharge_cooldown: u16,
    pub charge_animation: Option<Animation>,
    pub charge_light: Option<LightDefinition>,
    pub discharge_animation: Option<Animation>,
    pub discharge_light: Option<LightDefinition>,
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub default_output_signal: Option<SignalIDConnector>
}

/// <https://wiki.factorio.com/Prototype/ArtilleryTurret>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(artillery_turret)]
pub struct ArtilleryTurret {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub gun: String, // Name of a gun item
    pub inventory_size: u16, // Must be > 0
    pub ammo_stack_limit: u32, // Must be > 0
    pub automated_ammo_count: u32,
    pub turret_rotation_speed: f64,
    pub manual_range_modifier: f64, // Must be positive
    pub alert_when_attacking: bool, // Default: true
    pub disable_automatic_firing: bool, // Default: false
    pub base_picture_secondary_draw_order: u8, // Default: 0
    pub base_picture_render_layer: RenderLayer, // Default: "lower-object"
    pub base_shift: Option<Factorio2DVector>,
    pub base_picture: Option<Animation4Way>,
    pub cannon_base_pictures: Option<RotatedSprite>,
    pub cannon_barrel_pictures: Option<RotatedSprite>,
    pub rotating_sound: Option<InterruptibleSound>,
    pub rotating_stopped_sound: Option<Sound>,
    pub turn_after_shooting_cooldown: u16, // Default: 0
    pub cannon_parking_frame_count: u16, // Default: 0
    pub cannon_parking_speed: u16, // Default: 1
    pub cannon_barrel_recoil_shiftings: Option<Vec<Factorio3DVector>>,
    pub cannon_barrel_recoil_shiftings_load_correction_matrix: Option<Vec<Factorio3DVector>>, // Only loaded if cannon_barrel_recoil_shiftings is loaded
    cannon_barrel_light_direction: Option<Factorio3DVector> // Only loaded if cannon_barrel_recoil_shiftings is loaded
}

/// <https://wiki.factorio.com/Prototype/Beacon>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(beacon)]
pub struct Beacon {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_usage: Energy,
    pub energy_source: EnergySource,
    pub supply_area_distance: f64,
    pub distribution_effectivity: f64,
    pub module_specification: ModuleSpecification,
    pub graphics_set: Option<BeaconGraphicsSet>,
    pub animation: Option<Animation>, // Loaded only if `graphics_set` is not present
    pub base_picture: Option<Sprite>, // Loaded only if `graphics_set` is not present
    pub radius_visualization_picture: Option<Sprite>,
    pub allowed_effects: EffectTypeLimitation // Default: No effects are allowed
}

/// <https://wiki.factorio.com/Prototype/Boiler>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(boiler)]
pub struct Boiler {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_source: EnergySource,
    pub fluid_box: FluidBox,
    pub output_fluid_box: FluidBox,
    pub energy_consumption: Energy,
    pub burning_cooldown: u32,
    pub target_temperature: f64,
    pub structure: Animation4Way,
    pub fire: Animation4Way, // Can be empty
    pub fire_glow: Animation4Way, // Can be empty
    pub fire_glow_flicker_enabled: bool, // Default: false
    pub fire_flicker_enabled: bool, // Default: false
    pub mode: BoilerMode, // Default: "heat-water-inside"
    pub patch: Option<Sprite4Way>,
}

/// <https://wiki.factorio.com/Prototype/BurnerGenerator>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(burner_generator)]
pub struct BurnerGenerator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_source: EnergySource, // Emissions are ignored
    pub burner: EnergySource, // Must be a burner energy source
    pub animation: Animation4Way,
    pub max_power_output: Energy,
    pub idle_animation: Option<Animation4Way>,
    pub always_draw_idle_animation: bool, // Default: false
    pub min_perceived_performance: f64, // Default: 0.25
    pub performance_to_sound_speedup: f64, // Default: 0.5
}

/// <https://wiki.factorio.com/Prototype/Character>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(character)]
pub struct Character {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub mining_speed: f64,
    pub running_speed: f64,
    pub distance_per_frame: f64,
    pub maximum_corner_sliding_distance: f64,
    pub heartbeat: Sound,
    pub eat: Sound,
    pub inventory_size: ItemStackIndex,
    pub build_distance: u32,
    pub drop_item_distance: u32,
    pub reach_distance: u32,
    pub reach_resource_distance: f64,
    pub item_pickup_distance: f64,
    pub loot_pickup_distance: f64,
    pub ticks_to_keep_gun: u32,
    pub ticks_to_keep_aiming_direction: u32,
    pub ticks_to_stay_in_combat: u32,
    pub damage_hit_tint: Color,
    pub running_sound_animation_positions: Vec<f32>,
    pub mining_with_tool_particles_animation_positions: Vec<f32>,
    pub animations: Vec<CharacterArmorAnimation>,
    pub crafting_categories: Option<Vec<String>>, // (Names) Name of crafting category
    pub mining_categories: Option<Vec<String>>, // (Names) Name of mining category
    pub light: Option<LightDefinition>,
    pub enter_vehicle_distance: f64, // Default: 3.0
    pub tool_attack_distance: f64, // Default: 1.5
    pub respawn_time: u32, // Default: 10
    pub has_belt_immunity: bool, // Default: false
    pub character_corpse: Option<String>,
    pub footstep_particle_triggers: Option<FootstepTriggerEffectList>,
    pub synced_footstep_particle_triggers: Option<FootstepTriggerEffectList>,
    pub footprint_particles: Option<Vec<FootprintParticle>>,
    pub left_footprint_offset: Option<Factorio2DVector>,
    pub right_footprint_offset: Option<Factorio2DVector>,
    pub right_footprint_frames: Option<Factorio2DVector>,
    pub left_footprint_frames: Option<Factorio2DVector>,
    pub tool_attack_result: Option<Trigger>,
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
#[derive(Debug, Prototype, Combinator, DataTableAccessable)]
#[data_table(arithmetic_combinator)]
pub struct ArithmeticCombinator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    combinator_base: CombinatorBase,
    pub plus_symbol_sprites: Sprite4Way,
    pub minus_symbol_sprites: Sprite4Way,
    pub multiply_symbol_sprites: Sprite4Way,
    pub divide_symbol_sprites: Sprite4Way,
    pub modulo_symbol_sprites: Sprite4Way,
    pub power_symbol_sprites: Sprite4Way,
    pub left_shift_symbol_sprites: Sprite4Way,
    pub right_shift_symbol_sprites: Sprite4Way,
    pub and_symbol_sprites: Sprite4Way,
    pub or_symbol_sprites: Sprite4Way,
    pub xor_symbol_sprites: Sprite4Way,
}

/// <https://wiki.factorio.com/Prototype/DeciderCombinator>
#[derive(Debug, Prototype, Combinator, DataTableAccessable)]
#[data_table(decider_combinator)]
pub struct DeciderCombinator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    combinator_base: CombinatorBase,
    pub equal_symbol_sprites: Sprite4Way,
    pub greater_symbol_sprites: Sprite4Way,
    pub less_symbol_sprites: Sprite4Way,
    pub not_equal_symbol_sprites: Sprite4Way,
    pub greater_or_equal_symbol_sprites: Sprite4Way,
    pub less_or_equal_symbol_sprites: Sprite4Way,
}

/// <https://wiki.factorio.com/Prototype/ConstantCombinator>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(constant_combinator)]
pub struct ConstantCombinator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub item_slot_count: u32,
    pub sprites: Sprite4Way,
    pub activity_led_sprites: Sprite4Way,
    pub activity_led_light_offsets: [Factorio2DVector; 4],
    pub circuit_wire_connection_points: [WireConnectionPoint; 4],
    pub activity_led_light: Option<LightDefinition>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/Container>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(container)]
pub struct Container {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub inventory_size: u16,
    pub picture: Sprite,
    pub enable_inventory_bar: bool, // Default: true
    pub scale_info_icons: bool, // Default: false
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>
}

/// <https://wiki.factorio.com/Prototype/LogisticContainer>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(logistic_container)]
pub struct LogisticContainer {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub inventory_size: u16,
    pub picture: Option<Sprite>,
    pub logistic_mode: LogisticMode,
    pub enable_inventory_bar: bool, // Default: true
    pub scale_info_icons: bool, // Default: false
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub max_logistic_slots: Option<u16>, // requester-type must have > 0 and <= 1000 // Storage type must have <= 1
    pub render_not_in_network_icon: bool, // Default: true
    pub opened_duration: u8, // Default: 0
    pub animation: Option<Animation>,
    pub landing_location_offset: Option<Factorio2DVector>,
    pub animation_sound: Option<Sound>
}

/// <https://wiki.factorio.com/Prototype/InfinityContainer>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(infinity_container)]
pub struct InfinityContainer {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub inventory_size: u16, // Can't be 0
    pub picture: Option<Sprite>,
    pub logistic_mode: Option<LogisticMode>,
    pub erase_contents_when_mined: bool,
    pub enable_inventory_bar: bool, // Default: true
    pub scale_info_icons: bool, // Default: false
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub max_logistic_slots: Option<u16>, // requester-type must have > 0 and <= 1000 // Storage type must have <= 1
    pub render_not_in_network_icon: bool, // Default: false
    pub opened_duration: u8, // Default: 0
    pub animation: Option<Animation>,
    pub landing_location_offset: Option<Factorio2DVector>,
    pub animation_sound: Option<Sound>,
    pub gui_mode: GuiMode // Default: "none"
}

/// <https://wiki.factorio.com/Prototype/CraftingMachine>
#[derive(Debug)]
pub struct CraftingMachineBase {
    // If module inventory size > 0 and no effects allowed, its' and error
    // https://discord.com/channels/139677590393716737/306402592265732098/898733801679757332
    energy_usage: Energy, // Must be positive
    crafting_speed: f64, // Must be positive
    crafting_categories: Vec<String>, // (Names) Name of crafting category
    energy_source: EnergySource, // if drain is not specified, automatically set to energy_usage / 30
    fluid_boxes: Option<Vec<FluidBox>>,
    allowed_effects: EffectTypeLimitation, // Default: no effects are allowed
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
    fn allowed_effects(&self) -> EffectTypeLimitation;
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
#[derive(Debug, Prototype, CraftingMachine, DataTableAccessable)]
#[data_table(assembling_machine)]
pub struct AssemblingMachine {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    crafting_machine_base: CraftingMachineBase,
    pub fixed_recipe: String, // Default: "" // Name of Recipe
    pub gui_title_key: String, // Default: ""
    pub ingredient_count: u8, // Default: 255
}

/// <https://wiki.factorio.com/Prototype/RocketSilo>
#[derive(Debug, Prototype, CraftingMachine, DataTableAccessable)]
#[data_table(rocket_silo)]
pub struct RocketSilo {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    crafting_machine_base: CraftingMachineBase,
    pub fixed_recipe: String, // Default: "" // Name of Recipe
    pub gui_title_key: String, // Default: ""
    pub ingredient_count: u8, // Default: 255
    pub active_energy_usage: Energy,
    pub idle_energy_usage: Energy,
    pub lamp_energy_usage: Energy,
    pub rocket_entity: String, // Name of RocketSiloRocket
    pub satellite_animation: Animation,
    pub satellite_shadow_animation: Animation,
    pub arm_01_back_animation: Animation,
    pub arm_02_right_animation: Animation,
    pub arm_03_front_animation: Animation,
    pub shadow_sprite: Sprite,
    pub hole_sprite: Sprite,
    pub hole_light_sprite: Sprite,
    pub rocket_shadow_overlay_sprite: Sprite,
    pub rocket_glow_overlay_sprite: Sprite,
    pub door_back_sprite: Sprite,
    pub door_front_sprite: Sprite,
    pub base_day_sprite: Sprite,
    pub base_front_sprite: Sprite,
    pub red_lights_back_sprites: Sprite,
    pub red_lights_front_sprites: Sprite,
    pub hole_clipping_box: BoundingBox,
    pub door_back_open_offset: Factorio2DVector,
    pub door_front_open_offset: Factorio2DVector,
    pub silo_fade_out_start_distance: f64,
    pub silo_fade_out_end_distance: f64,
    pub times_to_blink: u8,
    pub light_blinking_speed: f64,
    pub door_opening_speed: f64,
    pub rocket_parts_required: u32,
    pub base_night_sprite: Option<Sprite>,
    pub base_light: Option<LightDefinition>,
    pub base_engine_light: Option<LightDefinition>,
    pub alarm_trigger: Option<TriggerEffect>,
    pub clamps_on_trigger: Option<TriggerEffect>,
    pub clamps_off_trigger: Option<TriggerEffect>,
    pub doors_trigger: Option<TriggerEffect>,
    pub raise_rocket_trigger: Option<TriggerEffect>,
    pub alarm_sound: Option<Sound>,
    pub clamps_on_sound: Option<Sound>,
    pub clamps_off_sound: Option<Sound>,
    pub doors_sound: Option<Sound>,
    pub raise_rocket_sound: Option<Sound>,
    pub flying_sound: Option<Sound>,
    pub rocket_result_inventory_size: u16 // Default: 0
}

/// <https://wiki.factorio.com/Prototype/Furnace>
#[derive(Debug, Prototype, CraftingMachine, DataTableAccessable)]
#[data_table(furnace)]
pub struct Furnace {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    crafting_machine_base: CraftingMachineBase,
    pub result_inventory_size: u16,
    pub source_inventory_size: u16 // Not more than 1
}

/// <https://wiki.factorio.com/Prototype/ElectricEnergyInterface>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(electric_energy_interface)]
pub struct ElectricEnergyInterface {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_source: EnergySource, // Must be electric
    pub energy_production: Energy, // Default: 0
    pub energy_usage: Energy, // Default: 0
    pub gui_mode: GuiMode, // Default: "none"
    pub continuous_animation: bool, // Default: false
    pub render_layer: RenderLayer, // Default: "object"
    pub light: Option<LightDefinition>,
    pub visuals: ElectricEnergyInterfaceVisuals
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
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(electric_pole)]
pub struct ElectricPole {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub pictures: RotatedSprite,
    pub supply_area_distance: f64, // Max value: 64
    pub connection_points: Vec<WireConnectionPoint>,
    pub radius_visualisation_picture: Option<Sprite>,
    pub active_picture: Option<Sprite>,
    pub maximum_wire_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub light: Option<LightDefinition>,
    pub track_coverage_during_build_by_moving: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/EnemySpawner>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(unit_spawner)]
pub struct EnemySpawner {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub animations: Vec<AnimationVariation>,
    pub max_count_of_owned_units: u32,
    pub max_friends_around_to_spawn: u32,
    pub spawning_cooldown: [f64; 2],
    pub spawning_radius: f64,
    pub spawning_spacing: f64,
    pub max_richness_for_spawn_shift: f64,
    pub max_spawn_shift: f64,
    pub pollution_absorption_absolute: f64,
    pub pollution_absorption_proportional: f64,
    pub call_for_help_radius: f64,
    pub result_units: Vec<UnitSpawnDefinition>,
    pub dying_sound: Option<Sound>,
    pub integration: Vec<SpriteVariation>,
    pub min_darkness_to_spawn: f32, // Default: 0.0
    pub max_darkness_to_spawn: f32, // Default: 1.0
    pub random_animation_offset: bool, // Default: true
    pub spawn_decorations_on_expansion: bool, // Default: false
    pub spawn_decoration: Vec<CreateDecorativesTriggerEffectItem>,
    // allow_run_time_change_of_is_military_target must be false
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
#[derive(Debug, Prototype, FlyingRobot, DataTableAccessable)]
#[data_table(combat_robot)]
pub struct CombatRobot {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    flying_robot_base: FlyingRobotBase,
    pub time_to_live: u32,
    pub attack_parameters: AttackParameters,
    pub idle: RotatedAnimation,
    pub shadow_idle: RotatedAnimation,
    pub in_motion: RotatedAnimation,
    pub shadow_in_motion: RotatedAnimation,
    pub range_from_player: f64, // Default: 0
    pub friction: f64, // Default: 0
    pub destroy_action: Option<Trigger>,
    pub follows_player: bool, // Default: false
    pub light: Option<LightDefinition>
}

/// <https://wiki.factorio.com/Prototype/ConstructionRobot>
#[derive(Debug, Prototype, FlyingRobot, DataTableAccessable)]
#[data_table(construction_robot)]
pub struct ConstructionRobot {
    // Must have collision box of zero
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    flying_robot_base: FlyingRobotBase,
    // RobotWithLogisticInterface
    pub max_payload_size: u32,
    pub cargo_centered: Factorio2DVector,
    pub idle: Option<RotatedAnimation>,
    pub in_motion: Option<RotatedAnimation>,
    pub shadow_idle: Option<RotatedAnimation>,
    pub shadow_in_motion: Option<RotatedAnimation>,
    pub destroy_action: Option<Trigger>,
    pub draw_cargo: bool, // Default: true
    // ConstructionRobot
    pub construction_vector: Factorio2DVector,
    pub working: Option<RotatedAnimation>,
    pub shadow_working: Option<RotatedAnimation>,
    pub smoke: Option<Animation>,
    pub sparks: Option<Vec<AnimationVariation>>,
    pub repairing_sound: Option<Sound>,
    pub working_light: Option<LightDefinition>
}

/// <https://wiki.factorio.com/Prototype/LogisticRobot>
#[derive(Debug, Prototype, FlyingRobot, DataTableAccessable)]
#[data_table(logistic_robot)]
pub struct LogisticRobot {
    // Must have collision box of zero
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    flying_robot_base: FlyingRobotBase,
    // RobotWithLogisticInterface
    pub max_payload_size: u32,
    pub cargo_centered: Factorio2DVector,
    pub idle: Option<RotatedAnimation>,
    pub in_motion: Option<RotatedAnimation>,
    pub shadow_idle: Option<RotatedAnimation>,
    pub shadow_in_motion: Option<RotatedAnimation>,
    pub destroy_action: Option<Trigger>,
    pub draw_cargo: bool, // Default: true
    // LogisticRobot
    pub idle_with_cargo: Option<RotatedAnimation>,
    pub in_motion_with_cargo: Option<RotatedAnimation>,
    pub shadow_idle_with_cargo: Option<RotatedAnimation>,
    pub shadow_in_motion_with_cargo: Option<RotatedAnimation>
}

/// <https://wiki.factorio.com/Prototype/Gate>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(gate)]
pub struct Gate {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub vertical_animation: Animation,
    pub horizontal_animation: Animation,
    pub vertical_rail_animation_left: Animation,
    pub vertical_rail_animation_right: Animation,
    pub horizontal_rail_animation_left: Animation,
    pub horizontal_rail_animation_right: Animation,
    pub vertical_rail_base: Animation,
    pub horizontal_rail_base: Animation,
    pub wall_patch: Animation,
    pub opening_speed: f32,
    pub activation_distance: f64,
    pub timeout_to_close: u32,
    pub open_sound: Sound,
    pub close_sound: Sound,
    pub fadeout_interval: u32, // Default: 0
    pub opened_collision_mask: CollisionMask // Default: ["object-layer", "item-layer", "floor-layer", "water-tile"]
}

/// <https://wiki.factorio.com/Prototype/Generator>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(generator)]
pub struct Generator {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_source: EnergySource, // Must be electric
    pub fluid_box: FluidBox,
    pub horizontal_animation: Animation,
    pub vertical_animation: Animation,
    pub effectivity: f64,
    pub fluid_usage_per_tick: f64,
    pub maximum_temperature: f64,
    pub smoke: Option<Vec<SmokeSource>>, // 1 or more, if specified
    pub burns_fluid: bool, // Default: false
    pub scale_fluid_usage: bool, // Default: false
    pub min_perceived_performance: f64, // Default: 0.25
    pub performance_to_sound_speedup: f64, // Default: 0.5
    pub max_power_output: Option<Energy>,
    pub destroy_non_fuel_fluid: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/HeatInterface>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(heat_interface)]
pub struct HeatInterface {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub heat_buffer: HeatBuffer,
    pub picture: Option<Sprite>,
    pub guid_mode: GuiMode, // Default: "all"
}

/// <https://wiki.factorio.com/Prototype/HeatPipe>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(heat_pipe)]
pub struct HeatPipe {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub connection_sprites: ConnectableEntityGraphics,
    pub heat_glow_sprites: ConnectableEntityGraphics,
    pub heat_buffer: HeatBuffer
}

/// <https://wiki.factorio.com/Prototype/Inserter>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(inserter)]
pub struct Inserter {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub extension_speed: f64,
    pub rotation_speed: f64,
    pub insert_position: Factorio2DVector,
    pub pickup_position: Factorio2DVector,
    pub platform_picture: Sprite4Way,
    pub hand_base_picture: Sprite,
    pub hand_open_picture: Sprite,
    pub hand_closed_picture: Sprite,
    pub energy_source: EnergySource, // Emissions are ignored
    pub energy_per_movement: Energy, // Default: 0
    pub energy_per_rotation: Energy, // Default: 0
    pub stack: bool, // Default: false
    pub allow_custom_vectors: bool, // Default: false
    pub allow_burner_leech: bool, // Default: false
    pub draw_held_item: bool, // Default: true
    pub use_easter_egg: bool, // Default: true
    pub filter_count: u8, // Default: 0
    pub hand_base_shadow: Option<Sprite>,
    pub hand_open_shadow: Option<Sprite>,
    pub hand_closed_shadow: Option<Sprite>,
    pub hand_size: f64, // Default: 0.75
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub default_stack_control_input_signal: Option<SignalIDConnector>,
    pub draw_inserter_arrow: bool, // Default: true
    pub chases_belt_items: bool, // Default: true
    pub stack_size_bonus: u32, // Default: 0
    pub circuit_wire_connection_points: Option<Vec<WireConnectionPoint>>,
    pub circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>
}

/// <https://wiki.factorio.com/Prototype/Lab>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(lab)]
pub struct Lab {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_usage: Energy,
    pub energy_source: EnergySource,
    pub on_animation: Animation,
    pub off_animation: Animation,
    pub inputs: Vec<String>, // (Names) Name of science pack items
    pub researching_speed: f64, // Default: 1
    pub allowed_effects: EffectTypeLimitation, // Default: all allowed
    pub light: Option<LightDefinition>,
    pub base_productivity: f32, // Default: 0
    pub entity_info_icon_shift: Factorio2DVector, // Default: (0, 0)
    pub module_specification: Option<ModuleSpecification>
}

/// <https://wiki.factorio.com/Prototype/Lamp>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(lamp)]
pub struct Lamp {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub picture_on: Sprite,
    pub picture_off: Sprite,
    pub energy_usage_per_tick: Energy,
    pub energy_source: EnergySource, // Must be electric or void, emissions are ignored
    pub light: Option<LightDefinition>,
    pub light_when_colored: Option<LightDefinition>,
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub glow_size: f32, // Default: 0
    pub glow_color_intensity: f32, // Default: 0
    pub darkness_for_all_lamps_on: f32, // Default: 0.5
    pub darkness_for_all_lamps_off: f32, // Default: 0.3
    pub always_on: bool, // Default: false
    pub signal_to_color_mapping: Option<Vec<SignalColorMapping>>,
    pub glow_render_mode: GlowRenderMode // Default: "additive"
}

/// <https://wiki.factorio.com/Prototype/LandMine>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(land_mine)]
pub struct LandMine {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub picture_safe: Sprite,
    pub picture_set: Sprite,
    pub trigger_radius: f64,
    pub picture_set_enemy: Option<Sprite>,
    pub timeout: u32, // Default: 120
    pub action: Option<Trigger>,
    pub ammo_category: Option<String>, // Name of AmmoCategory
    pub force_die_on_attack: bool, // Default: true
    pub trigger_force: ForceCondition, // Default: "enemy"
    pub trigger_collision_mask: CollisionMask, // Default: ["item-layer", "object-layer", "player-layer", "water-tile"]
}

/// <https://wiki.factorio.com/Prototype/LinkedContainer>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(linked_container)]
pub struct LinkedContainer {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub inventory_size: u16, // Must be >0
    pub picture: Option<Sprite>,
    pub gui_mode: GuiMode, // Default: "all"
    pub scale_info_icons: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/Market>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(market)]
pub struct Market {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub picture: Sprite,
    pub allow_access_to_all_forces: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/MiningDrill>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(mining_drill)]
pub struct MiningDrill {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub vector_to_place_result: Factorio2DVector,
    pub resource_searching_radius: f64,
    pub energy_usage: Energy,
    pub mining_speed: f64,
    pub energy_source: EnergySource,
    pub resource_categories: Vec<String>, // (Names) Name of resourceCategory
    pub output_fluid_box: Option<FluidBox>,
    pub input_fluid_box: Option<FluidBox>,
    pub animations: Option<Animation4Way>, // Loaded only if `graphics_set` is not present
    pub graphics_set: Option<MiningDrillGraphicsSet>,
    pub wet_mining_graphics_set: Option<MiningDrillGraphicsSet>,
    pub base_picture: Option<Sprite4Way>,
    pub allowed_effects: EffectTypeLimitation, // Default: all allowed
    pub radius_visualisation_picture: Option<Sprite>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub base_render_layer: RenderLayer, // Default: "lower-object"
    pub base_productivity: f32, // Default: 0
    pub monitor_visualization_tint: Option<Color>,
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory iff `circuit_wire_max_distance` > 0
    pub module_specification: Option<ModuleSpecification>
}

/// <https://wiki.factorio.com/Prototype/OffshorePump>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(offshore_pump)]
pub struct OffshorePump {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub fluid_box: FluidBox,
    pub pumping_speed: f32, // Must be > 0
    pub fluid: String, // Name of Fluid
    pub graphics_set: Option<OffshorePumpGraphicsSet>, // Mandatory if `picture` is not defined
    pub picture: Option<Sprite4Way>, // Deprecated
    pub min_perceived_performance: f32, // Default: 0.25
    pub fluid_box_tile_collision_test: CollisionMask, // Default: "ground-tile"
    pub adjacent_tile_collision_test: CollisionMask, // Defauylt: "water-tile"
    pub adjacent_tile_collision_mask: CollisionMask, // Default: none
    pub center_collision_mask: CollisionMask, // Default: none
    pub adjacent_tile_collision_box: BoundingBox, // Default: ((-0.05, -0.8), (0.05, -0.7))
    pub placeable_position_visualization: Option<Sprite>,
    pub remove_on_tile_collision: bool, // Default: false
    pub always_draw_fluid: bool, // Default: true
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites> // Mandatory if `circuit_wire_max_distance` > 0
}

/// <https://wiki.factorio.com/Prototype/Pipe>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(pipe)]
pub struct Pipe {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub fluid_box: FluidBox,
    pub horizontal_window_bounding_box: BoundingBox,
    pub vertical_window_bounding_box: BoundingBox,
    pub pictures: PipePictures
}

/// <https://wiki.factorio.com/Prototype/InfinityPipe>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(infinity_pipe)]
pub struct InfinityPipe {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub fluid_box: FluidBox,
    pub horizontal_window_bounding_box: BoundingBox,
    pub vertical_window_bounding_box: BoundingBox,
    pub pictures: PipePictures,
    pub gui_mode: GuiMode
}

/// <https://wiki.factorio.com/Prototype/PipeToGround>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(pipe_to_ground)]
pub struct PipeToGround {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub fluid_box: FluidBox,
    pub pictures: PipeToGroundPictures,
    pub draw_fluid_icon_override: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/PlayerPort>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(player_port)]
pub struct PlayerPort {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub animation: Animation
}

/// <https://wiki.factorio.com/Prototype/PowerSwitch>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(power_switch)]
pub struct PowerSwitch {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub power_on_animation: Animation,
    pub overlay_start: Animation,
    pub overlay_loop: Animation,
    pub led_on: Sprite,
    pub led_off: Sprite,
    pub overlay_start_delay: u8,
    pub circuit_wire_connection_point: WireConnectionPoint,
    pub left_wire_connection_point: WireConnectionPoint,
    pub right_wire_connection_point: WireConnectionPoint,
    pub wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/ProgrammableSpeaker>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(programmable_speaker)]
pub struct ProgrammableSpeaker {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_source: EnergySource, // Must be electric
    pub energy_usage_per_tick: Energy,
    pub sprite: Sprite,
    pub maximum_polyphony: u32,
    pub instruments: Vec<Instrument>,
    pub audible_distance_modifier: f32, // Default: 1
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>
}

/// <https://wiki.factorio.com/Prototype/Pump>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(pump)]
pub struct Pump {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub fluid_box: FluidBox,
    pub energy_source: EnergySource,
    pub energy_usage: Energy,
    pub pumping_speed: f64,
    pub animations: Animation4Way,
    pub fluid_wagon_connector_speed: f64, // Default: 1 / 64.0
    pub fluid_wagon_connector_alignment_tolerance: f64, // Default: 2 / 32.0
    pub fluid_wagon_connector_frame_count: u8, // Default: 1
    pub fluid_animation: Option<Animation4Way>,
    pub glass_pictures: Option<Sprite4Way>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory if `circuit_wire_max_distance` > 0
    pub fluid_wagon_connector_graphics: PumpConnectorGraphicsFluidWagon
}

/// <https://wiki.factorio.com/Prototype/Radar>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(radar)]
pub struct Radar {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_usage: Energy,
    pub energy_per_sector: Energy,
    pub energy_per_nearby_scan: Energy,
    pub energy_source: EnergySource,
    pub pictures: RotatedSprite,
    pub max_distance_of_sector_revealed: u32,
    pub max_distance_of_nearby_sector_revealed: u32,
    pub radius_minimap_visualisation_color: Option<Color>,
    pub rotation_speed: f64, // Default: 0.01
}

/// <https://wiki.factorio.com/Prototype/CurvedRail>
/// <https://wiki.factorio.com/Prototype/Rail>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(curved_rail)]
pub struct CurvedRail {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub pictures: RailPictures,
    pub walking_sound: Option<Sound>,
    pub bending_type: BendingType // Must be "turn"
}

/// <https://wiki.factorio.com/Prototype/StraightRail>
/// <https://wiki.factorio.com/Prototype/Rail>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(straight_rail)]
pub struct StraightRail {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub pictures: RailPictures,
    pub walking_sound: Option<Sound>,
    pub bending_type: BendingType // Must be "straight"
}

/// `collision_box` is hardcoded to ((-0.2, -0.2), (0.2, 0.2))
/// "placeable-off-grid" flag is ignored
/// Rail signals must collide with each other
/// <https://wiki.factorio.com/Prototype/RailChainSignal>
/// <https://wiki.factorio.com/Prototype/RailSignalBase>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(rail_chain_signal)]
pub struct RailChainSignal {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub animation: RotatedAnimation,
    pub rail_piece: Option<Animation>,
    pub green_light: Option<LightDefinition>,
    pub orange_light: Option<LightDefinition>,
    pub red_light: Option<LightDefinition>,
    pub default_red_output_signal: Option<SignalIDConnector>,
    pub default_orange_output_signal: Option<SignalIDConnector>,
    pub default_green_output_signal: Option<SignalIDConnector>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory if `circuit_wire_max_distance` > 0
    pub selection_box_offsets: [Factorio2DVector; 8],
    pub blue_light: Option<LightDefinition>,
    pub default_blue_output_signal: Option<SignalIDConnector>
}

/// `collision_box` is hardcoded to ((-0.2, -0.2), (0.2, 0.2))
/// "placeable-off-grid" flag is ignored
/// Rail signals must collide with each other
/// <https://wiki.factorio.com/Prototype/RailSignal>
/// <https://wiki.factorio.com/Prototype/RailSignalBase>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(rail_signal)]
pub struct RailSignal {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub animation: RotatedAnimation,
    pub rail_piece: Option<Animation>,
    pub green_light: Option<LightDefinition>,
    pub orange_light: Option<LightDefinition>,
    pub red_light: Option<LightDefinition>,
    pub default_red_output_signal: Option<SignalIDConnector>,
    pub default_orange_output_signal: Option<SignalIDConnector>,
    pub default_green_output_signal: Option<SignalIDConnector>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory if `circuit_wire_max_distance` > 0
}

/// <https://wiki.factorio.com/Prototype/Reactor>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(reactor)]
pub struct Reactor {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub working_light_picture: Sprite,
    pub heat_buffer: HeatBuffer,
    pub energy_source: EnergySource,
    pub consumption: Energy,
    // If defined, Number of variations must be >= count of connections defined in `heat_buffer`
    pub connection_patches_connected: Option<SpriteVariations>,
    pub connection_patches_disconnected: Option<SpriteVariations>,
    pub heat_connection_patches_connected: Option<SpriteVariations>,
    pub heat_connection_patches_disconnected: Option<SpriteVariations>,
    pub lower_layer_picture: Option<Sprite>,
    pub heat_lower_layer_picture: Option<Sprite>,
    pub picture: Option<Sprite>,
    pub light: Option<LightDefinition>,
    pub meltdown_action: Option<Trigger>,
    pub neighbour_bonus: f64, // Default: 1
    pub scale_energy_usage: bool, // Default: false
    pub use_fuel_glow_color: bool, // Default: false
    pub default_fuel_glow_color: Color, // Default: (1, 1, 1, 1)
}

/// <https://wiki.factorio.com/Prototype/Roboport>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(roboport)]
pub struct Roboport {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_source: EnergySource, // Must be electric or void
    pub energy_usage: Energy,
    pub recharge_minimum: Energy,
    pub robot_slots_count: ItemStackIndex,
    pub material_slots_count: ItemStackIndex,
    pub base: Sprite,
    pub base_patch: Sprite,
    pub base_animation: Animation,
    pub door_animation_up: Animation,
    pub door_animation_down: Animation,
    pub request_to_open_door_timeout: u32,
    pub recharging_animation: Animation,
    pub spawn_and_station_height: f32,
    pub charge_approach_distance: f32,
    pub logistics_radius: f32, // Can't be negative
    pub construction_radius: f32, // Can'be negative
    pub charging_energy: Energy,
    pub open_door_trigger_effect: Option<TriggerEffect>,
    pub close_door_trigger_effect: Option<TriggerEffect>,
    pub default_available_logistic_output_signal: Option<SignalIDConnector>,
    pub default_total_logistic_output_signal: Option<SignalIDConnector>,
    pub default_available_construction_output_signal: Option<SignalIDConnector>,
    pub default_total_construction_output_signal: Option<SignalIDConnector>,
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub spawn_and_station_shadow_height_offset: f32, // Default: 0
    pub draw_logistic_radius_visualization: bool, // Default: true
    pub draw_construction_radius_visualization: bool, // Default: true
    pub recharging_light: Option<LightDefinition>,
    pub charging_station_count: u32, // Default: 0
    pub charging_distance: f32, // Default: 0
    pub charging_station_shift: Option<Factorio2DVector>,
    pub charging_threshold_distance: f32, // Default: 1
    pub robot_vertical_acceleration: f32, // Default: 0.01
    pub stationing_offset: Option<Factorio2DVector>,
    pub robot_limit: ItemCountType, // Default: u32::MAX
    pub robots_shrink_when_entering_and_exiting: bool, // Default: false
    pub charging_offsets: Option<Vec<Factorio2DVector>>,
    pub logistics_connection_distance: Option<f32> // Must be >= `logistics_radius`
}

/// <https://wiki.factorio.com/Prototype/SimpleEntityWithOwner>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(simple_entity_with_owner)]
pub struct SimpleEntityWithOwner {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub render_layer: RenderLayer, // default: "object"
    pub secondary_draw_order: i8, // Default: 0
    pub random_animation_offset: bool, // Default: false
    pub random_variation_on_create: bool, // Default: true
    pub visuals: SimpleEntityWithOwnerVisuals, // Either `picture`, `pictures` or `animations`
    pub force_visibility: ForceCondition, // Default: "all"
}

/// <https://wiki.factorio.com/Prototype/SimpleEntityWithForce>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(simple_entity_with_force)]
pub struct SimpleEntityWithForce {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub render_layer: RenderLayer, // default: "object"
    pub secondary_draw_order: i8, // Default: 0
    pub random_animation_offset: bool, // Default: false
    pub random_variation_on_create: bool, // Default: true
    pub visuals: SimpleEntityWithOwnerVisuals, // Either `picture`, `pictures` or `animations`
    pub force_visibility: ForceCondition, // Default: "all"
}

/// <https://wiki.factorio.com/Prototype/SolarPanel>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(solar_panel)]
pub struct SolarPanel {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub energy_source: EnergySource, // Must be electric
    pub picture: SpriteVariations,
    pub production: Energy,
    pub overlay: Option<SpriteVariations>
}

/// <https://wiki.factorio.com/Prototype/StorageTank>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(storage_tank)]
pub struct StorageTank {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub fluid_box: FluidBox,
    pub window_bounding_box: BoundingBox,
    pub pictures: StorageTankPictures,
    pub flow_length_in_ticks: u32, // Must be positive
    pub two_direction_only: bool, // Default: false
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>, // Mandatory if `circuit_wire_max_distance` > 0
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>, // Mandatory if `circuit_wire_max_distance` > 0
    pub scale_info_icons: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/TrainStop>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(train_stop)]
pub struct TrainStop {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub animation_ticks_per_frame: u32,
    pub rail_overlay_animations: Option<Animation4Way>,
    pub animations: Option<Animation4Way>,
    pub top_animations: Option<Animation4Way>,
    pub default_train_stopped_signal: Option<SignalIDConnector>,
    pub default_trains_count_signal: Option<SignalIDConnector>,
    pub default_trains_limit_signal: Option<SignalIDConnector>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub color: Option<Color>,
    pub chart_name: bool, // Default: true
    pub light1: Option<TrainStopLight>,
    pub light2: Option<TrainStopLight>,
    pub drawing_boxes: Option<TrainStopDrawingBoxes>,
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>,
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>
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
    AnimationSet(Box<BeltAnimationSet>),
    GraphicsSet(Box<BeltGraphicsSet>)
}

/// <https://wiki.factorio.com/Prototype/TransportBeltConnectable>
pub trait TransportBeltConnectable {
    fn speed(&self) -> f64;
    fn animation_speed_coefficient(&self) -> f64;
    fn belt_animation_set(&self) -> &TransportBeltConnectableGraphics;
}

/// <https://wiki.factorio.com/Prototype/LinkedBelt>
#[derive(Debug, Prototype, TransportBeltConnectable, DataTableAccessable)]
#[data_table(linked_belt)]
pub struct LinkedBelt {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    pub structure: BeltStructureWithSideLoading,
    pub structure_render_layer: RenderLayer, // Default: "object"
    pub allow_clone_connection: bool, // Default: true
    pub allow_blueprint_connection: bool, // Default: true
    pub allow_side_loading: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/Loader1x1>
#[derive(Debug, Prototype, TransportBeltConnectable, DataTableAccessable)]
#[data_table(loader_1x1)]
pub struct Loader1x1 {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    pub structure: BeltStructure,
    pub filter_count: u8,
    pub structure_render_layer: RenderLayer, // Default: "object"
    pub container_distance: f64, // Default: 1.5
    pub belt_length: f64, // Default: 0.5
}

/// <https://wiki.factorio.com/Prototype/Loader1x2>
#[derive(Debug, Prototype, TransportBeltConnectable, DataTableAccessable)]
#[data_table(loader)]
pub struct Loader1x2 {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    pub structure: BeltStructure,
    pub filter_count: u8,
    pub structure_render_layer: RenderLayer, // Default: "object"
    pub container_distance: f64, // Default: 1.5
    pub belt_length: f64, // Default: 0.5
}

/// <https://wiki.factorio.com/Prototype/Splitter>
#[derive(Debug, Prototype, TransportBeltConnectable, DataTableAccessable)]
#[data_table(splitter)]
pub struct Splitter {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    pub structure: Animation4Way,
    pub structure_patch: Option<Animation4Way>,
    pub structure_animation_speed_coefficient: f64, // Default: 1
    pub structure_animation_movement_cooldown: u32, // Default: 10
}

/// <https://wiki.factorio.com/Prototype/TransportBelt>
#[derive(Debug, Prototype, TransportBeltConnectable, DataTableAccessable)]
#[data_table(transport_belt)]
pub struct TransportBelt {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    pub connector_frame_sprites: TransportBeltConnectorFrame,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_wire_connection_point: Option<Vec<WireConnectionPoint>>,
    pub circuit_connector_sprites: Option<Vec<CircuitConnectorSprites>>,
    pub belt_animation_set_indexes: Option<BeltAnimationSetIndexes>,
    pub animations: Option<RotatedAnimation>, // Must have 12 animations
    pub related_underground_belt: Option<String>, // Name of underground belt
}

/// <https://wiki.factorio.com/Prototype/UndergroundBelt>
#[derive(Debug, Prototype, TransportBeltConnectable, DataTableAccessable)]
#[data_table(underground_belt)]
pub struct UndergroundBelt {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    transport_belt_connectable_base: TransportBeltConnectableBase,
    pub max_distance: u8,
    pub structure: BeltStructureWithSideLoading,
    pub underground_sprite: Sprite,
    pub underground_remove_belts_sprite: Option<Sprite>
}

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
pub trait Turret {
    fn attack_parameters(&self) -> &AttackParameters;
    fn folded_animation(&self) -> &RotatedAnimation4Way;
    fn call_for_help_radius(&self) -> f64;
    fn corpse(&self) -> &Option<String>; 
    fn attack_target_mask(&self) -> &Option<TriggerTargetMask>; 
    fn ignore_target_mask(&self) -> &Option<TriggerTargetMask>; 
    fn shoot_in_prepare_state(&self) -> bool; 
    fn turret_base_has_direction(&self) -> bool; 
    fn random_animation_offset(&self) -> bool; 
    fn secondary_animation(&self) -> bool; 
    fn attack_from_start_frame(&self) -> bool; 
    fn allow_turning_when_starting_attack(&self) -> bool; 
    fn base_picture_secondary_draw_order(&self) -> u8; 
    fn gun_animation_secondary_draw_order(&self) -> u8; 
    fn base_picture_render_layer(&self) -> RenderLayer; 
    fn gun_animation_render_layer(&self) -> RenderLayer; 
    fn base_picture(&self) -> &Option<Animation4Way>;
    fn preparing_animation(&self) -> &Option<RotatedAnimation4Way>;
    fn prepared_animation(&self) -> &Option<RotatedAnimation4Way>;
    fn prepared_alternative_animation(&self) -> &Option<RotatedAnimation4Way>;
    fn starting_attack_animation(&self) -> &Option<RotatedAnimation4Way>;
    fn attacking_animation(&self) -> &Option<RotatedAnimation4Way>;
    fn energy_glow_animation(&self) -> &Option<RotatedAnimation4Way>;
    fn ending_attack_animation(&self) -> &Option<RotatedAnimation4Way>;
    fn folding_animation(&self) -> &Option<RotatedAnimation4Way>;
    fn integration(&self) -> &Option<Sprite>;
    fn glow_light_intensity(&self) -> f32; 
    fn starting_attack_sound(&self) -> &Option<Sound>;
    fn dying_sound(&self) -> &Option<Sound>;
    fn preparing_sound(&self) -> &Option<Sound>;
    fn folding_sound(&self) -> &Option<Sound>;
    fn prepared_sound(&self) -> &Option<Sound>;
    fn prepared_alternative_sound(&self) -> &Option<Sound>;
    fn rotation_speed(&self) -> f32; 
    fn preparing_speed(&self) -> f32; 
    fn folded_speed(&self) -> f32; 
    fn folded_speed_secondary(&self) -> f32; 
    fn prepared_speed(&self) -> f32; 
    fn prepared_speed_secondary(&self) -> f32; 
    fn prepared_alternative_speed(&self) -> f32; 
    fn prepared_alternative_speed_secondary(&self) -> f32; 
    fn prepared_alternative_chance(&self) -> f32; 
    fn starting_attack_speed(&self) -> f32; 
    fn attacking_speed(&self) -> f32; 
    fn ending_attack_speed(&self) -> f32; 
    fn folding_speed(&self) -> f32; 
    fn prepare_range(&self) -> f64; 
    fn alert_when_attacking(&self) -> bool; 
    fn spawn_decorations_on_expansion(&self) -> bool; 
    fn spawn_decoration(&self) -> &Option<Vec<CreateDecorativesTriggerEffectItem>>;
}

/// <https://wiki.factorio.com/Prototype/Turret>
#[derive(Debug, Prototype, Turret, DataTableAccessable)]
#[data_table(turret)]
pub struct TurretPrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    turret_base: TurretBase
}

/// <https://wiki.factorio.com/Prototype/AmmoTurret>
#[derive(Debug, Prototype, Turret, DataTableAccessable)]
#[data_table(ammo_turret)]
pub struct AmmoTurret {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    turret_base: TurretBase,
    pub inventory_size: ItemStackIndex,
    pub animated_ammo_count: ItemCountType,
    pub entity_info_icon_shift: Option<Factorio2DVector>
}

/// <https://wiki.factorio.com/Prototype/ElectricTurret>
#[derive(Debug, Prototype, Turret, DataTableAccessable)]
#[data_table(electric_turret)]
pub struct ElectricTurret {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    turret_base: TurretBase,
    pub energy_source: EnergySource
}

// `turret_base_has_direction` must = true
/// <https://wiki.factorio.com/Prototype/FluidTurret>
#[derive(Debug, Prototype, Turret, DataTableAccessable)]
#[data_table(fluid_turret)]
pub struct FluidTurret {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    turret_base: TurretBase,
    pub fluid_buffer_size: f32,
    pub fluid_buffer_input_flow: f32,
    pub activation_buffer_ratio: f32,
    pub fluid_box: FluidBox,
    pub muzzle_light: Option<LightDefinition>,
    pub enough_fuel_indicator_light: Option<LightDefinition>,
    pub not_enough_fuel_indicator_light: Option<LightDefinition>,
    pub muzzle_animation: Option<Animation>,
    pub folded_muzzle_animation_shift: Option<AnimatedVector>,
    pub preparing_muzzle_animation_shift: Option<AnimatedVector>,
    pub prepared_muzzle_animation_shift: Option<AnimatedVector>,
    pub starting_attack_muzzle_animation_shift: Option<AnimatedVector>,
    pub attacking_muzzle_animation_shift: Option<AnimatedVector>,
    pub ending_attack_muzzle_animation_shift: Option<AnimatedVector>,
    pub folding_muzzle_animation_shift: Option<AnimatedVector>,
    pub enough_fuel_indicator_picture: Option<Sprite4Way>,
    pub not_enough_fuel_indicator_picture: Option<Sprite4Way>,
    pub out_of_ammo_alert_icon: Option<Sprite>
}

/// <https://wiki.factorio.com/Prototype/Unit>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(unit)]
pub struct Unit {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub run_animation: RotatedAnimation,
    pub attack_parameters: AttackParameters, // Requires animation in attack_paramaters. Requires ammo_type in attack_paramaters
    pub movement_speed: f32, // Must be >= 0
    pub distance_per_frame: f32,
    pub pollution_to_join_attack: f32,
    pub distraction_cooldown: u32,
    pub vision_distance: f64, // 100 max
    pub rotation_speed: f32, // Default: 0.025
    pub dying_sound: Option<Sound>,
    pub min_pursue_time: u32, // Default: 600
    pub has_belt_immunity: bool, // Default: false
    pub spawning_time_modifier: f64, // Default: 1
    pub max_pursue_distance: f64, // Default: 50
    pub radar_range: u32, // Default: 0
    pub ai_settings: Option<UnitAISettings>,
    pub move_while_shooting: bool, // Default: false
    pub can_open_gates: bool, // Default: false
    pub affected_by_tiles: bool, // Default: false
    pub render_layer: RenderLayer, // Default: "object"
    pub light: Option<LightDefinition>,
    pub walking_sound: Option<Sound>,
    pub alternative_attacking_frame_sequence: Option<UnitAlternativeAttackingFrameSequence>,
    pub running_sound_animation_positions: Option<Vec<f32>>, // Ignored if `walking_sound` is not defined
    // allow_run_time_change_of_is_military_target must be false
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
#[derive(Debug, Prototype, Vehicle, DataTableAccessable)]
#[data_table(car)]
pub struct Car {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    vehicle_base: VehicleBase,
    pub animation: RotatedAnimation,
    pub effectivity: f64,
    pub consumption: Energy,
    pub rotation_speed: f64,
    pub energy_source: EnergySource, // If used from `burner`, must be a burner energy source // Otherwise can also be a void energy source
    pub inventory_size: ItemStackIndex,
    pub turret_animation: Option<RotatedAnimation>,
    pub light_animation: Option<RotatedAnimation>, // Must have the same frame count as `animation`
    pub render_layer: RenderLayer, // Default: "object"
    pub tank_driving: bool, // Default: false
    pub has_belt_immunity: bool, // Default: false
    pub immune_to_tree_impacts: bool, // Default: false
    pub immune_to_rock_impacts: bool, // Default: false
    pub turret_rotation_speed: f64, // Default: 0.01
    pub turret_return_timeout: u32, // Default: 60
    pub light: Option<LightDefinition>,
    pub sound_no_fuel: Option<Sound>,
    pub darkness_to_render_light_animation: f32, // Default: 0.3
    pub track_particle_triggers: Option<FootstepTriggerEffectList>,
    pub guns: Vec<String>, // (Names) Name of gun prototypes
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
#[derive(Debug, Prototype, RollingStock, DataTableAccessable)]
#[data_table(artillery_wagon)]
pub struct ArtilleryWagon {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    vehicle_base: VehicleBase,
    rolling_stock_base: RollingStockBase,
    pub gun: String, // Name of Prototype/Gun
    pub inventory_size: ItemStackIndex, // Must be > 0
    pub ammo_stack_limit: ItemCountType, // Must be > 0
    pub turret_rotation_speed: f64,
    pub manual_range_modifier: f64, // Must be > 0
    pub disable_automatic_firing: bool, // Default: false
    pub cannon_base_pictures: Option<RotatedSprite>,
    pub cannon_barrel_pictures: Option<RotatedSprite>,
    pub rotating_sound: Option<InterruptibleSound>,
    pub rotating_stopped_sound: Option<Sound>,
    pub turn_after_shooting_cooldown: u16, // Default: 0
    pub cannon_parking_frame_count: u16, // Default: 0
    pub cannon_parking_speed: f32, // Default: 1
    pub cannon_base_shiftings: Option<Vec<Factorio2DVector>>, // Must match `cannon_base_pictures` frame count
    pub cannon_barrel_recoil_shiftings: Option<Vec<Factorio3DVector>>,
    pub cannon_barrel_recoil_shiftings_load_correction_matrix: Option<Vec<Factorio3DVector>>, // Only loaded if `cannon_barrel_recoil_shiftings` is loaded
    pub cannon_barrel_light_direction: Option<Factorio3DVector>, // Only loaded if `cannon_barrel_recoil_shiftings` is loaded
}

/// <https://wiki.factorio.com/Prototype/CargoWagon>
#[derive(Debug, Prototype, RollingStock, DataTableAccessable)]
#[data_table(cargo_wagon)]
pub struct CargoWagon {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    vehicle_base: VehicleBase,
    rolling_stock_base: RollingStockBase,
    pub inventory_size: ItemStackIndex
}

/// <https://wiki.factorio.com/Prototype/FluidWagon>
#[derive(Debug, Prototype, RollingStock, DataTableAccessable)]
#[data_table(fluid_wagon)]
pub struct FluidWagon {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    vehicle_base: VehicleBase,
    rolling_stock_base: RollingStockBase,
    pub capacity: f64,
    pub tank_count: u8, // Default: 3 // Must be one of: 1, 2, 3
}

/// <https://wiki.factorio.com/Prototype/Locomotive>
#[derive(Debug, Prototype, RollingStock, DataTableAccessable)]
#[data_table(locomotive)]
pub struct Locomotive {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    vehicle_base: VehicleBase,
    rolling_stock_base: RollingStockBase,
    pub max_power: Energy,
    pub reversing_power_modifier: f64,
    pub energy_source: EnergySource, // Must be burner if used through `burner`, otherwise can also be void
    pub front_light: Option<LightDefinition>,
    pub front_light_pictures: Option<RotatedSprite>,
    pub darkness_to_render_light_animation: f32, // Default: 0.3
    pub max_snap_to_train_stop_distance: f32, // Default: 3.0
}

/// <https://wiki.factorio.com/Prototype/SpiderVehicle>
#[derive(Debug, Prototype, Vehicle, DataTableAccessable)]
#[data_table(spider_vehicle)]
pub struct SpiderVehicle {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    vehicle_base: VehicleBase,
    pub energy_source: EnergySource, // Must be burner if used through `burner`, otherwise can also be void
    pub inventory_size: ItemStackIndex,
    pub graphics_set: SpiderVehicleGraphicsSet,
    pub spider_engine: SpiderEnginePrototype,
    pub height: f32,
    pub chunk_exploration_radius: u32,
    pub movement_energy_consumption: Energy,
    pub automatic_weapon_cycling: bool,
    pub chain_shooting_cooldown_modifier: f32,
    pub torso_rotation_speed: f32, // Default: 1
    pub trash_inventory_size: ItemStackIndex, // Default: 0
    pub guns: Vec<String>, // (Names) Name of gun
}

/// <https://wiki.factorio.com/Prototype/Wall>
#[derive(Debug, Prototype, EntityWithOwner, DataTableAccessable)]
#[data_table(wall)]
pub struct Wall {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    entity_with_owner_base: EntityWithOwnerBase,
    pub pictures: WallPictures,
    pub visual_merge_group: u32, // Default: 0
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    pub circuit_wire_max_distance: f64, // Default: 0
    pub draw_copper_wires: bool, // Default: true
    pub draw_circuit_wires: bool, // Default: true
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub default_output_signal: Option<SignalIDConnector>,
    pub wall_diode_green: Option<Sprite4Way>,
    pub wall_diode_red: Option<Sprite4Way>,
    pub wall_diode_green_light_top: Option<LightDefinition>,
    pub wall_diode_green_light_right: Option<LightDefinition>,
    pub wall_diode_green_light_bottom: Option<LightDefinition>,
    pub wall_diode_green_light_left: Option<LightDefinition>,
    pub wall_diode_red_light_top: Option<LightDefinition>,
    pub wall_diode_red_light_right: Option<LightDefinition>,
    pub wall_diode_red_light_bottom: Option<LightDefinition>,
    pub wall_diode_red_light_left: Option<LightDefinition>,
    pub connected_gate_visualization: Option<Sprite>
}

//
// STOP HERE
//

/// <https://wiki.factorio.com/Prototype/Fish>
#[derive(Debug, Prototype, EntityWithHealth, DataTableAccessable)]
#[data_table(fish)]
pub struct Fish {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pub pictures: Vec<SpriteVariation>
}

/// <https://wiki.factorio.com/Prototype/SimpleEntity>
#[derive(Debug, Prototype, EntityWithHealth, DataTableAccessable)]
#[data_table(simple_entity)]
pub struct SimpleEntity {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pub count_as_rock_for_filtered_deconstruction: bool, // Default: false
    pub render_layer: RenderLayer, // Default: "object"
    pub secondary_draw_order: i8, // Default: 0
    pub random_animation_offset: bool, // Default: false
    pub random_variation_on_create: bool, // Default: true
    pub visuals: SimpleEntityVisuals
}

/// <https://wiki.factorio.com/Prototype/SpiderLeg>
#[derive(Debug, Prototype, EntityWithHealth, DataTableAccessable)]
#[data_table(spider_leg)]
pub struct SpiderLeg {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pub part_length: f64, // Must be > 0
    pub initial_movement_speed: f64,
    pub movement_acceleration: f64,
    pub target_position_randomisation_distance: f64,
    pub minimal_step_size: f64,
    pub movement_based_position_selection_distance: f64,
    pub graphics_set: SpiderLegGraphicsSet,
    pub walking_sound_volume_modifier: f64, // Default: 1
}

/// <https://wiki.factorio.com/Prototype/Tree>
#[derive(Debug, Prototype, EntityWithHealth, DataTableAccessable)]
#[data_table(tree)]
pub struct Tree {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    entity_with_health_base: EntityWithHealthBase,
    pub variation_weights: Option<Vec<f64>>,
    pub darkness_of_burnt_tree: f32, // Default: 0.5
    pub visuals: TreeVisuals,
    // healing_per_tick: default 0.001666
}

/// <https://wiki.factorio.com/Prototype/Explosion>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(explosion)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(flame_thrower_explosion)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(fire)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(stream)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(flying_text)]
pub struct FlyingText {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    speed: f32,
    time_to_live: u32,
    text_alignment: TextAlignment // Default: "left"
}

/// <https://wiki.factorio.com/Prototype/HighlightBoxEntity>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(highlight_box)]
pub struct HighlightBoxEntity {
    // Bruh
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
}

/// <https://wiki.factorio.com/Prototype/ItemEntity>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(item_entity)]
pub struct ItemEntity {
    // Bruh
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
}

/// <https://wiki.factorio.com/Prototype/ItemRequestProxy>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(item_request_proxy)]
pub struct ItemRequestProxy {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase,
    picture: Sprite,
    use_target_entity_alert_icon_shift: bool, // Default: true
}

/// <https://wiki.factorio.com/Prototype/ParticleSource>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(particle_source)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(projectile)]
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
    turn_speed: f32, // Default: 1 // Must be >= 0
    speed_modifier: Factorio2DVector, // Default: (1, 1)
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(resource)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(rocket_silo_rocket)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(rocket_silo_rocket_shadow)]
pub struct RocketSiloRocketShadow {
    name: String,
    prototype_base: PrototypeBaseSpec,
    entity_base: EntityBase
}

/// <https://wiki.factorio.com/Prototype/SmokeWithTrigger>
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(smoke_with_trigger)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(speech_bubble)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(sticker)]
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
#[derive(Debug, Prototype, Entity, DataTableAccessable)]
#[data_table(tile_ghost)]
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
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(active_defense_equipment)]
pub struct ActiveDefenseEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    automatic: bool,
    attack_parameters: AttackParameters
}

/// <https://wiki.factorio.com/Prototype/BatteryEquipment>
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(battery_equipment)]
pub struct BatteryEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
}

/// <https://wiki.factorio.com/Prototype/BeltImmunityEquipment>
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(belt_immunity_equipment)]
pub struct BeltImmunityEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    energy_consumption: Energy
}

/// <https://wiki.factorio.com/Prototype/EnergyShieldEquipment>
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(energy_shield_equipment)]
pub struct EnergyShieldEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    max_shield_value: f32,
    energy_per_shield: Energy
}

/// <https://wiki.factorio.com/Prototype/GeneratorEquipment>
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(generator_equipment)]
pub struct GeneratorEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    power: Energy,
    burner: Option<EnergySource> // Must be a burner
}

/// <https://wiki.factorio.com/Prototype/MovementBonusEquipment>
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(movement_bonus_equipment)]
pub struct MovementBonusEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    energy_consumption: Energy,
    movement_bonus: f64
}

/// <https://wiki.factorio.com/Prototype/NightVisionEquipment>
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(night_vision_equipment)]
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
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(roboport_equipment)]
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
#[derive(Debug, Prototype, Equipment, DataTableAccessable)]
#[data_table(solar_panel_equipment)]
pub struct SolarPanelEquipment {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_base: EquipmentBase,
    power: Energy
}

/// <https://wiki.factorio.com/Prototype/EquipmentCategory>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(equipment_category)]
pub struct EquipmentCategory {
    name: String,
    prototype_base: PrototypeBaseSpec,
}

/// <https://wiki.factorio.com/Prototype/EquipmentGrid>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(equipment_grid)]
pub struct EquipmentGrid {
    name: String,
    prototype_base: PrototypeBaseSpec,
    equipment_categories: Vec<String>, // (Names) Name of Equipment category // HashSet::intersection can be used here if I were to implement it runtime
    width: u32,
    height: u32,
    locked: bool // Default: false
}

/// <https://wiki.factorio.com/Prototype/Fluid>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(fluid)]
pub struct Fluid {
    name: String,
    prototype_base: PrototypeBaseSpec,
    icon: IconSpecification,
    default_temperature: f64,
    base_color: Color,
    flow_color: Color,
    max_temperature: f64, // Default: `default_temperature`
    heat_capacity: Energy, // Default: 1KJ
    fuel_value: Energy, // Default: "0J"
    emissions_multiplier: f64, // Default: 1
    subgroup: String, // Default: "fluid" // Can'be empty string - either nil or a non-empty string
    gas_temperature: f64, // Default: f64::MAX
    hidden: bool, // Default: false
    //auto_barrel: bool, // Default: true // Ignored by mod loader
}

/// <https://wiki.factorio.com/Prototype/FuelCategory>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(fuel_category)]
pub struct FuelCategory {
    name: String,
    prototype_base: PrototypeBaseSpec,
}

// Documentation is confusing, not deriving PrototypeBase because it's probably the intended way
/// <https://wiki.factorio.com/Prototype/GuiStyle>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(gui_style)]
pub struct GuiStyle {
    name: String,
    prototype_base: PrototypeBaseSpec,
    // Some styles are mandatory
    // All properties that are not prototype properties should be considered styles. I'm not sure
    // whether non-style values are allowed, probably not.
    styles: HashMap<String, StyleSpecification>, // God damnit
    default_tileset: FileName, // Default: ""
    default_sprite_scale: f64, // Default: 1
    default_sprite_priority: SpritePriority, // Default: "medium"
}

/// <https://wiki.factorio.com/Prototype/Item>
#[derive(Debug)]
pub struct ItemBase {
    icon: IconSpecification,
    stack_size: u32, // Must be 1 when "not-stackable" flag is set
    place_result: String, // Default: "" // Name of Entity
    placed_as_equipment_result: String, // Default: ""
    subgroup: String, // Default: "other" // Empty text is not allowed
    fuel_category: String, // Default: "" // Must exist when fuel_value is defined // Name of FuelCategory
    burnt_result: String, // Default: "" // Name of Item
    place_as_tile: Option<PlaceAsTile>,
    pictures: Option<SpriteVariations>, // 16 max
    flags: Option<ItemPrototypeFlags>,
    default_request_amount: u32, // Default: `stack_size`
    wire_count: u32, // Default: 0
    fuel_value: Energy, // Default: "0J" // Mandatory for: `fuel_acceleration_multiplier`, `fuel_top_speed_multiplier`, `fuel_emissions_multiplier`, `fuel_glow_color`
    fuel_acceleration_multiplier: f64, // Default: 1.0
    fuel_top_speed_multiplier: f64, // Default: 1.0
    fuel_emissions_multiplier: f64, // Default: 1.0
    fuel_glow_color: Color, // Default: {r=0, g=0, b=0, a=1}
    open_sound: Option<Sound>,
    close_sound: Option<Sound>,
    dark_background_icon: Option<IconSpecification>,
    rocket_launch_products: Option<Vec<ItemProductPrototype>>,
    rocket_launch_product: Option<ItemProductPrototype>,
}

/// <https://wiki.factorio.com/Prototype/Item>
pub trait Item: PrototypeBase {
    fn icon(&self) -> &IconSpecification;
    fn stack_size(&self) -> u32;
    fn place_result(&self) -> &String;
    fn placed_as_equipment_result(&self) -> &String;
    fn subgroup(&self) -> &String;
    fn fuel_category(&self) -> &String;
    fn burnt_result(&self) -> &String;
    fn place_as_tile(&self) -> &Option<PlaceAsTile>;
    fn pictures(&self) -> &Option<SpriteVariations>;
    fn flags(&self) -> &Option<ItemPrototypeFlags>;
    fn default_request_amount(&self) -> u32;
    fn wire_count(&self) -> u32;
    fn fuel_value(&self) -> Energy;
    fn fuel_acceleration_multiplier(&self) -> f64;
    fn fuel_top_speed_multiplier(&self) -> f64;
    fn fuel_emissions_multiplier(&self) -> f64;
    fn fuel_glow_color(&self) -> &Color;
    fn open_sound(&self) -> &Option<Sound>;
    fn close_sound(&self) -> &Option<Sound>;
    fn dark_background_icon(&self) -> &Option<IconSpecification>;
    fn rocket_launch_products(&self) -> &Option<Vec<ItemProductPrototype>>;
    fn rocket_launch_product(&self) -> &Option<ItemProductPrototype>;
}

/// <https://wiki.factorio.com/Prototype/Item>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(item)]
pub struct ItemPrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase
}

/// <https://wiki.factorio.com/Prototype/AmmoItem>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(ammo)]
pub struct AmmoItem {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    ammo_type: Vec<AmmoItemAmmoType>, // if `ammo_type` field is not array - use as definition of ammotype
    magazine_size: f32, // Default: 1 // Must be >= 1
    reload_time: f32, // Default: 0 // Must be >= 0
}

/// <https://wiki.factorio.com/Prototype/Capsule>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(capsule)]
pub struct Capsule {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    capsule_action: CapsuleAction,
    radius_color: Option<Color>
}

/// <https://wiki.factorio.com/Prototype/Gun>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(gun)]
pub struct Gun {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    attack_parameters: AttackParameters
}

/// <https://wiki.factorio.com/Prototype/ItemWithEntityData>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(item_with_entity_data)]
pub struct ItemWithEntityData {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    icon_tintable: Option<IconSpecification>,
    icon_tintable_mask: Option<IconSpecification>
}

/// <https://wiki.factorio.com/Prototype/ItemWithLabel>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(item_with_label)]
pub struct ItemWithLabel {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/ItemWithInventory>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(item_with_inventory)]
pub struct ItemWithInventory {
    // Stack size must be 1
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
    inventory_size: ItemStackIndex,
    item_filters: Vec<String>, // (Names) Name of item
    item_group_filters: Vec<String>, // (Names) Name of item groups
    item_subgroup_filters: Vec<String>, // (Names) Name of item subgroups
    filter_mode: FilterMode, // Default: "whitelist" // If no filters are defined, automatically set to "none"
    filter_message_key: String, // Default: "item-limitation.item-not-allowed-in-this-container-item" // Locale key, probably doesn't need checking
    extends_inventory_by_default: bool, // Default: false
    insertion_priority_mode: InsertionPriorityMode, // Default: "default"
}

/// <https://wiki.factorio.com/Prototype/BlueprintBook>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(blueprint_book)]
pub struct BlueprintBook {
    // Stack size must be 1
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
    inventory_size: ItemStackIndex,
    item_filters: Vec<String>, // (Names) Name of item
    item_group_filters: Vec<String>, // (Names) Name of item groups
    item_subgroup_filters: Vec<String>, // (Names) Name of item subgroups
    filter_mode: Option<FilterMode>, // Default: "whitelist" // If no filters are defined, automatically set to None
    filter_message_key: String, // Default: "item-limitation.item-not-allowed-in-this-container-item" // Locale key, probably doesn't need checking
    extends_inventory_by_default: bool, // Default: false
    insertion_priority_mode: InsertionPriorityMode, // Default: "default"
}

/// <https://wiki.factorio.com/Prototype/ItemWithTags>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(item_with_tags)]
pub struct ItemWithTags {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/SelectionTool>
#[derive(Debug)]
pub struct SelectionToolBase {
    selection_color: Color,
    alt_selection_color: Color,
    selection_mode: SelectionMode,
    alt_selection_mode: SelectionMode,
    selection_cursor_box_type: CursorBoxType,
    alt_selection_cursor_box_type: CursorBoxType,
    reverse_selection_color: Color, // Default: Value of `selection_color`
    selection_count_button_color: Color, // Default: Value of `selection_color`
    alt_selection_count_button_color: Color, // Default: Value of `alt_selection_color`
    reverse_selection_count_button_color: Color, // Default: Value of `reverse_selection_color`
    chart_selection_color: Color, // Default: Value of `selection_color`
    chart_alt_selection_color: Color, // Default: Value of alt_selection_color
    chart_reverse_selection_color: Color, // Default: Value of `reverse_selection_color`
    reverse_selection_mode: SelectionMode, // Default: Value of `selection_mode`
    reverse_selection_cursor_box_type: CursorBoxType, // Default: Value of `selection_cursor_box_type`
    always_include_tiles: bool, // Default: false
    mouse_cursor: String, // Default: "selection-tool-cursor" // Name of Prototype/MouseCursor
    entity_filters: Option<Vec<String>>, // (Names) Name of Entity
    alt_entity_filters: Option<Vec<String>>, // (Names) Name of Entity
    entity_type_filters: Option<Vec<String>>, // (Names) Name of Entity type
    alt_entity_type_filters: Option<Vec<String>>, // (Names) Name of Entity type
    tile_filters: Option<Vec<String>>, // (Names) Name of a Tile
    alt_tile_filters: Option<Vec<String>>, // (Names) Name of a Tile
    entity_filter_mode: FilterMode, // Default: "whitelist"
    alt_entity_filter_mode: FilterMode, // Default: "whitelist"
    tile_filter_mode: FilterMode, // Default: "whitelist"
    alt_tile_filter_mode: FilterMode, // Default: "whitelist"
}

/// <https://wiki.factorio.com/Prototype/SelectionTool>
pub trait SelectionTool: Item {
    fn selection_color(&self) -> &Color;
    fn alt_selection_color(&self) -> &Color;
    fn selection_mode(&self) -> SelectionMode;
    fn alt_selection_mode(&self) -> SelectionMode;
    fn selection_cursor_box_type(&self) -> CursorBoxType;
    fn alt_selection_cursor_box_type(&self) -> CursorBoxType;
    fn reverse_selection_color(&self) -> &Color;
    fn selection_count_button_color(&self) -> &Color;
    fn alt_selection_count_button_color(&self) -> &Color;
    fn reverse_selection_count_button_color(&self) -> &Color;
    fn chart_selection_color(&self) -> &Color;
    fn chart_alt_selection_color(&self) -> &Color;
    fn chart_reverse_selection_color(&self) -> &Color;
    fn reverse_selection_mode(&self) -> SelectionMode;
    fn reverse_selection_cursor_box_type(&self) -> CursorBoxType;
    fn always_include_tiles(&self) -> bool;
    fn mouse_cursor(&self) -> &String;
    fn entity_filters(&self) -> &Option<Vec<String>>;
    fn alt_entity_filters(&self) -> &Option<Vec<String>>;
    fn entity_type_filters(&self) -> &Option<Vec<String>>;
    fn alt_entity_type_filters(&self) -> &Option<Vec<String>>;
    fn tile_filters(&self) -> &Option<Vec<String>>;
    fn alt_tile_filters(&self) -> &Option<Vec<String>>;
    fn entity_filter_mode(&self) -> FilterMode;
    fn alt_entity_filter_mode(&self) -> FilterMode;
    fn tile_filter_mode(&self) -> FilterMode;
    fn alt_tile_filter_mode(&self) -> FilterMode;
}

/// <https://wiki.factorio.com/Prototype/SelectionTool>
#[derive(Debug, Prototype, PrototypeBase, Item, SelectionTool, DataTableAccessable)]
#[data_table(selection_tool)]
pub struct SelectionToolPrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
    selection_tool_base: SelectionToolBase
}

/// <https://wiki.factorio.com/Prototype/BlueprintItem>
#[derive(Debug, Prototype, PrototypeBase, Item, SelectionTool, DataTableAccessable)]
#[data_table(blueprint)]
pub struct BlueprintItem {
    // Stack size must be 1
    // Ignored/forced properties:
    // selection_mode = "blueprint"
    // alt_selection_mode = "blueprint"
    // always_include_tiles = false
    // entity_filters
    // entity_type_filters
    // tile_filters
    // entity_filter_mode
    // tile_filter_mode
    // alt_entity_filters
    // alt_entity_type_filters
    // alt_tile_filters
    // alt_entity_filter_mode
    // alt_tile_filter_mode
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
    selection_tool_base: SelectionToolBase
}

/// <https://wiki.factorio.com/Prototype/CopyPasteTool>
#[derive(Debug, Prototype, PrototypeBase, Item, SelectionTool, DataTableAccessable)]
#[data_table(copy_paste_tool)]
pub struct CopyPasteTool {
    // Stack size must be 1
    // Ignored/forced properties:
    // always_include_tiles = false
    // entity_filters
    // entity_type_filters
    // tile_filters
    // entity_filter_mode
    // tile_filter_mode
    // alt_entity_filters
    // alt_entity_type_filters
    // alt_tile_filters
    // alt_entity_filter_mode
    // alt_tile_filter_mode
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
    selection_tool_base: SelectionToolBase,
    cuts: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/DeconstructionItem>
#[derive(Debug, Prototype, PrototypeBase, Item, SelectionTool, DataTableAccessable)]
#[data_table(deconstruction_item)]
pub struct DeconstructionItem {
    // Stack size must be 1
    // Ignored/forced properties:
    // selection_mode = "deconstruct"
    // alt_selection_mode = "cancel-deconstruct"
    // always_include_tiles = false
    // entity_filters
    // entity_type_filters
    // tile_filters
    // entity_filter_mode
    // tile_filter_mode
    // alt_entity_filters
    // alt_entity_type_filters
    // alt_tile_filters
    // alt_entity_filter_mode
    // alt_tile_filter_mode
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
    selection_tool_base: SelectionToolBase,
    entity_filter_count: ItemStackIndex, // Default: 0 // Can't be > 255 // So u8? Kappa
    tile_filter_count: ItemStackIndex, // Default: 0 // Can't be > 255
}

/// <https://wiki.factorio.com/Prototype/UpgradeItem>
#[derive(Debug, Prototype, PrototypeBase, Item, SelectionTool, DataTableAccessable)]
#[data_table(upgrade_item)]
pub struct UpgradeItem {
    // Stack size must be 1
    // Ignored/forced properties:
    // selection_mode = "upgrade"
    // alt_selection_mode = "cancel-upgrade"
    // always_include_tiles = false
    // entity_filters
    // entity_type_filters
    // tile_filters
    // entity_filter_mode
    // tile_filter_mode
    // alt_entity_filters
    // alt_entity_type_filters
    // alt_tile_filters
    // alt_entity_filter_mode
    // alt_tile_filter_mode
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    default_label_color: Color, // Default: default item text color
    draw_label_for_cursor_render: bool, // Default: false
    selection_tool_base: SelectionToolBase,
    mapper_count: ItemStackIndex, // Default: 0 // Can't be > 255
}

/// <https://wiki.factorio.com/Prototype/Module>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(module)]
pub struct Module {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    category: String, // Name of ModuleCategory
    tier: u32,
    effect: Effect,
    requires_beacon_alt_mode: bool, // Default: true
    limitation: Vec<String>, // (Names) Name of recipe // Not sure if invalid names are accepted
    limitation_blacklist: Vec<String>, // Same as above
    limitation_message_key: String, // Locale key // Not checked at data load
    art_style: Option<String>,
    beacon_tint: RecipeTint
}

/// <https://wiki.factorio.com/Prototype/RailPlanner>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(rail_planner)]
pub struct RailPlanner {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    straight_rail: String, // Name of entity of type "straight-rail" // First item to place must be this rail planner
    curved_rail: String, // Name of entity of type "curved-rail"
}

/// <https://wiki.factorio.com/Prototype/SpidertronRemote>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(spidertron_remote)]
pub struct SpidertronRemote {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    icon_color_indicator_mask: IconSpecification
}

/// <https://wiki.factorio.com/Prototype/Tool>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(tool)]
pub struct Tool {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    durability: Option<f64>, // Must be positive // Mandatory if `infinite` = false, ignored if true
    durability_description_key: String, // Default: "description.durability-key" // May not be longer than 200 characters
    durability_description_value: String, // Default: "description.durability-value" // May not be longer than 200 characters
    infinite: bool, // Default: false // If false, `durability` must be set
}

/// <https://wiki.factorio.com/Prototype/Armor>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(armor)]
pub struct Armor {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    durability: f64, // Must be positive // Mandatory if `infinite` is false
    durability_description_key: String, // Default: "description.durability-key" // May not be longer than 200 characters
    durability_description_value: String, // Default: "description.durability-value" // May not be longer than 200 characters
    infinite: bool, // Default: true
    equipment_grid: Option<String>, // Name of EquipmentGrid
    resistances: Option<Resistances>,
    inventory_size_bonus: Option<ItemStackIndex>
}

/// <https://wiki.factorio.com/Prototype/RepairTool>
#[derive(Debug, Prototype, PrototypeBase, Item, DataTableAccessable)]
#[data_table(repair_tool)]
pub struct RepairTool {
    name: String,
    prototype_base: PrototypeBaseSpec,
    item_base: ItemBase,
    durability: f64, // Must be positive // Mandatory if `infinite` is false
    durability_description_key: String, // Default: "description.durability-key" // May not be longer than 200 characters
    durability_description_value: String, // Default: "description.durability-value" // May not be longer than 200 characters
    infinite: bool, // Default: true
    speed: f32,
    repair_result: Option<Trigger>
}

/// <https://wiki.factorio.com/Prototype/ItemGroup>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(item_group)]
pub struct ItemGroup {
    name: String,
    prototype_base: PrototypeBaseSpec,
    icon: IconSpecification,
    order_in_recipe: String, // Default: `order`
}

/// <https://wiki.factorio.com/Prototype/ItemSubGroup>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(item_subgroup)]
pub struct ItemSubGroup {
    name: String,
    prototype_base: PrototypeBaseSpec,
    group: String, // Name of ItemGroup
}

/// <https://wiki.factorio.com/Prototype/ModuleCategory>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(module_category)]
pub struct ModuleCategory {
    name: String,
    prototype_base: PrototypeBaseSpec
}

/// <https://wiki.factorio.com/Prototype/NamedNoiseExpression>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(noise_expression)]
pub struct NamedNoiseExpression {
    name: String,
    prototype_base: PrototypeBaseSpec,
    expression: NoiseExpression,
    intended_protperty: String,
}

/// <https://wiki.factorio.com/Prototype/NoiseLayer>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(noise_layer)]
pub struct NoiseLayer {
    name: String,
    prototype_base: PrototypeBaseSpec
}

/// <https://wiki.factorio.com/Prototype/Particle>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(optimized_particle)]
pub struct Particle {
    name: String,
    prototype_base: PrototypeBaseSpec,
    pictures: AnimationVariations,
    life_time: u16, // Can't be 1
    shadows: Option<AnimationVariations>,
    draw_shadow_when_on_ground: bool, // Default: true
    regular_trigger_effect: Option<TriggerEffect>,
    ended_in_water_trigger_effect: Option<TriggerEffect>,
    ended_on_ground_trigger_effect: Option<TriggerEffect>,
    render_layer: RenderLayer, // Default: "object"
    render_layer_when_on_ground: RenderLayer, // Default: "lower-object"
    regular_trigger_effect_frequency: u32, // Default: 0 // Can't be 1
    movement_modifier_when_on_ground: f64, // Default: 0.8
    movement_modifier: f64, // Default: 1
    vertical_acceleration: f32, // Default: -0.004 // Has to be >= -0.01 and <= 0.01
    mining_particle_frame_speed: f32, // Default: 0
    fade_away_duration: u16, // Degault: `life-time`, capped to 60. If equals to 0, silently(?) changed to 1
}

/// <https://wiki.factorio.com/Prototype/Recipe>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(recipe)]
pub struct Recipe {
    // recipe with category named "crafting" cannot have fluid ingredients or products
    name: String,
    prototype_base: PrototypeBaseSpec,
    category: String, // Default: "crafting" // Name of RecipeCategory
    subgroup: String, // Default: subgroup of product (if only 1) or main_product if multiple. Required either.
    icon: IconSpecification, // Mandatory for recipe with multiple products and no main_product. Defaults to icon of `main_product` or index 1
    crafting_machine_tint: RecipeTint, // Defaults to all zeros
    // If one of the difficulties is defined, and the other is set to false, `enabled = false` in other and
    // copies over protperties from the first difficulty. If one difficulty is not defined (set to nil), it uses other
    // difficulty's properties. If RecipeData defined directly in table body (if there are no `normal` and `expensive`), set only `normal
    normal: RecipeData,
    expensive: Option<RecipeData>
}

/// <https://wiki.factorio.com/Prototype/RecipeCategory>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(recipe_category)]
pub struct RecipeCategory {
    name: String,
    prototype_base: PrototypeBaseSpec
}

/// <https://wiki.factorio.com/Prototype/ResourceCategory>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(resource_category)]
pub struct ResourceCategory {
    name: String,
    prototype_base: PrototypeBaseSpec
}

/// <https://wiki.factorio.com/Prototype/Shortcut>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(shortcut)]
pub struct Shortcut {
    name: String,
    prototype_base: PrototypeBaseSpec,
    action: ShortcutAction,
    icon: Sprite,
    item_to_spawn: Option<String>, // Name of Item
    technology_to_unlock: Option<String>, // Name of technology
    toggleable: bool, // Default: false
    associated_control_input: String, // Default: ""
    small_icon: Option<Sprite>, // Default: `icon`
    disabled_icon: Option<Sprite>, // Default: `icon`
    disabled_small_icon: Option<Sprite>, // Default: `icon`
    style: ShortcutStyle, // Default: "default"
}

/// <https://wiki.factorio.com/Prototype/Technology>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(technology)]
pub struct Technology {
    name: String,
    prototype_base: PrototypeBaseSpec,
    icon: IconSpecification,
    // Same deal as with Recipe prototype
    normal: TechnologyData,
    expensive: Option<TechnologyData>
}

/// <https://wiki.factorio.com/Prototype/Tile>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(tile)]
pub struct Tile {
    // 255 instances max
    name: String,
    prototype_base: PrototypeBaseSpec,
    collision_mask: CollisionMask,
    layer: u8,
    variants: MainTileTransitions,
    map_color: Color,
    pollution_absorption_per_second: f64,
    transition_overlay_layer_offset: u8, // Default: 0
    layer_group: LayerGroup, // Default: "water" if `draw_in_water_layer` else "ground"
    draw_in_water_layer: bool, // Default: false
    transition_merges_with_tile: Option<String>, // Name of a tile
    effect_color: Color, // Default: all 1
    tint: Color, // Default: all 1
    walking_sound: Option<Sound>,
    build_sound: Option<TileBuildSound>,
    mined_sound: Option<Sound>,
    walking_speed_modifier: f64, // Default: 1
    vehicle_friction_modifier: f64, // Default: 1
    decorative_removal_probability: f32, // Default: 0
    allowed_neighbors: Option<Vec<String>>, // (Names) Name of Tile
    needs_correction: bool, // Default: false
    minable: Option<MinableProperties>,
    next_direction: Option<String>, // Name of a Tile
    can_be_part_of_blueprint: bool, // Default: true
    effect: Option<String>, // Name of TileEffect
    trigger_effect: Option<TriggerEffect>,
    scorch_mark_color: Option<Color>,
    icon: Option<IconSpecification>,
    effect_color_secondary: Option<Color>,
    effect_is_opaque: Option<bool>, // Why is there no default
    transitions: Option<Vec<ExtraTileTransitions>>,
    transitions_between_transitions: Option<Vec<BetweenTileTransitions>>,
    autoplace: Option<AutoplaceSpecification>,
    placeable_by: Option<Vec<ItemToPlace>>,
    check_collision_with_entities: bool, // Default: false
}

/// <https://wiki.factorio.com/Prototype/TipsAndTricksItem>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(tips_and_tricks_item)]
pub struct TipsAndTricksItem {
    name: String,
    prototype_base: PrototypeBaseSpec,
    image: FileName, // Default: ""
    simulation: Option<SimulationDefinition>,
    tag: String, // Default: ""
    category: String, // Default: `name` // Name of TipsAndTricksItemCategory
    indent: u8, // Default: 0
    is_title: bool, // Default: false
    trigger: Option<TipTrigger>,
    skip_trigger: Option<TipTrigger>,
    tutorial: String, // Default: "" // Name of prototype/Tutorial
    starting_status: TipStatus, // Default: "locked"
    dependencies: Vec<String> // (Names) Name of TipsAndTricksItem
}

/// <https://wiki.factorio.com/Prototype/TrivialSmoke>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(trivial_smoke)]
pub struct TrivialSmoke {
    name: String,
    prototype_base: PrototypeBaseSpec,
    animation: Animation,
    duration: u32, // Can't be 0
    glow_animation: Option<Animation>,
    color: Color, // Default: all 0.375
    start_scale: f64, // Default: 1
    movement_slow_down_factor: f64, // Default: 0.995 // Between 1 and 0 (inclusive both sides)
    spread_duration: u32, // Default: 0
    // `fade_in_duration` + `fade_away_duration` must be <= `duration`
    fade_away_duration: u32, // Default: 0
    fade_in_duration: u32, // Default: 0
    glow_fade_away_duration: u32, // Default: `fade_away_duration`
    cyclic: bool, // Default: false
    affected_by_wind: bool, // Default: true
    show_when_smoke_off: bool, // Default: false
    render_layer: RenderLayer, // Default: "smoke"
}

/// <https://wiki.factorio.com/Prototype/Tutorial>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(tutorial)]
pub struct Tutorial {
    name: String,
    prototype_base: PrototypeBaseSpec,
    scenario: String, // filename?
    // trigger // Not listed in other properties but listed in table of contents
}

/// <https://wiki.factorio.com/Prototype/VirtualSignal>
#[derive(Debug, Prototype, PrototypeBase, DataTableAccessable)]
#[data_table(virtual_signal)]
pub struct VirtualSignal {
    name: String,
    prototype_base: PrototypeBaseSpec,
    icon: IconSpecification,
    subgroup: String, // Default: "virtual-signal" // Name of ItemSubGroup
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
    PrototypeNotFound(String),
    #[error("Field {0} is required")]
    FieldRequired(String)
}
