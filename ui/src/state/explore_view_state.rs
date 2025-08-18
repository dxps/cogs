use cogs_shared::domain::model::meta::Kind;

use crate::views::{ExploreCategory, ExploreKind};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ExploreViewState {
    pub category: ExploreCategory,
    pub kind: ExploreKind,
    pub add_kind: Option<Kind>,
}
