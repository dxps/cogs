use cogs_shared::{app::AppError, domain::model::UserAccount};

use crate::ManagedAttrTemplate;

#[derive(Clone, Debug)]
pub enum UiMessage {
    Login(Result<Option<UserAccount>, AppError>),
    Logout,
    Settings,
    AttrTemplatesFetched(Result<Vec<ManagedAttrTemplate>, AppError>),
}
