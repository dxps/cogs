use serde::{Deserialize, Serialize};

use crate::{app_err_uc::AppError, domain::model::UserAccount};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResult {
    pub is_succcess: bool,
    pub account: Option<UserAccount>,
    pub error: Option<AppError>,
}

impl From<AppError> for LoginResult {
    fn from(app_err: AppError) -> Self {
        Self {
            is_succcess: false,
            account: None,
            error: Some(app_err),
        }
    }
}
