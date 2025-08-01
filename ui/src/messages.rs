use cogs_shared::{app::AppError, domain::model::UserAccount};

#[derive(Debug)]
pub enum UiMessage {
    Login(Result<UserAccount, AppError>),
    Logout,
    Settings,
}
