use factorio_lib_rs_derive::abstract_prototype_get;
use crate::prototypes::{PrototypeType, DataTable, DataTableAccessable, PrototypesErr, Prototype};

#[derive(Debug, Clone)]
pub struct AbstractEntity {
    pub name: String,
    pub prototype_type: PrototypeType
}

impl Prototype for AbstractEntity {
    fn name(&self) -> &String {
        &self.name
    }

    fn prototype_type(&self) -> PrototypeType {
        self.prototype_type
    }
}

impl DataTableAccessable for AbstractEntity {
    fn find<'a>(_data_table: &'a DataTable, _name: &str) -> Result<&'a Self, PrototypesErr> {
        Err(PrototypesErr::AbstractFind)
    }

    fn find_cloned(data_table: &DataTable, name: &str) -> Result<Self, PrototypesErr> {
        // TODO 
        abstract_prototype_get!(arrow, artillery_flare)
    }
}
