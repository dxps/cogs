use crate::views::{ExploreCategory, ExploreKind};
use cogs_shared::domain::model::{
    Id,
    meta::{AttrTemplate, Item, ItemTemplate, Kind},
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

    // The name and description columns widths of the explore table.
    pub table_col_widths: Option<[f32; 3]>,

    /// The open windows for creating (one) or editing (one or more) items.
    // #[serde(skip)]  todo: temporary used during form dev.
    pub open_windows_item: HashMap<Id, Arc<Mutex<Item>>>,

    /// The open windows for creating (one) or editing (one or more) attribute templates.
    // #[serde(skip)]  todo: temporary used during form dev.
    pub open_windows_attr_template: HashMap<Id, Arc<Mutex<AttrTemplate>>>,

    /// The open windows for creating (one) or editing (one or more) item templates.
    // #[serde(skip)]  todo: temporary used during form dev.
    pub open_windows_item_template: HashMap<Id, Arc<Mutex<ItemTemplate>>>,

    /// The element that is currently clicked (not double clicked) in the Explore's table.
    #[serde(skip)]
    pub curr_sel_elem: Option<(Kind, Id)>,

    // --------------------------------------------------------
    // State of `ItemTemplateWindow`s when creating or editing.
    // --------------------------------------------------------
    //
    /// The attribute template that is selected as listing attribute.
    #[serde(skip)]
    pub add_item_template_listing_attr_template: AttrTemplate,

    /// The attribute template that is selected to be added
    /// when creating or editing an item template.
    #[serde(skip)]
    pub item_template_cu_add_attr_template: HashMap<Id, Option<AttrTemplate>>,

    /// The item template that is selected to be added as a link
    /// when creating or editing an item template.
    #[serde(skip)]
    pub item_template_cu_add_link_template: HashMap<Id, Option<Id>>,
}
