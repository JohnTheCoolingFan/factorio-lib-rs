//! <https://wiki.factorio.com/Web_authentication_API>
//!
//! This module provides interface to authenticate to factorio.com using a username and password

use std::fmt::Display;
use thiserror::Error;
use reqwest::{Client, StatusCode};
use serde::{Serialize, Deserialize};

pub const FACTORIO_API_AUTH: &str = "https://auth.factorio.com/api-login";

pub async fn login(client: &Client, username: String, password: String, require_game_ownership: bool, email_auth_code: Option<String>) -> Result<LoginResponse, ClientLoginError> {
    let data = ApiLoginRequestParameters {
        username,
        password,
        require_game_ownership,
        email_authentication_code: email_auth_code
    };
    let response = client.post(FACTORIO_API_AUTH).json(&data).send().await?;
    if response.status() == StatusCode::OK {
        response.json::<LoginResponse>().await.map_err(ClientLoginError::from)
    } else {
        match response.json::<LoginError>().await.map_err(ClientLoginError::from) {
            Err(v) => Err(v),
            Ok(v) => Err(v.into())
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientLoginError {
    #[error("Error constructing/sending request: {0}")]
    Client(reqwest::Error),
    #[error("Login error: {0}")]
    Login(LoginError)
}

impl From<reqwest::Error> for ClientLoginError {
    fn from(v: reqwest::Error) -> Self {
        Self::Client(v)
    }
}

impl From<LoginError> for ClientLoginError {
    fn from(v: LoginError) -> Self {
        Self::Login(v)
    }
}

/// <https://wiki.factorio.com/Web_authentication_API>
/// Used for authenticating to factorio.
#[derive(Debug, Clone)]
pub struct ApiLoginRequestParameters {
    pub username: String,
    pub password: String,
    pub require_game_ownership: bool,
    pub email_authentication_code: Option<String>
}

impl ApiLoginRequestParameters {
    fn to_parameters(&self) -> Vec<(String, String)> {
        let mut result = vec![("username".into(), self.username.clone()), ("password".into(), self.password.clone())];
        if self.require_game_ownership {
            result.push(("require_game_ownership".into(), "true".into()))
        }
        if let Some(email_auth_code) = &self.email_authentication_code {
            result.push(("email_quthentication_code".into(), email_auth_code.clone()))
        }
        result
    }

    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            require_game_ownership: false,
            email_authentication_code: None
        }
    }
}

impl Serialize for ApiLoginRequestParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        self.to_parameters().serialize(serializer)    
    }
}

/// Use this if status code is 200.
#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String
}

/// Use this on non-200 status code
#[derive(Debug, Clone, Error, Deserialize)]
pub struct LoginError {
    pub error: String,
    pub message: String
}

impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Login failed with error `{}`. Message: {}", self.error, self.message)
    }
}
