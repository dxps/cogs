use crate::views::{ExploreCategory, ExploreKind, ViewType};
use cogs_shared::{
    app::AppError,
    domain::model::{UserAccount, meta::ItemTemplate},
};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    pub view_type: ViewType,
    pub auth: AuthState,
    pub explore: ExploreViewState,
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AuthState {
    pub user: String,
    // #[serde(skip)]  // todo: temporary stored, during development
    pub pass: String,
    pub login_error: Option<AppError>,
    pub user_account: Option<UserAccount>,
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ExploreViewState {
    pub category: ExploreCategory,
    pub kind: ExploreKind,
    pub template: Option<ItemTemplate>,
}
