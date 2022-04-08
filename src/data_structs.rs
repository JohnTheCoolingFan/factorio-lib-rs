use std::{path::PathBuf, str::FromStr, fmt::{self, Display, Formatter}};
use semver::Version;
use std::cmp::Ordering;
use lexical_sort::natural_only_alnum_cmp;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::Regex;
use semver::VersionReq;
use thiserror::Error;
use serde::{Serialize, Deserialize};

// Credit for the most part goes to raiguard's factorio_mod_manager
// https://github.com/raiguard/factorio_mod_manager

const MOD_DEPENDENCY_REGEX: &str = r"^(?:(?P<type>[!?~]|\(\?\)) *)?(?P<name>(?: *[a-zA-Z0-9_-]+)+(?: *$)?)(?: *(?P<version_req>[<>=]=?) *(?P<version>(?:\d+\.){1,2}\d+))?$";


#[derive(Debug, PartialEq, Deserialize)]
#[serde(try_from = "String")]
pub struct ModDependency {
    pub dep_type: ModDependencyType,
    pub name: String,
    pub version_req: Option<VersionReq>,
}

impl ModDependency {
    pub fn new(input: &str) -> Result<Self, ModDependencyErr> {
        static DEP_STRING_REGEX: OnceCell<Regex> = OnceCell::new();
        let captures = DEP_STRING_REGEX
            .get_or_init(|| {
                Regex::new(MOD_DEPENDENCY_REGEX).unwrap()
            })
            .captures(input)
            .ok_or_else(|| ModDependencyErr::InvalidDependencyString(input.into()))?;

        Ok(Self {
            dep_type: match captures.name("type").map(|mtch| mtch.as_str()) {
                None => ModDependencyType::Required,
                Some("!") => ModDependencyType::Incompatible,
                Some("~") => ModDependencyType::NoLoadOrder,
                Some("?") => ModDependencyType::Optional,
                Some("(?)") => ModDependencyType::OptionalHidden,
                Some(str) => return Err(ModDependencyErr::UnknownModifier(str.to_string())),
            },
            name: match captures.name("name") {
                Some(mtch) => mtch.as_str().to_string(),
                None => return Err(ModDependencyErr::NameIsUnparsable(input.into())),
            },
            version_req: match [captures.name("version_req"), captures.name("version")] {
                [Some(req_match), Some(version_match)] => {
                    let version_str = version_match.as_str();
                    #[allow(unstable_name_collisions)]
                    let sanitized = version_str
                        .split('.')
                        .map(|sub| {
                            Ok(sub
                                .parse::<usize>()
                                .map_err(|_| {
                                    ModDependencyErr::InvalidDependencyString(input.into())
                                })?
                                .to_string())
                        })
                        .intersperse(Ok(".".to_string()))
                        .collect::<Result<String, ModDependencyErr>>()?;
                    let mut version_req = String::new();
                    version_req.push_str(req_match.as_str());
                    version_req.push(' ');
                    version_req.push_str(&sanitized);

                    match VersionReq::parse(&version_req) {
                        Ok(version_req) => Some(version_req),
                        Err(_) => {
                            return Err(ModDependencyErr::InvalidVersionReq(
                                    req_match.as_str().to_string(),
                            ))
                        }
                    }
                }
                _ => None,
            }
        })
    }
}

impl TryFrom<String> for ModDependency {
    type Error = ModDependencyErr;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

#[derive(Debug, PartialEq)]
pub enum ModDependencyType {
    Incompatible,
    NoLoadOrder,
    Optional,
    OptionalHidden,
    Required,
}

pub type ModDependencyResult = Result<Vec<ModDependency>, ModDependencyErr>;

// enum for states of a mod (enabled or disabled)
#[derive(Debug)]
pub enum ModEnabledType {
    Disabled,
    Latest,           // Legacy from factorio_mod_manager, probably will be renamed
    Version(Version), // Legacy from factorio_mod_manager, probably will be removed
}

// Structs and enums for representing mod info related data

// Mod struct, containing mod name, version and enabled info
#[derive(Debug)]
pub struct Mod {
    pub name: String,
    pub version: Option<ModVersion>,
    pub enabled: ModEnabledType,
}

// impls for sorting the mod list for loading order
impl PartialOrd for Mod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.name == other.name {
            return None;
        }
        Some(self.cmp(other))
    }
}

impl Ord for Mod {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.has_dependency(&other.name) {
            Ordering::Greater
        } else {
            natural_only_alnum_cmp(&self.name, &other.name)
        }
    }
}

impl PartialEq for Mod {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.name == other.name
    }
}

impl Eq for Mod {}

impl Mod {
    // Check if this mod has other mod as a dependency
    fn has_dependency(&self, dep_name: &str) -> bool {
        match &self.version {
            Some(version) => {
                for dependency in &version.dependencies {
                    if dependency.name == dep_name {
                        matches!(&dependency.dep_type, ModDependencyType::Optional | ModDependencyType::Required | ModDependencyType::OptionalHidden);
                    }
                }
                false
            },
            _ => false,
        }
    }
}

// Struct for Mod version (or file, terminology isn't perfect)
#[derive(Debug)]
pub struct ModVersion {
    pub dependencies: Vec<ModDependency>,
    pub version: Version,
}

// impls for comparing mod versions
impl PartialOrd for ModVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Ord for ModVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl PartialEq for ModVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl Eq for ModVersion {}

// Structs for deserializing json files
#[derive(Deserialize, Debug)]
pub struct InfoJson {
    pub name: String,
    pub version: Version,
    pub title: String,
    pub author: ModAuthor,
    pub contact: Option<String>,
    pub homepage: Option<String>,
    pub description: Option<String>,
    pub factorio_version: FactorioVersion,
    #[serde(default = "default_dependencies")]
    pub dependencies: Vec<ModDependency>
}

fn default_dependencies() -> Vec<ModDependency> {
    vec![ModDependency{dep_type: ModDependencyType::Required, name: "base".into(), version_req: None}]
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ModAuthor {
    Author(String),
    Authors(Vec<String>)
}

#[derive(Deserialize)]
pub struct ModListJson {
    pub mods: Vec<ModListJsonMod>,
}

#[derive(Deserialize)]
pub struct ModListJsonMod {
    pub name: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Error)]
pub enum ModDependencyErr {
    #[error("Invalid dependency string: `{0}`")]
    InvalidDependencyString(String),
    #[error("Invalid dependency version requirement: `{0}`")]
    InvalidVersionReq(String),
    #[error("Dependency name could not be parsed: `{0}`")]
    NameIsUnparsable(String),
    #[error("Unknown dependency modifier: `{0}`")]
    UnknownModifier(String),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FactorioVersion {
    #[serde(rename = "0.13")]
    v0_13,
    #[serde(rename = "0.14")]
    v0_14,
    #[serde(rename = "0.15")]
    v0_15,
    #[serde(rename = "0.16")]
    v0_16,
    #[serde(rename = "0.17")]
    v0_17,
    #[serde(rename = "0.18")]
    v0_18,
    #[serde(rename = "1.0")]
    v1_0,
    #[serde(rename = "1.1")]
    v1_1
}

impl FromStr for FactorioVersion {
    type Err = FactorioVersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0.13" => Ok(Self::v0_13),
            "0.14" => Ok(Self::v0_14),
            "0.15" => Ok(Self::v0_15),
            "0.16" => Ok(Self::v0_16),
            "0.17" => Ok(Self::v0_17),
            "0.18" => Ok(Self::v0_18),
            "1.0" => Ok(Self::v1_0),
            "1.1" => Ok(Self::v1_1),
            _ => Err(FactorioVersionParseError(s.into()))
        }
    }
}

impl Display for FactorioVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::v0_13 => "0.13",
            Self::v0_14 => "0.14",
            Self::v0_15 => "0.15",
            Self::v0_16 => "0.16",
            Self::v0_17 => "0.17",
            Self::v0_18 => "0.18",
            Self::v1_0 => "1.0",
            Self::v1_1 => "1.1"
        })
    }
}

#[derive(Debug, Error)]
pub struct FactorioVersionParseError(String);

impl Display for FactorioVersionParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Incorrect factorio version string: \"{}\"", self.0)
    }
}

#[derive(Debug, Error)]
pub enum ModDataErr {
    #[error("Filesystem error")]
    FilesystemError,
    #[error("Invalid mod sctucture")]
    InvalidModStructure,
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
}
