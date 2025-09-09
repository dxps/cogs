use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{
        Id, UserAccount,
        meta::{AttrTemplate, Kind},
    },
};

#[derive(Clone, Debug)]
pub enum UiMessage {
    Login(Result<Option<UserAccount>, AppError>),
    Logout,

    Settings,

    // TODO: Have the _Upserted and _Deleted messages more reusable
    //       by including the element type.
    AttrTemplatesFetched(Result<Vec<AttrTemplate>, AppError>),
    AttrTemplateUpserted(Result<Id, AppError>),
    AttrTemplateDeleted(Result<Id, AppError>),

    ElementCreated(Kind, AppResult<Id>),
    ElementUpdated(Kind, AppResult<Id>),
    ElementDeleted(Kind, AppResult<Id>),
}
