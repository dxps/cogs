use crate::{app::AppError, domain::model::UserAccount};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl LoginRequest {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    /// Get the JSON representation of it.
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct LoginResponse {
    pub session: Option<String>,
    pub expires_in_seconds: Option<i64>,
    pub user: Option<UserAccount>,
    pub error: Option<AppError>,
}
