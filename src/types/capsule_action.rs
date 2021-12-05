use crate::types::AttackParameters;

/// <https://wiki.factorio.com/Types/CapsuleAction>
#[derive(Debug, Clone)]
pub enum CapsuleAction {
    Throw(ThrowCapsuleAction),
    EquipmentRemote(ActivateEquipmentCapsuleAction),
    UseOnSelf(UseOnSelfCapsuleAction),
    ArtilleryRemote(ArtilleryRemoteCapsuleAction),
    DestroyCliffs(DestroyCliffsCapsuleAction)
}

/// <https://wiki.factorio.com/Types/ThrowCapsuleAction>
#[derive(Debug, Clone)]
pub struct ThrowCapsuleAction {
    attack_parameters: AttackParameters,
    uses_stack: bool // Default: true
}

/// <https://wiki.factorio.com/Types/ActivateEquipmentCapsuleAction>
#[derive(Debug, Clone)]
pub struct ActivateEquipmentCapsuleAction {
    equipment: String, // Name of Equipment prototype
}

/// <https://wiki.factorio.com/Types/UseOnSelfCapsuleAction>
#[derive(Debug, Clone)]
pub struct UseOnSelfCapsuleAction {
    attack_parameters: AttackParameters,
    uses_stack: bool // Default: true
}

/// <https://wiki.factorio.com/Types/ArtilleryRemoteCapsuleAction>
#[derive(Debug, Clone)]
pub struct ArtilleryRemoteCapsuleAction {
    flare: String, // Name of ArtilleryFlare prototype
    play_sound_on_failure: bool // Default: true
}

/// <https://wiki.factorio.com/Types/DestroyCliffsCapsuleAction>
#[derive(Debug, Clone)]
pub struct DestroyCliffsCapsuleAction {
    attack_parameters: AttackParameters,
    radius: f32,
    timeout: u32, // Default: 3600
    play_sound_on_failure: bool, // Default: true
    uses_stack: bool // Default: true
}
