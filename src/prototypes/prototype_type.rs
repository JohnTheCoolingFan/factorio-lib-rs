use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, AsRefStr};

/// Enum for all Prototype types available in the game
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, EnumString, AsRefStr, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum PrototypeType {
    // All prototypes
    #[strum(to_string = "ambient-sound")]
    #[serde(rename = "ambient-sound")]
    AmbientSoundPrototype,
    #[strum(to_string = "animation")]
    #[serde(rename = "animation")]
    AnimationPrototype,
    EditorController,
    Font,
    GodController,
    MapGenPresets,
    MapSettings,
    MouseCursor,
    #[strum(to_string = "sound")]
    #[serde(rename = "sound")]
    SoundPrototype,
    SpectatorController,
    #[strum(to_string = "sprite")]
    #[serde(rename = "sprite")]
    SpritePrototype,
    TileEffect,
    TipsAndTricksItemCategory,
    TriggerTargetType,
    WindSound,
    Achievement,
    BuildEntityAchievement,
    #[strum(to_string = "combat-robot-count")]
    #[serde(rename = "combat-robot-count")]
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
    #[strum(to_string = "optimized-decorative")]
    #[serde(rename = "optimized-decorative")]
    Decorative,
    Arrow,
    ArtilleryFlare,
    ArtilleryProjectile,
    Beam,
    CharacterCorpse,
    Cliff,
    #[strum(to_string = "corpse")]
    #[serde(rename = "corpse")]
    CorpsePrototype,
    RailRemnants,
    DeconstructibleTileProxy,
    EntityGhost,
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
    LogisticRobot,
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
    #[strum(to_string = "loader-1x1")]
    #[serde(rename = "loader-1x1")]
    Loader1x1,
    #[strum(to_string = "loader")]
    #[serde(rename = "loader")]
    Loader1x2,
    Splitter,
    TransportBelt,
    UndergroundBelt,
    Tree,
    #[strum(to_string = "turret")]
    #[serde(rename = "turret")]
    TurretPrototype,
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
    #[strum(to_string = "fire")]
    #[serde(rename = "fire")]
    FireFlame,
    #[strum(to_string = "stream")]
    #[serde(rename = "stream")]
    FluidStream,
    FlyingText,
    #[strum(to_string = "highlight-box")]
    #[serde(rename = "highlight-box")]
    HighlightBoxEntity,
    ItemEntity,
    ItemRequestProxy,
    ParticleSource,
    Projectile,
    #[strum(to_string = "resource")]
    #[serde(rename = "resource")]
    ResourceEntity,
    RocketSiloRocket,
    RocketSiloRocketShadow,
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
    #[strum(to_string = "item")]
    #[serde(rename = "item")]
    ItemPrototype,
    #[strum(to_string = "ammo")]
    #[serde(rename = "ammo")]
    AmmoItem,
    Capsule,
    Gun,
    ItemWithEntityData,
    ItemWithLabel,
    ItemWithInventory,
    BlueprintBook,
    ItemWithTags,
    #[strum(to_string = "selection-tool")]
    #[serde(rename = "selection_tool")]
    SelectionToolPrototype,
    #[strum(to_string = "blueprint")]
    #[serde(rename = "blueprint")]
    BlueprintItem,
    CopyPasteTool,
    DeconstructionItem,
    UpgradeItem,
    Module,
    RailPlanner,
    SpidertronRemote,
    Tool,
    Armor,
    RepairTool,
    ItemGroup,
    #[strum(to_string = "item-subgroup")]
    #[serde(rename = "item-subgroup")]
    ItemSubGroup,
    ModuleCategory,
    #[strum(to_string = "noise-expression")]
    #[serde(rename = "noise-expression")]
    NamedNoiseExpression,
    NoiseLayer,
    #[strum(to_string = "optimized-particle")]
    #[serde(rename = "optimized-particle")]
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
    UtilityConstants,
    UtilitySounds,
    UtilitySprites,
    VirtualSignal,
    // Mod setting prototypes
    BoolSetting,
    IntSetting,
    DoubleSetting,
    StringSetting
}
