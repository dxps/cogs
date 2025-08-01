use cogs_shared::{app::AppError, domain::model::UserAccount};

#[derive(Clone, Debug)]
pub enum UiMessage {
    Login(Result<Option<UserAccount>, AppError>),
    Logout,
    Settings,
}
