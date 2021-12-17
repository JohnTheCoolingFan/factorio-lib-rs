//! This module contains abstract prototypes used for [PrototypeReference](crate::PrototypeReference).
//! Not all properties are implemented

use crate::prototypes::Prototype;

// TODO: Finish other abstract prototypes
// TODO: impl DataTableAccessable

/// <https://wiki.factorio.com/PrototypeBase>
#[derive(Debug, Clone, Prototype)]
pub struct AbstractPrototypeBase {
    name: String,
    // TODO (not important): type property
}
