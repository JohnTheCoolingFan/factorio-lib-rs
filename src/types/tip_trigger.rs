use strum_macros::{EnumString, AsRefStr};

/// <https://wiki.factorio.com/Types/TipTrigger>
#[derive(Debug)]
pub enum TipTrigger {
    Or(OrTipTrigger),
    And(AndTipTrigger),
    Sequence(SequenceTipTrigger),
    DependenciesMet(DependenciesMetTipTrigger),
    TimeElapsed(TimeElapsedTipTrigger),
    Research(ResearchTechnologyTipTrigger),
    UnlockedRecipe(UnlockedRecipeTipTrigger),
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
    GroupAttack(GroupAttackTipTrigger)
}

/// <https://wiki.factorio.com/Types/TipTrigger#OrTipTrigger>
#[derive(Debug)]
pub struct OrTipTrigger {
    triggers: Vec<TipTrigger>
}

/// <https://wiki.factorio.com/Types/TipTrigger#AndTipTrigger>
#[derive(Debug)]
pub struct AndTipTrigger {
    triggers: Vec<TipTrigger>
}

/// <https://wiki.factorio.com/Types/TipTrigger#SequenceTipTrigger>
#[derive(Debug)]
pub struct SequenceTipTrigger {
    triggers: Vec<TipTrigger>
}

/// <https://wiki.factorio.com/Types/TipTrigger#DependenciesMetTipTrigger>
#[derive(Debug)]
pub struct DependenciesMetTipTrigger;

/// <https://wiki.factorio.com/Types/TipTrigger#TimeElapsedTipTrigger>
#[derive(Debug)]
pub struct TimeElapsedTipTrigger {
    ticks: u32
}

/// <https://wiki.factorio.com/Types/TipTrigger#ResearchTechnologyTipTrigger>
#[derive(Debug)]
pub struct ResearchTechnologyTipTrigger {
    technology: String // Name of technology
}

/// <https://wiki.factorio.com/Types/TipTrigger#UnlockedRecipeTipTrigger>
#[derive(Debug)]
pub struct UnlockedRecipeTipTrigger {
    recipe: String // Name of recipe
}

/// <https://wiki.factorio.com/Types/TipTrigger#CraftItemTipTrigger>
#[derive(Debug)]
pub struct CraftItemTipTrigger {
    count: u32, // Default: 0
    item: Option<String>, // Name of Item
    consecutive: bool, // Default: false
    event_type: CraftItemTipTriggerEventType
}

/// <https://wiki.factorio.com/Types/TipTrigger#event_type>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum CraftItemTipTriggerEventType {
    CraftingOfSingleItemOrdered,
    CraftingOfMultipleItemsOrdered,
    CraftingFinished,
}

/// <https://wiki.factorio.com/Types/TipTrigger#BuildEntityTipTrigger>
#[derive(Debug)]
pub struct BuildEntityTipTrigger {
    count: u32, // Default: 1
    entity: Option<String>, // Name of Entity
    match_type_only: bool, // Default: false
    build_by_dragging: bool, // Default: false
}

/// <https://wiki.factorio.com/Types/TipTrigger#ManualTransferTipTrigger>
#[derive(Debug)]
pub struct ManualTransferTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#StackTransferTipTrigger>
#[derive(Debug)]
pub struct StackTransferTipTrigger {
    count: u32, // Default: 0
    transfer: Option<StackTransferType>
}

/// <https://wiki.factorio.com/Types/TipTrigger#transfer>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum StackTransferType {
    Stack,
    Inventory,
    WholeInventory,
}

/// <https://wiki.factorio.com/Types/TipTrigger#EntityTransferTipTrigger>
#[derive(Debug)]
pub struct EntityTransferTipTrigger {
    count: u32, // Default: 0
    transfer: Option<EntityTransferType>
}

/// <https://wiki.factorio.com/Types/TipTrigger#transfer_2>
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum EntityTransferType {
    In,
    Out,
}

/// <https://wiki.factorio.com/Types/TipTrigger#SetRecipeTipTrigger>
#[derive(Debug)]
pub struct SetRecipeTipTrigger {
    count: u32, // Default: 0
    recipe: Option<String>, // Name of Recipe
    machine: Option<String>, // Name of Entity
    consecutive: bool, // default: false
    uses_fluid: Option<bool>, // Default: None.
}

/// <https://wiki.factorio.com/Types/TipTrigger#LimitChestTipTrigger>
#[derive(Debug)]
pub struct LimitChestTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#UsePipetteTipTrigger>
#[derive(Debug)]
pub struct UsePipetteTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#SetLogisticRequestTipTrigger>
#[derive(Debug)]
pub struct SetLogisticRequestTipTrigger {
    count: u32, // Default: 0
    logistic_chest_only: bool, // default: false
}

/// <https://wiki.factorio.com/Types/TipTrigger#UseConfirmTipTrigger>
#[derive(Debug)]
pub struct UseConfirmTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#LowPowerTipTrigger>
#[derive(Debug)]
pub struct LowPowerTipTrigger {
    count: u32, // Default: 0
}

/// <https://wiki.factorio.com/Types/TipTrigger#PasteEntitySettingsTipTrigger>
#[derive(Debug)]
pub struct PasteEntitySettingsTipTrigger {
    count: u32, // Default: 0
    source: Option<String>, // Name of Entity
    target: Option<String>, // Name of Entity
    match_type_only: bool, // Default: false
}

/// <https://wiki.factorio.com/Types/TipTrigger#FastReplaceTipTrigger>
#[derive(Debug)]
pub struct FastReplaceTipTrigger {
    count: u32, // Default: 0
    source: Option<String>, // Name of Entity
    target: Option<String>, // Name of Entity
    match_type_only: bool, // Default: ffalse
}

/// <https://wiki.factorio.com/Types/TipTrigger#GroupAttackTipTrigger>
#[derive(Debug)]
pub struct GroupAttackTipTrigger {
    count: u32, // Default: 0
}
