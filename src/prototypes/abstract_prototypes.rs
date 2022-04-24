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
        data_table.arrow.get(name).map(|p| Self{name: p.name.clone(), prototype_type: p.prototype_type()})
            .or_else(|| data_table.artillery_flare.get(name).map(|p| Self{name: p.name.clone(), prototype_type: p.prototype_type()}))
            .ok_or_else(|| PrototypesErr::PrototypeNotFound(name.into()))
    }

    fn extend(self, _data_table: &mut DataTable) -> Result<(), PrototypesErr> {
        Err(PrototypesErr::AbstractExtend)
    }
}
