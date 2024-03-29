use std::{collections::HashMap, fmt, io::Read, ops::Index, rc::Rc};

use ini::Ini;
use thiserror::Error;

// Unfinished
/// Handles the locale file parsing, aggregation and unified access
#[derive(Debug, Default)]
pub struct LocaleHandler {
    entries: HashMap<String, String>,
}

impl Index<String> for LocaleHandler {
    type Output = String;

    fn index(&self, key: String) -> &Self::Output {
        self.entries.get(&key).unwrap() // Improve to not use unwrap
    }
}

impl LocaleHandler {
    pub fn append_from_reader<R: Read>(&mut self, reader: &mut R) -> Result<(), ini::Error> {
        let ini = Ini::read_from_noescape(reader)?;
        if !ini.is_empty() {
            for (section, property) in ini.iter() {
                if let Some(section) = section {
                    for (key, value) in property.iter() {
                        self.entries
                            .insert(format!("{section}.{key}"), value.to_string());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_by_key(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }
}

// Factorio concepts
// https://lua-api.factorio.com/latest/Concepts.html

/// The input type for functions that accept LocalisedString
/// Also used in LocalisedString itself
#[derive(Debug, Clone)]
pub enum LocalisedStringEntry {
    String(String),             // Just a string
    LocString(LocalisedString), // Table / LocalisedString
}

impl fmt::Display for LocalisedStringEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalisedStringEntry::String(s) => write!(f, "{s}"),
            LocalisedStringEntry::LocString(ls) => write!(f, "{ls}"),
        }
    }
}

#[cfg(feature = "lua")]
impl<'lua> mlua::FromLua<'lua> for LocalisedStringEntry {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(t) => Ok(LocalisedStringEntry::LocString(
                lua.unpack::<LocalisedString>(lua.pack(t)?)?,
            )),
            _ => Ok(LocalisedStringEntry::String(lua.unpack::<String>(value)?)),
        }
    }
}

/// LocalisedString
///
/// This is unfinished but working implementation.
/// Referencing other locale entries is not implemented
#[derive(Debug, Clone)]
pub struct LocalisedString {
    key: String,
    parameters: Vec<LocalisedStringEntry>, // All elements after first element
    locale_handler: Rc<Option<LocaleHandler>>,
}

impl fmt::Display for LocalisedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // To print actual localised strings, access to locale info is needed, located in .cfg (ini) files
        if self.key.is_empty() {
            // Concatenate all parameters
            for parameter in self.parameters.clone() {
                match parameter {
                    LocalisedStringEntry::String(v) => write!(f, "{v}")?, // It's just a string or converted to string, simply write it
                    LocalisedStringEntry::LocString(mut v) => {
                        v.set_handler(&self.locale_handler); // Pass the locale_handler reference to inner LocalisedString
                        write!(f, "{v}")?
                    }
                }
            }
        } else {
            // Resolve the key and use parameters for substitution
            let locale_string = {
                match &*self.locale_handler {
                    // Dereference, then reference?..
                    Some(lh) => lh.get_by_key(&self.key),
                    _ => None,
                }
            };

            if let Some(s) = locale_string {
                // Key is found
                let mut temp_str = s.to_string();
                for i in 1..self.parameters.len() {
                    // Search for substituion spots
                    if temp_str.as_str().contains(&format!("__{i}__")) {
                        temp_str = temp_str.as_str().replace(
                            format!("__{i}__").as_str(),
                            match &self.parameters[i - 1] {
                                LocalisedStringEntry::String(st) => st,
                                _ => return Err(fmt::Error),
                            },
                        );
                    }
                }
                write!(f, "{temp_str}")?
            } else {
                write!(f, "Unknown key: \"{}\"", self.key)?
            }
        }
        Ok(())
    }
}

#[cfg(feature = "lua")]
impl<'lua> mlua::FromLua<'lua> for LocalisedString {
    fn from_lua(value: mlua::Value<'lua>, _: &'lua mlua::Lua) -> LuaResult<Self> {
        match &value {
            mlua::Value::Table(t) => {
                let mut seq_t = t.clone().sequence_values::<LocalisedStringEntry>();
                let key = match seq_t.next() {
                    Some(k) => match k? {
                        LocalisedStringEntry::String(s) => s,
                        _ => {
                            return Err(mlua::Error::FromLuaConversionError {
                                from: value.type_name(),
                                to: "LocalisedString",
                                message: Some(String::from(
                                    "LocalisedString key entry can only be string",
                                )),
                            })
                        }
                    },
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: value.type_name(),
                            to: "LocalisedString",
                            message: Some(String::from("Failed to retrieve LocalisedString key")),
                        })
                    }
                };
                let parameters =
                    seq_t.collect::<Result<Vec<LocalisedStringEntry>, mlua::Error>>()?;
                Ok(Self {
                    key,
                    parameters,
                    locale_handler: Rc::new(None),
                })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LocalisedString",
                message: Some(String::from("LocalisedString can be built only from table")),
            }),
        }
    }
}

impl LocalisedString {
    pub fn set_handler(&mut self, locale_handler: &Rc<Option<LocaleHandler>>) {
        self.locale_handler = Rc::clone(locale_handler);
    }
}

// Error enum for concepts
#[derive(Debug, Error)]
pub enum ConceptsErr {}
