use serde::{Deserialize, Serialize};

use crate::{app_err_uc::AppError, domain::model::UserAccount};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentUserAccount {
    pub is_fetched: bool,
    pub account: Option<UserAccount>,
    pub error: Option<AppError>,
}

impl From<AppError> for CurrentUserAccount {
    fn from(app_err: AppError) -> Self {
        Self {
            is_fetched: true,
            account: None,
            error: Some(app_err),
        }
    }
}
