//! <https://wiki.factorio.com/Mod_upload_API>
//!
//! This module provides an easy interface to upload mods to a mod portal

use reqwest::{Body, Client, StatusCode};
use serde::Deserialize;
use std::fmt::Display;
use strum::Display;
use thiserror::Error;

pub const INIT_UPLOAD_URL: &str = "https://mods.factorio.com/api/v2/mods/releases/init_upload";

pub async fn upload_mod(
    client: &Client,
    api_key: &str,
    mod_name: &str,
    file: impl Into<Body>,
) -> Result<(), ModUploadError> {
    let response = client
        .post(INIT_UPLOAD_URL)
        .bearer_auth(api_key)
        .body(mod_name.to_string())
        .send()
        .await?;
    if response.status() == StatusCode::OK {
        let data = response.json::<ModUploadInitResponse>().await?;
        let upload_response = client.post(data.upload_url).body(file).send().await?;
        if upload_response.status() != StatusCode::OK {
            Err(upload_response
                .json::<ModUploadErrorResponse>()
                .await?
                .into())
        } else {
            Ok(())
        }
    } else {
        Err(response.json::<ModUploadErrorResponse>().await?.into())
    }
}

#[derive(Debug, Deserialize)]
pub struct ModUploadInitResponse {
    pub upload_url: String,
}

#[derive(Debug, Error, Deserialize)]
pub struct ModUploadErrorResponse {
    pub error: ModUploadErrorKind,
    pub message: String,
}

impl Display for ModUploadErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to upload with error `{}`, message: {}",
            self.error, self.message
        )
    }
}

#[derive(Debug, Deserialize, Display)]
pub enum ModUploadErrorKind {
    InvalidApiKey,
    InvalidRequest,
    InternalError,
    Forbidden,
    Unknown,
    InvalidModRelease,
    InvalidModUpload,
    UnknownMod,
}

#[derive(Debug, Error)]
pub enum ModUploadError {
    #[error("{0}")]
    Upload(ModUploadErrorResponse),
    #[error("Client error: {0}")]
    Client(reqwest::Error),
}

impl From<reqwest::Error> for ModUploadError {
    fn from(e: reqwest::Error) -> Self {
        Self::Client(e)
    }
}

impl From<ModUploadErrorResponse> for ModUploadError {
    fn from(e: ModUploadErrorResponse) -> Self {
        Self::Upload(e)
    }
}
