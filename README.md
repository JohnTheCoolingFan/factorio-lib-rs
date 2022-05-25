`factorio-lib-rs` is a rust library for various Factorio concepts and data struct (prototypes, for example) to be interacted with using Rust. Check the docs (`cargo doc --open`) for an overview of what this library is capable of.

This library uses [custom fork](https://github.com/JohnTheCoolingFan/factorio-mlua) of [mlua](https://github.com/khvzak/mlua) (There is currently a [PR](https://github.com/khvzak/mlua/pull/141) to merge it into mlua) that includes lua runtime that is used in the game itself, source provided by the developers. It is used to load the lua code of mods and then parse the data into prototype structs that can then be used from Rust code.

This library consists of a number of modules:
- `prototypes`: reimplementation of factorio mod loading
- `webapi`: simplified usage of factorio's web api
- `data_structs`: data structures related to factorio, like info.json
- `concepts`: Factorio runtime concepts, like LocalisedString

These features are available through corresponding features, all enabled by default

This library is still WIP.

Development status: Implementing PrototypeFromLua, making derive macro.
