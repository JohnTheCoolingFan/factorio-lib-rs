//! Factorio lib
//!
//! This library provides an interface to Factorio data types, mod prototypes and Web API.
//! Check the individual module documentation for details
//!
//! Modules are gated behind features with corresponding names, which are all enabled by default.

#[macro_use]
extern crate macro_rules_attribute;

// TODO: transcode lua data to json

// If concepts are disabled, LocalisedString can't be used...
#[cfg(feature = "concepts")]
pub mod concepts;
#[cfg(feature = "data_structs")]
pub mod data_structs;
#[cfg(feature = "prototypes")]
pub mod prototypes;
#[cfg(feature = "webapi")]
pub mod webapi;

#[cfg(feature = "lua")]
use mlua::{prelude::*, Integer, Lua, Table};

// TODO: add more features
/// Unfinished wrapper around [mlua::Lua::new] that sets some global variables
///
/// Adds `table_size` global function into environment
#[cfg(feature = "lua")]
pub fn new_factorio_lua() -> LuaResult<Lua> {
    let lua = Lua::new();

    {
        let globals = lua.globals();

        fn tablesize(_lua: &Lua, table: Table) -> LuaResult<Integer> {
            Ok(table.table_size(true))
        }

        let tablesize_luaf = lua.create_function(tablesize)?;

        globals.set("table_size", tablesize_luaf)?;
    }

    Ok(lua)
}
