// If concepts are disabled, LocalisedString can't be used...
#[cfg(feature = "concepts")]
pub mod concepts;
#[cfg(feature = "data-structs")]
pub mod data_structs;
#[cfg(feature = "prototypes")]
pub mod prototypes;
/// <https://wiki.factorio.com/Factorio_HTTP_API_usage_guidelines#APIs>
#[cfg(feature = "webapi")]
pub mod webapi;

#[cfg(feature = "lua")]
use mlua::{Lua, prelude::LuaResult, Integer, Table};

// TODO: add more features
/// Unfinished wrapper around [mlua::Lua::new] that sets some global variables
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
