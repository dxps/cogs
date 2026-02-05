use cogs_shared::{app::AppError, domain::model::UserAccount};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AuthState {
    pub user: String,

    // #[serde(skip)]  // TODO: Remove this line: temporary stored and userd during development.
    pub pass: String,

    #[serde(skip)]
    pub login_user_focus: bool,

    #[serde(skip)]
    /// Used to determine if the enter key was pressed, after the password field was focused.
    pub login_pass_enter: bool,

    pub login_error: Option<AppError>,

    pub user_account: Option<UserAccount>,
    pub user_session: Option<String>,
}
