pub mod concepts;
pub mod data_structs;
pub mod prototypes;

use mlua::{Lua, prelude::LuaResult, Integer, Table};
use prototypes::*;

// TODO: add more features
/// Unfinished wrapper around [mlua::Lua::new] that sets some global variables
/// Adds `table_size` global function into environment
pub fn new_lua_instance() -> LuaResult<Lua> {
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
