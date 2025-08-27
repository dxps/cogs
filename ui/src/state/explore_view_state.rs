use crate::views::{ExploreCategory, ExploreKind};
use cogs_shared::domain::model::{
    Id,
    meta::{AttrTemplate, ItemTemplate, Kind, LinkTemplate},
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

    /// The open windows for creating (one) or editing attribute templates.
    #[serde(skip)]
    pub open_windows_attr_template: HashMap<Id, Arc<Mutex<AttrTemplate>>>,

    /// The open windows for creating (one) or editing link templates.
    #[serde(skip)]
    pub open_windows_link_template: HashMap<Id, Arc<Mutex<LinkTemplate>>>,

    /// The open windows for creating (one) or editing item templates.
    // #[serde(skip)]  todo: temporary used during form dev.
    pub open_windows_item_template: HashMap<Id, Arc<Mutex<ItemTemplate>>>,

    /// The element that is currently clicked (not double clicked) in the Explore's table.
    #[serde(skip)]
    pub curr_sel_elem: Option<(Kind, Id)>,

    // --------------------------------
    // State of new item template form.
    // --------------------------------
    //
    /// The attribute template that is selected as listing attribute.
    #[serde(skip)]
    pub add_item_template_listing_attr_template: AttrTemplate,

    /// The attribute template that is selected to be added.
    #[serde(skip)]
    pub add_item_template_add_attr_template: AttrTemplate,
}
