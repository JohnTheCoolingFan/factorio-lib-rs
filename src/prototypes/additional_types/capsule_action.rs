use super::AttackParameters;
use crate::util::defaults::*;
use serde::Deserialize;

/// <https://wiki.factorio.com/Types/CapsuleAction>
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CapsuleAction {
    Throw(ThrowCapsuleAction),
    EquipmentRemote(ActivateEquipmentCapsuleAction),
    UseOnSelf(UseOnSelfCapsuleAction),
    ArtilleryRemote(ArtilleryRemoteCapsuleAction),
    DestroyCliffs(DestroyCliffsCapsuleAction),
}

/// <https://wiki.factorio.com/Types/ThrowCapsuleAction>
#[derive(Debug, Clone, Deserialize)]
pub struct ThrowCapsuleAction {
    pub attack_parameters: AttackParameters,
    #[serde(default = "default_bool::<true>")]
    pub uses_stack: bool, // Default: true
}

/// <https://wiki.factorio.com/Types/ActivateEquipmentCapsuleAction>
#[derive(Debug, Clone, Deserialize)]
pub struct ActivateEquipmentCapsuleAction {
    pub equipment: String, // Name of Equipment prototype
}

/// <https://wiki.factorio.com/Types/UseOnSelfCapsuleAction>
#[derive(Debug, Clone, Deserialize)]
pub struct UseOnSelfCapsuleAction {
    pub attack_parameters: AttackParameters,
    #[serde(default = "default_bool::<true>")]
    pub uses_stack: bool, // Default: true
}

/// <https://wiki.factorio.com/Types/ArtilleryRemoteCapsuleAction>
#[derive(Debug, Clone, Deserialize)]
pub struct ArtilleryRemoteCapsuleAction {
    pub flare: String, // Name of ArtilleryFlare prototype
    #[serde(default = "default_bool::<true>")]
    pub play_sound_on_failure: bool, // Default: true
}

/// <https://wiki.factorio.com/Types/DestroyCliffsCapsuleAction>
#[derive(Debug, Clone, Deserialize)]
pub struct DestroyCliffsCapsuleAction {
    pub attack_parameters: AttackParameters,
    pub radius: f32,
    #[serde(default = "default_u32::<3600>")]
    pub timeout: u32, // Default: 3600
    #[serde(default = "default_bool::<true>")]
    pub play_sound_on_failure: bool, // Default: true
    #[serde(default = "default_bool::<true>")]
    pub uses_stack: bool, // Default: true
}
