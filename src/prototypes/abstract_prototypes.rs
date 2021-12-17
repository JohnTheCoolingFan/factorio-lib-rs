//! This module contains abstract prototypes used for [PrototypeReference](crate::PrototypeReference).
//! Not all properties are implemented

use crate::prototypes::Prototype;

// TODO: Finish other abstract prototypes
// TODO: impl DataTableAccessable
// TODO (not important): type property

/// <https://wiki.factorio.com/PrototypeBase>
#[derive(Debug, Clone, Prototype)]
pub struct AbstractPrototypeBase {
    name: String,
}

/// <https://wiki.factorio.com/Prototype/Achievement>
#[derive(Debug, Clone, Prototype)]
pub struct AbstractAchievement {
    name: String
}

/// <https://wiki.factorio.com/Prototype/Entity>
#[derive(Debug, Clone, Prototype)]
pub struct AbstractEntity {
    name: String
}
