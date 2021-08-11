pub mod concepts;
pub mod mod_structs;
pub mod prototypes;
pub mod prototype_type;
pub mod types;

#[cfg(test)]
use crate::prototypes::PrototypeGeneral;

// I guess this is a good place for todos

// TODO: FromLua for prototypes
// TODO: rest of the prototypes

#[test]
fn check_prototypes_size() {
    println!("Size of PrototypeGeneral enum: {} bytes", std::mem::size_of::<PrototypeGeneral>())
}
