//! This module contains abstract prototypes used for [PrototypeReference](crate::PrototypeReference).
//! Not all properties are implemented

use crate::prototypes::{Prototype, PrototypeBase, PrototypeBaseSpec};
use crate::concepts::LocalisedString;

// TODO: Finish other abstract prototypes
// TODO: impl DataTableAccessable

/// <https://lua-api.factorio.com/next/LuaAchievementPrototype.html>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct AbstractAchievementPrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
    allowed_without_fight: bool,
    hidden: bool,
    valid: bool,
}

/// <https://lua-api.factorio.com/next/LuaEntityPrototype.html>
#[derive(Debug, Prototype, PrototypeBase)]
pub struct AbstractEntityPrototype {
    name: String,
    prototype_base: PrototypeBaseSpec,
}
