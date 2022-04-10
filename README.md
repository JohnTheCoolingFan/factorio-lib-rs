`factorio-lib-rs` is a rust library for various Factorio concepts and data struct (prototypes, for example) to be interacted with using Rust.

This library uses [custom fork](https://github.com/JohnTheCoolingFan/factorio-mlua) of [mlua](https://github.com/khvzak/mlua) (There is currently a [PR](https://github.com/khvzak/mlua/pull/141) to merge it into mlua) that includes lua runtime that is used in the game itself, source provided by the developers. It is used to load the lua code of mods and then parse the data into prototype structs that can then be used from Rust code.

This library is still WIP.

Development status: Implementing PrototypeFromLua, making derive macro.
