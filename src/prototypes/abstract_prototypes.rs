//! This module contains abstract prototypes used for [PrototypeReference](crate::PrototypeReference).
//! Minimal amount of properties is implemented.
//! Prototypes will be added on demand

use crate::prototypes::Prototype;

// TODO: Finish other abstract prototypes
// TODO: impl DataTableAccessable
// TODO (not important): type property

/// <https://wiki.factorio.com/PrototypeBase>
#[derive(Debug, Clone, Prototype)]
pub struct AbstractPrototypeBase {
    pub name: String,
}

/// <https://wiki.factorio.com/Prototype/Achievement>
#[derive(Debug, Clone, Prototype)]
pub struct AbstractAchievement {
    pub name: String
}

/// <https://wiki.factorio.com/Prototype/Entity>
#[derive(Debug, Clone, Prototype)]
pub struct AbstractEntity {
    pub name: String
}
