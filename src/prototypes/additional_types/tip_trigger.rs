use strum_macros::{EnumString, AsRefStr};

/// <https://wiki.factorio.com/Types/TipTrigger>
#[derive(Debug, Clone)]
pub enum TipTrigger {
    Or(OrTipTrigger),
    And(AndTipTrigger),
    Sequence(SequenceTipTrigger),
    DependenciesMet(DependenciesMetTipTrigger),
    TimeElapsed(TimeElapsedTipTrigger),
    Research(ResearchTechnologyTipTrigger),
    UnlockRecipe(UnlockRecipeTipTrigger),
    CraftItem(CraftItemTipTrigger),
    BuildEntity(BuildEntityTipTrigger),
    ManualTransfer(ManualTransferTipTrigger),
    StackTransfer(StackTransferTipTrigger),
    EntityTransfer(EntityTransferTipTrigger),
    SetRecipe(SetRecipeTipTrigger),
    LimitChest(LimitChestTipTrigger),
    UsePipette(UsePipetteTipTrigger),
    SetLogisticRequest(SetLogisticRequestTipTrigger),
    UseConfirm(UseConfirmTipTrigger),
    LowPower(LowPowerTipTrigger),
    PasteEntitySettings(PasteEntitySettingsTipTrigger),
    FastReplace(FastReplaceTipTrigger),
    GroupAttack(GroupAttackTipTrigger),
    FastBeltBend(FastBeltBendTipTrigger),
    BeltTraverse(BeltTraverseTipTrigger),
    PlaceEquipment(PlaceEquipmentTipTrigger),
    ClearCursor(ClearCursorTipTrigger),
    ShiftBuild(ShiftBuildTipTrigger),
    GateOverRailBuild(GateOverRailBuildTipTrigger),
    ManualWireDrag(ManualWireDragTipTrigger),
}

/// <https://wiki.factorio.com/Types/TipTrigger#OrTipTrigger>
#[derive(Debug, Clone)]
pub struct OrTipTrigger {
    triggers: Vec<TipTrigger>
}

/// <https://wiki.factorio.com/Types/TipTrigger#AndTipTrigger>
#[derive(Debug, Clone)]
pub struct AndTipTrigger {
    triggers: Vec<TipTrigger>
}

/// <https://wiki.factorio.com/Types/TipTrigger#SequenceTipTrigger>
#[derive(Debug, Clone)]
pub struct SequenceTipTrigger {
    triggers: Vec<TipTrigger>
}

/// <https://wiki.factorio.com/Types/TipTrigger#DependenciesMetTipTrigger>
#[derive(Debug, Clone)]
pub struct DependenciesMetTipTrigger;

/// <https://wiki.factorio.com/Types/TipTrigger#TimeElapsedTipTrigger>
#[derive(Debug, Clone)]
pub struct TimeElapsedTipTrigger {
    ticks: u32
}

/// <https://wiki.factorio.com/Types/TipTrigger#ResearchTechnologyTipTrigger>
#[derive(Debug, Clone)]
pub struct ResearchTechnologyTipTrigger {
    technology: String // Name of technology
}

/// <https://wiki.factorio.com/Types/TipTrigger#UnlockedRecipeTipTrigger>
/// <https://wiki.factorio.com/Types/TipTrigger#UnlockRecipeTipTrigger>
#[derive(Debug, Clone)]
pub struct UnlockRecipeTipTrigger {
    recipe: String // Name of recipe
}

/// <https://wiki.factorio.com/Types/TipTrigger#CraftItemTipTrigger>
#[derive(Debug, Clone)]
pub struct CraftItemTipTrigger {
    count: u32, // Default: 0
    item: Option<String>, // Name of Item
    consecutive: bool, // Default: false
    event_type: CraftItemTipTriggerEventType
}

/// <https://wiki.factorio.com/Types/TipTrigger#event_type>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum CraftItemTipTriggerEventType {
    CraftingOfSingleItemOrdered,
    CraftingOfMultipleItemsOrdered,
    CraftingFinished,
}

/// <https://wiki.factorio.com/Types/TipTrigger#BuildEntityTipTrigger>
#[derive(Debug, Clone)]
pub struct BuildEntityTipTrigger {
    count: u32, // Default: 1
    entity: Option<String>, // Name of Entity
    match_type_only: bool, // Default: false
    build_by_dragging: bool, // Default: false
}

/// <https://wiki.factorio.com/Types/TipTrigger#ManualTransferTipTrigger>
#[derive(Debug, Clone)]
pub struct ManualTransferTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#StackTransferTipTrigger>
#[derive(Debug, Clone)]
pub struct StackTransferTipTrigger {
    count: u32, // Default: 0
    transfer: Option<StackTransferType>
}

/// <https://wiki.factorio.com/Types/TipTrigger#transfer>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum StackTransferType {
    Stack,
    Inventory,
    WholeInventory,
}

/// <https://wiki.factorio.com/Types/TipTrigger#EntityTransferTipTrigger>
#[derive(Debug, Clone)]
pub struct EntityTransferTipTrigger {
    count: u32, // Default: 0
    transfer: Option<EntityTransferType>
}

/// <https://wiki.factorio.com/Types/TipTrigger#transfer_2>
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum EntityTransferType {
    In,
    Out,
}

/// <https://wiki.factorio.com/Types/TipTrigger#SetRecipeTipTrigger>
#[derive(Debug, Clone)]
pub struct SetRecipeTipTrigger {
    count: u32, // Default: 0
    recipe: Option<String>, // Name of Recipe
    machine: Option<String>, // Name of Entity
    consecutive: bool, // default: false
    uses_fluid: Option<bool>, // Default: None.
}

/// <https://wiki.factorio.com/Types/TipTrigger#LimitChestTipTrigger>
#[derive(Debug, Clone)]
pub struct LimitChestTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#UsePipetteTipTrigger>
#[derive(Debug, Clone)]
pub struct UsePipetteTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#SetLogisticRequestTipTrigger>
#[derive(Debug, Clone)]
pub struct SetLogisticRequestTipTrigger {
    count: u32, // Default: 0
    logistic_chest_only: bool, // default: false
}

/// <https://wiki.factorio.com/Types/TipTrigger#UseConfirmTipTrigger>
#[derive(Debug, Clone)]
pub struct UseConfirmTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#LowPowerTipTrigger>
#[derive(Debug, Clone)]
pub struct LowPowerTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#PasteEntitySettingsTipTrigger>
#[derive(Debug, Clone)]
pub struct PasteEntitySettingsTipTrigger {
    count: u32, // Default: 0
    source: Option<String>, // Name of Entity
    target: Option<String>, // Name of Entity
    match_type_only: bool, // Default: false
}

/// <https://wiki.factorio.com/Types/TipTrigger#FastReplaceTipTrigger>
#[derive(Debug, Clone)]
pub struct FastReplaceTipTrigger {
    count: u32, // Default: 0
    source: Option<String>, // Name of Entity
    target: Option<String>, // Name of Entity
    match_type_only: bool, // Default: ffalse
}

/// <https://wiki.factorio.com/Types/TipTrigger#GroupAttackTipTrigger>
#[derive(Debug, Clone)]
pub struct GroupAttackTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#FastBeltBendTipTrigger>
#[derive(Debug, Clone)]
pub struct FastBeltBendTipTrigger {
    count: u32, // Default: 1
}

/// <https://wiki.factorio.com/Types/TipTrigger#BeltTraverseTipTrigger>
#[derive(Debug, Clone)]
pub struct BeltTraverseTipTrigger {
    count: u32, // Default: 1
}

/// <https://wiki.factorio.com/Types/TipTrigger#PlaceEquipmentTipTrigger>
#[derive(Debug, Clone)]
pub struct PlaceEquipmentTipTrigger {
    count: u32, // Default: 1
    equipment: Option<String>, // Name of Equipment prototype
}

/// <https://wiki.factorio.com/Types/TipTrigger#ClearCursorTipTrigger>
#[derive(Debug, Clone)]
pub struct ClearCursorTipTrigger {
    count: u32, // Default: 1
}

/// <https://wiki.factorio.com/Types/TipTrigger#ShiftBuildTipTrigger>
#[derive(Debug, Clone)]
pub struct ShiftBuildTipTrigger {
    count: u32, // Default: 1
}

/// <https://wiki.factorio.com/Types/TipTrigger#GateOverRailBuildTipTrigger>
#[derive(Debug, Clone)]
pub struct GateOverRailBuildTipTrigger {
    count: u32, // Default: 1
}

/// <https://wiki.factorio.com/Types/TipTrigger#ManualWireDragTipTrigger>
#[derive(Debug, Clone)]
pub struct ManualWireDragTipTrigger {
    count: u32, // Default: 1
}
