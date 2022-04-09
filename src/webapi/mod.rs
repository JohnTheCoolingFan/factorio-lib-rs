use strum::Display;
use semver::Version;
use crate::data_structs::{FactorioVersion, InfoJson};
use const_format::concatcp;
use serde::{Serialize, Deserialize};

pub mod auth;
pub mod mod_upload;

pub const MOD_PORTAL_URL: &str = "https://mods.factorio.com";
pub const MOD_PORTAL_MODS_API_URL: &str = concatcp!(MOD_PORTAL_URL, "/api/mods");

pub fn mod_portal_mod_info_url(mod_name: &str) -> String {
    format!("{}/{}", MOD_PORTAL_MODS_API_URL, mod_name)
}

pub fn mod_portal_mod_info_full_url(mod_name: &str) -> String {
    format!("{}/full", mod_portal_mod_info_url(mod_name))
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(untagged)]
pub enum PageSize {
    Number(u64),
    #[serde(rename = "max")]
    Max
}

impl std::fmt::Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => f.write_fmt(format_args!("{}", n)),
            Self::Max => f.write_str("max")
        }
    }
}

#[derive(Debug, Default)]
pub struct ModPortalModsApiRequestParameters {
    pub hide_deprecated: Option<bool>,
    pub sort: Option<ModPortalRequestSort>,
    pub sort_order: Option<ModPortalRequestSortOrder>,
    pub page: Option<u64>,
    pub page_size: Option<PageSize>,
    pub namelist: Option<Vec<String>>,
    pub version: Option<FactorioVersion>
}

impl ModPortalModsApiRequestParameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hide_deprecated(&mut self, v: bool) -> &mut Self {
        self.hide_deprecated = Some(v); self
    }

    pub fn sort(&mut self, v: ModPortalRequestSort) -> &mut Self {
        self.sort = Some(v); self
    }

    pub fn sort_order(&mut self, v: ModPortalRequestSortOrder) -> &mut Self {
        self.sort_order = Some(v); self
    }
     
    pub fn page(&mut self, v: u64) -> &mut Self {
        self.page = Some(v); self
    }
     
    pub fn page_size(&mut self, v: PageSize) -> &mut Self {
        self.page_size = Some(v); self
    }
     
    pub fn namelist(&mut self, v: Vec<String>) -> &mut Self {
        self.namelist = Some(v); self
    }
     
    pub fn version(&mut self, v: FactorioVersion) -> &mut Self {
        self.version = Some(v); self
    }

    fn to_query_parameters(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        if let Some(hide_deprecated) = self.hide_deprecated {
            Self::push_arg(&mut result, "hide_deprecated", hide_deprecated)
        }
        if let Some(sort) = self.sort {
            Self::push_arg(&mut result, "sort", sort)
        }
        if let Some(sort_order) = self.sort_order {
            Self::push_arg(&mut result, "sort_order", sort_order)
        }
        if let Some(page) = self.page {
            Self::push_arg(&mut result, "page", page)
        }
        if let Some(page_size) = self.page_size {
            Self::push_arg(&mut result, "page_size", page_size)
        }
        if let Some(namelist) = &self.namelist {
            Self::push_arg(&mut result, "page_size", namelist.clone().join(","))
        }
        if let Some(version) = self.version {
            Self::push_arg(&mut result, "version", version)
        }
        result
    }

    fn push_arg(result: &mut Vec<(String, String)>, name: &str, value: impl ToString) {
        result.push((name.into(), value.to_string()))
    }
}

impl Serialize for ModPortalModsApiRequestParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        self.to_query_parameters().serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy, Display, Serialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ModPortalRequestSort {
    Name,
    CreatedAt,
    UpdatedAt
}

#[derive(Debug, Clone, Copy, Display, Serialize)]
pub enum ModPortalRequestSortOrder {
    #[strum(serialize = "asc")]
    #[serde(rename = "asc")]
    Ascending,
    #[strum(serialize = "desc")]
    #[serde(rename = "desc")]
    Descending
}

#[derive(Debug, Deserialize)]
pub struct ModListResponse {
    pub pagination: Pagination,
    pub results: Vec<ModListResult>
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub count: u64,
    pub links: PaginationLinks,
    pub page: u64,
    pub page_count: u64,
    pub page_size: u64
}

// TODO: parse to url
#[derive(Debug, Deserialize)]
pub struct PaginationLinks {
    pub first: String,
    pub prev: String,
    pub next: String,
    pub last: String
}

#[derive(Debug, Deserialize)]
pub struct ModListResult {
    pub downloads_count: u64,
    pub latest_release: ModRelease,
    pub name: String,
    pub owner: String,
    pub summary: String,
    pub title: String,
    pub category: Option<ModTag>
}

#[derive(Debug, Deserialize)]
pub struct ModResultShort {
    pub downloads_count: u64,
    pub name: String,
    pub owner: String,
    pub releases: Vec<ModRelease>,
    pub summary: String,
    pub title: String,
    pub category: Option<ModTag>
}

#[derive(Debug, Deserialize)]
pub struct ModResultFull {
    pub downloads_count: u64,
    pub name: String,
    pub owner: String,
    pub releases: Vec<ModRelease>,
    pub summary: String,
    pub title: String,
    pub category: Option<ModTagName>,
    pub changelog: String,
    pub created_at: String, // TODO: parse iso
    pub description: String,
    pub github_path: String,
    pub homepage: String,
    pub tag: ModTag
}

#[derive(Debug, Deserialize)]
pub struct ModRelease {
    pub download_url: String,
    pub file_name: String,
    pub info_json: InfoJson,
    pub released_at: String, // TODO: parse iso
    pub version: Version,
    pub sha1: String
}

impl ModRelease {
    pub fn full_download_url(&self) -> String {
        format!("{}{}", MOD_PORTAL_URL, self.download_url)
    }
}

#[derive(Debug, Deserialize)]
pub struct ModTag {
    pub name: ModTagName
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModTagName {
    General,
    NonGameChanging,
    HelperMods,
    BigMods,
    Transportation,
    Logistics,
    Utility,
    Balancing,
    Enemies,
    Weapons,
    Armor,
    Oil,
    LogisticsNetwork,
    Storage,
    PowerProduction,
    Manufacture,
    Blueprints,
    Cheats,
    Defense,
    Mining,
    Info,
    Trains
}
