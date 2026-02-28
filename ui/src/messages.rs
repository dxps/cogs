use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{
        Id, UserAccount,
        meta::{AttrTemplate, ItemTemplate, Kind},
    },
};

#[derive(Clone, Debug, PartialEq)]
pub enum UiMessage {
    Login(Result<Option<(UserAccount, String)>, AppError>),
    Logout,

    Settings,

    // TODO: Have the _Upserted and _Deleted messages more reusable
    //       by including the element type.
    AttrTemplateUpserted(Result<Id, AppError>),
    AttrTemplateDeleted(Result<Id, AppError>),

    ElementCreated(Kind, AppResult<Id>),
    ElementUpdated(Kind, AppResult<Id>),
    ElementDeleted(Kind, AppResult<Id>),

    AttrTemplatesFetched(Result<Vec<AttrTemplate>, AppError>),
    ItemTemplatesFetched(Result<Vec<ItemTemplate>, AppError>),
}
