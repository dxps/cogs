use crate::{
    ManagedAttrTemplate,
    views::{ExploreCategory, ExploreKind},
};
use cogs_shared::domain::model::Id;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ExploreViewState {
    pub category: ExploreCategory,

    pub kind: ExploreKind,

    /// The id of the element that is currently selected row in the Explore's table.
    #[serde(skip)]
    pub curr_sel_row_elem_id: Option<Id>,

    /// The open windows for creating (one) or editing (one or many) attribute templates.
    #[serde(skip)]
    pub open_attr_template_windows: HashMap<Id, Arc<Mutex<ManagedAttrTemplate>>>,
}
