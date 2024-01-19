use crate::prototypes::{DataTable, DataTableAccessable, Prototype, PrototypeType, PrototypesErr};

#[derive(Debug, Clone)]
pub struct AbstractEntity {
    pub name: String,
    pub prototype_type: PrototypeType,
}

impl Prototype for AbstractEntity {
    fn name(&self) -> &String {
        &self.name
    }

    fn prototype_type(&self) -> PrototypeType {
        self.prototype_type
    }
}

// Executes get() on every specified prototype category, returning the first match. Errors if no
// prototypes with this name found  in corresponding category.
macro_rules! abstract_prototype_get {(
    $data_table:ident, $name:ident,
    $( $ptype:ident $(,)? )*
    ) => (
    $( if let Some(prototype) = $data_table.$ptype.get($name) {
        return Ok(Self{name: $name.into(), prototype_type: prototype.prototype_type()})
    } )*
    return Err(PrototypesErr::PrototypeNotFound($name.into()))
)}

impl DataTableAccessable for AbstractEntity {
    fn find<'a>(_data_table: &'a DataTable, _name: &str) -> Result<&'a Self, PrototypesErr> {
        Err(PrototypesErr::AbstractFind)
    }

    fn find_cloned(data_table: &DataTable, name: &str) -> Result<Self, PrototypesErr> {
        // TODO: add other types
        abstract_prototype_get!(data_table, name, arrow, artillery_flare);
    }

    fn extend(self, _data_table: &mut DataTable) -> Result<(), PrototypesErr> {
        Err(PrototypesErr::AbstractExtend)
    }
}
