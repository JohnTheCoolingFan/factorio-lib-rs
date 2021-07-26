use std::path::PathBuf;
use crate::concepts::LocalisedString;
use thiserror::Error;
use std::str::FromStr;
use std::fmt;

// Struct representing global `data` table in lua environment
#[derive(Debug)]
pub struct DataTable {
    prototypes: Vec<Box<dyn Prototype>>
}

// Factorio prototypes
// Source info:
// For prototypes: https://wiki.factorio.com/Prototype_definitions
// For settings: https://wiki.factorio.com/Tutorial:Mod_settings

// Prototype
// Contains all values (accessors) for every prototype in the game
trait Prototype: fmt::Debug {
    fn r#type(&self) -> PrototypeType;
    fn name(&self) -> String;
}

#[derive(Debug, Clone)]
enum ModSettingType {
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

trait ModSetting: Prototype {
    fn localised_name(&self) -> Option<LocalisedString>;
    fn localised_description(&self) -> Option<LocalisedString>;
    fn order(&self) -> Option<String>;
    fn hidden(&self) -> Option<bool>;
    fn setting_type(&self) -> ModSettingType;
}

#[derive(Debug)]
struct BoolModSetting<'a> {
    name: String,
    localised_name: Option<LocalisedString<'a>>,
    localised_description: Option<LocalisedString<'a>>,
    order: Option<String>,
    hidden: Option<bool>,
    setting_type: ModSettingType,
    default_value: bool,
    forced_value: Option<bool>,
}

impl Prototype for BoolModSetting<'_> {
    fn r#type(&self) -> PrototypeType { PrototypeType::BoolSetting }
    fn name(&self) -> String { self.name.clone() }
}

impl ModSetting for BoolModSetting<'_> {
    fn localised_name(&self) -> Option<LocalisedString> { self.localised_name.clone() }
    fn localised_description(&self) -> Option<LocalisedString> { self.localised_description.clone() }
    fn order(&self) -> Option<String> { self.order.clone() }
    fn hidden(&self) -> Option<bool> { self.hidden }
    fn setting_type(&self) -> ModSettingType { self.setting_type.clone() }
}

impl BoolModSetting<'_> {
    fn default_value(&self) -> bool { self.default_value }
    fn forced_value(&self) -> Option<bool> { self.forced_value }
}

#[derive(Debug)]
struct IntModSetting<'a> {
    name: String,
    localised_name: Option<LocalisedString<'a>>,
    localised_description: Option<LocalisedString<'a>>,
    order: Option<String>,
    hidden: Option<bool>,
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

impl ModSetting for IntModSetting<'_> {
    fn localised_name(&self) -> Option<LocalisedString> { self.localised_name.clone() }
    fn localised_description(&self) -> Option<LocalisedString> { self.localised_description.clone() }
    fn order(&self) -> Option<String> { self.order.clone() }
    fn hidden(&self) -> Option<bool> { self.hidden }
    fn setting_type(&self) -> ModSettingType { self.setting_type.clone() }
}

impl IntModSetting<'_> {
    fn default_value(&self) -> i64 { self.default_value }
    fn minimum_value(&self) -> Option<i64> { self.minimum_value }
    fn maximum_value(&self) -> Option<i64> { self.maximum_value }
    fn allowed_values(&self) -> Option<Vec<i64>> { self.allowed_values.clone() }
}

#[derive(Debug)]
struct DoubleModSetting<'a> {
    name: String,
    localised_name: Option<LocalisedString<'a>>,
    localised_description: Option<LocalisedString<'a>>,
    order: Option<String>,
    hidden: Option<bool>,
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

impl ModSetting for DoubleModSetting<'_> {
    fn localised_name(&self) -> Option<LocalisedString> { self.localised_name.clone() }
    fn localised_description(&self) -> Option<LocalisedString> { self.localised_description.clone() }
    fn order(&self) -> Option<String> { self.order.clone() }
    fn hidden(&self) -> Option<bool> { self.hidden }
    fn setting_type(&self) -> ModSettingType { self.setting_type.clone() }
}

impl DoubleModSetting<'_> {
    fn default_value(&self) -> f64 { self.default_value }
    fn minimum_value(&self) -> Option<f64> { self.minimum_value }
    fn maximum_value(&self) -> Option<f64> { self.maximum_value }
    fn allowed_values(&self) -> Option<Vec<f64>> { self.allowed_values.clone() }
}

#[derive(Debug)]
struct StringModSetting<'a> {
    name: String,
    localised_name: Option<LocalisedString<'a>>,
    localised_description: Option<LocalisedString<'a>>,
    order: Option<String>,
    hidden: Option<bool>,
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

impl ModSetting for StringModSetting<'_> {
    fn localised_name(&self) -> Option<LocalisedString> { self.localised_name.clone() }
    fn localised_description(&self) -> Option<LocalisedString> { self.localised_description.clone() }
    fn order(&self) -> Option<String> { self.order.clone() }
    fn hidden(&self) -> Option<bool> { self.hidden }
    fn setting_type(&self) -> ModSettingType { self.setting_type.clone() }
}

impl StringModSetting<'_> {
    fn default_value(&self) -> String { self.default_value.clone() }
    fn allow_blank(&self) -> Option<bool> { self.allow_blank }
    fn auto_trim(&self) -> Option<bool> {self.auto_trim }
    fn allowed_values(&self) -> Option<Vec<String>> { self.allowed_values.clone() }
}

// Enum for all prototype types
#[derive(Debug)]
enum PrototypeType {
    // General prototypes
    AmbientSound,
    Animation,
    EditorController,
    Font,
    GodController,
    MapGenPresets,
    MapGenSettings,
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
            PrototypeType::MapGenSettings => "map-settings",
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
            "map-settings" => Ok(PrototypeType::MapGenSettings),
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
}
