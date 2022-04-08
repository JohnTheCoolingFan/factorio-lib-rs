use std::fmt::Display;
use thiserror::Error;

use serde::{Serialize, Deserialize};

pub const FACTORIO_API_AUTH: &str = "https://auth.factorio.com/api-login";

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
