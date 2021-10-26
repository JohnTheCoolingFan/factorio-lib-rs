//! This module contains abstract prototypes used for [PrototypeReference].
//! Instead of Data stage, these structs correspond to the scripting stage of Factorio modding.
//! Not all properties are implemented

use crate::prototypes::{Prototype, PrototypeBase, PrototypeBaseSpec};
use crate::concepts::LocalisedString;

// TODO: Finish other abstract prototypes
// TODO: impl DataTableAccessable
// TODO: impl From<>

/// <https://lua-api.factorio.com/next/LuaAchievementPrototype.html>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct AbstractAchievement {
    name: String,
    prototype_base: PrototypeBaseSpec,
    allowed_without_fight: bool,
    hidden: bool,
    valid: bool,
}
