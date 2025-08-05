use crate::views::{ExploreCategory, ExploreKind, ViewType};
use cogs_shared::{
    app::AppError,
    domain::model::{
        Id, UserAccount,
        meta::{AttributeValueType, Kind},
    },
};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    pub view_type: ViewType,
    pub auth: AuthState,
    pub explore: ExploreViewState,
    pub data_mgmt: DataMgmtState,
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
    pub add_kind: Option<Kind>,
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DataMgmtState {
    pub curr_attr_template: ManagedAttrTemplate,
}

#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManagedAttrTemplate {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_required: bool,
}
