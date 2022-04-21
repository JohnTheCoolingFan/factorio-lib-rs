use super::AttackParameters;
use strum::{EnumDiscriminants, EnumString};
use mlua::{prelude::LuaResult, Value, Lua};
use crate::prototypes::{DataTable, PrototypeFromLua, GetPrototype};

/// <https://wiki.factorio.com/Types/CapsuleAction>
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString), strum(serialize_all = "kebab-case"))]
pub enum CapsuleAction {
    Throw(ThrowCapsuleAction),
    EquipmentRemote(ActivateEquipmentCapsuleAction),
    UseOnSelf(UseOnSelfCapsuleAction),
    ArtilleryRemote(ArtilleryRemoteCapsuleAction),
    DestroyCliffs(DestroyCliffsCapsuleAction)
}

impl<'lua> PrototypeFromLua<'lua> for CapsuleAction {
    fn prototype_from_lua(value: Value<'lua>, lua: &'lua Lua, data_table: &mut DataTable) -> LuaResult<Self> {
        if let Value::Table(table) = &value {
            Ok(match table.get::<_, String>("type")?.parse().map_err(mlua::Error::external)? {
                CapsuleActionDiscriminants::Throw => CapsuleAction::Throw(ThrowCapsuleAction::prototype_from_lua(value, lua, data_table)?),
                CapsuleActionDiscriminants::EquipmentRemote => CapsuleAction::EquipmentRemote(ActivateEquipmentCapsuleAction::prototype_from_lua(value, lua, data_table)?),
                CapsuleActionDiscriminants::UseOnSelf => CapsuleAction::UseOnSelf(UseOnSelfCapsuleAction::prototype_from_lua(value, lua, data_table)?),
                CapsuleActionDiscriminants::ArtilleryRemote => CapsuleAction::ArtilleryRemote(ArtilleryRemoteCapsuleAction::prototype_from_lua(value, lua, data_table)?),
                CapsuleActionDiscriminants::DestroyCliffs => CapsuleAction::DestroyCliffs(DestroyCliffsCapsuleAction::prototype_from_lua(value, lua, data_table)?),
            })
        } else {
            Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "CapsuleAction", message: Some("expected table".into()) })
        }
    }
}

/// <https://wiki.factorio.com/Types/ThrowCapsuleAction>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ThrowCapsuleAction {
    pub attack_parameters: AttackParameters,
    #[default(true)]
    pub uses_stack: bool // Default: true
}

/// <https://wiki.factorio.com/Types/ActivateEquipmentCapsuleAction>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ActivateEquipmentCapsuleAction {
    pub equipment: String, // Name of Equipment prototype
}

/// <https://wiki.factorio.com/Types/UseOnSelfCapsuleAction>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct UseOnSelfCapsuleAction {
    pub attack_parameters: AttackParameters,
    #[default(true)]
    pub uses_stack: bool // Default: true
}

/// <https://wiki.factorio.com/Types/ArtilleryRemoteCapsuleAction>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct ArtilleryRemoteCapsuleAction {
    pub flare: String, // Name of ArtilleryFlare prototype
    #[default(true)]
    pub play_sound_on_failure: bool // Default: true
}

/// <https://wiki.factorio.com/Types/DestroyCliffsCapsuleAction>
#[derive(Debug, Clone, PrototypeFromLua)]
pub struct DestroyCliffsCapsuleAction {
    pub attack_parameters: AttackParameters,
    pub radius: f32,
    #[default(3600_u32)]
    pub timeout: u32, // Default: 3600
    #[default(true)]
    pub play_sound_on_failure: bool, // Default: true
    #[default(true)]
    pub uses_stack: bool // Default: true
}
