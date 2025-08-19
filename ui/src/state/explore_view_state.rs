use std::collections::HashMap;

use cogs_shared::domain::model::{Id, meta::Kind};

use crate::views::{ExploreCategory, ExploreKind};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ExploreViewState {
    pub category: ExploreCategory,

    pub kind: ExploreKind,

    pub add_kind: Option<Kind>,

    /// The id of the element that is currently selected row in the Explore's table.
    #[serde(skip)]
    pub curr_sel_row_elem_id: Option<Id>,

    /// The open windows.
    #[serde(skip)]
    pub open_windows: HashMap<(Kind, Id), String>,
}
