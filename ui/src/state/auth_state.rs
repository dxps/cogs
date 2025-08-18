use cogs_shared::{app::AppError, domain::model::UserAccount};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AuthState {
    pub user: String,
    // #[serde(skip)]  // todo: temporary stored, during development
    pub pass: String,
    #[serde(skip)]
    pub login_user_focus: bool,
    #[serde(skip)]
    /// Used to determine if the enter key was pressed, after the password field was focused.
    pub login_pass_enter: bool,
    pub login_error: Option<AppError>,
    pub user_account: Option<UserAccount>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: Default::default(),
            pass: Default::default(),
            login_user_focus: true,
            login_pass_enter: false,
            login_error: Default::default(),
            user_account: Default::default(),
        }
    }
}
