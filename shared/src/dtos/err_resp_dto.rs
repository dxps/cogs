use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl From<String> for ErrorResponse {
    fn from(error: String) -> Self {
        Self { error }
    }
}
