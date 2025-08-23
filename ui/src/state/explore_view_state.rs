use crate::{
    ManagedAttrTemplate,
    views::{ExploreCategory, ExploreKind},
};
use cogs_shared::domain::model::{
    Id,
    meta::{Kind, LinkTemplate},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ExploreViewState {
    pub category: ExploreCategory,

    pub kind: ExploreKind,

    /// The open windows for creating (one) or editing (one or more) attribute templates.
    #[serde(skip)]
    pub open_windows_attr_template: HashMap<Id, Arc<Mutex<ManagedAttrTemplate>>>,

    /// The open windows for creating (one) or editing (one or more) link templates.
    #[serde(skip)]
    pub open_windows_link_template: HashMap<Id, Arc<Mutex<LinkTemplate>>>,

    /// The element that is currently clicked in the Explore's table.
    #[serde(skip)]
    pub curr_sel_elem: Option<(Kind, Id)>,
}
