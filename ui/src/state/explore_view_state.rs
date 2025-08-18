use cogs_shared::domain::model::{Id, meta::Kind};

use crate::views::{ExploreCategory, ExploreKind};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ExploreViewState {
    pub category: ExploreCategory,

    pub kind: ExploreKind,

    pub add_kind: Option<Kind>,

    #[serde(skip)]
    /// The id of the element that is currently selected row in the Explore's table.
    pub curr_sel_row_elem_id: Option<Id>,
    #[serde(skip)]
    /// The kind (type) of the element that is currently selected row in the Explore's table.
    pub curr_sel_row_elem_type: Option<Kind>,
}
