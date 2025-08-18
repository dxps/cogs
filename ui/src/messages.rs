use crate::ManagedAttrTemplate;
use cogs_shared::{
    app::AppError,
    domain::model::{Id, UserAccount},
};

#[derive(Clone, Debug)]
pub enum UiMessage {
    Login(Result<Option<UserAccount>, AppError>),
    Logout,
    Settings,
    AttrTemplatesFetched(Result<Vec<ManagedAttrTemplate>, AppError>),
    AttrTemplateUpserted(Result<Id, AppError>),
}
