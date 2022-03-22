use semver::Version;
use crate::data_structs::{FactorioVersion, InfoJson};
use const_format::concatcp;
use serde::{Serialize, Deserialize};

pub const MOD_PORTAL_URL: &str = "https://mods.factorio.com";
pub const MOD_PORTAL_MODS_API_URL: &str = concatcp!(MOD_PORTAL_URL, "/api/mods");

pub fn mod_portal_mod_info_url(mod_name: &str) -> String {
    format!("{}/{}", MOD_PORTAL_MODS_API_URL, mod_name)
}

pub fn mod_portal_mod_info_full_url(mod_name: &str) -> String {
    format!("{}/full", mod_portal_mod_info_url(mod_name))
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum PageSize {
    Number(u64),
    #[serde(rename = "max")]
    Max
}

#[derive(Debug, Serialize)]
pub struct ModPortalModsApiRequestParameters {
    pub hide_deprecated: bool,
    pub sort: ModPortalRequestSort,
    pub sort_order: ModPortalRequestSortOrder,
    pub page: Option<u64>,
    pub page_size: Option<PageSize>,
    pub namelist: Option<Vec<String>>,
    pub version: Option<FactorioVersion>
}

impl ModPortalModsApiRequestParameters {
    pub fn new() -> Self {
        Self{
            hide_deprecated: false,
            page: None,
            page_size: None,
            sort: ModPortalRequestSort::Name,
            sort_order: ModPortalRequestSortOrder::Descending,
            namelist: None,
            version: None
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModPortalRequestSort {
    Name,
    CreatedAt,
    UpdatedAt
}

#[derive(Debug, Serialize)]
pub enum ModPortalRequestSortOrder {
    #[serde(rename = "asc")]
    Ascending,
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
