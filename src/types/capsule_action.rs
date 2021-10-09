use crate::types::{AttackParameters};

/// <https://wiki.factorio.com/Types/CapsuleAction>
#[derive(Debug)]
pub enum CapsuleAction {
    Throw(ThrowCapsuleAction),
    EquipmentRemote(ActivateEquipmentCapsuleAction),
    UseOnSelf(UseOnSelfCapsuleAction),
    ArtilleryRemote(ArtilleryRemoteCapsuleAction),
    DestroyCliffs(DestroyCliffsCapsuleAction)
}

/// <https://wiki.factorio.com/Types/ThrowCapsuleAction>
#[derive(Debug)]
pub struct ThrowCapsuleAction {
    attack_parameters: AttackParameters,
    uses_stack: bool // Default: true
}

/// <https://wiki.factorio.com/Types/ActivateEquipmentCapsuleAction>
#[derive(Debug)]
pub struct ActivateEquipmentCapsuleAction {
    equipment: String, // Name of Equipment prototype
}

/// <https://wiki.factorio.com/Types/UseOnSelfCapsuleAction>
#[derive(Debug)]
pub struct UseOnSelfCapsuleAction {
    attack_parameters: AttackParameters,
    uses_stack: bool // Default: true
}

/// <https://wiki.factorio.com/Types/ArtilleryRemoteCapsuleAction>
#[derive(Debug)]
pub struct ArtilleryRemoteCapsuleAction {
    flare: String, // Name of ArtilleryFlare prototype
    play_sound_on_failure: bool // Default: true
}

/// <https://wiki.factorio.com/Types/DestroyCliffsCapsuleAction>
#[derive(Debug)]
pub struct DestroyCliffsCapsuleAction {
    attack_parameters: AttackParameters,
    radius: f32,
    timeout: u32, // Default: 3600
    play_sound_on_failure: bool, // Default: true
    uses_stack: bool // Default: true
}
