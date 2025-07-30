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
