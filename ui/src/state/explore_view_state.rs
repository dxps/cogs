use crate::{
    SourceType,
    views::{ExploreCategory, ExploreKind},
};
use cogs_shared::domain::model::{
    Id,
    meta::{AddAttribute, AttrTemplate, Item, ItemTemplate, Kind},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
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

    // ------------------------------------------------
    // State of `ItemWindow`s when creating or editing.
    // ------------------------------------------------
    //
    /// An item can be created from scratch or from a template.
    /// This tuple contains `source type`, `item template` (if source type is `Template` and user selected one) and `continue`.
    pub add_item_src_type_tmpl_cont: Option<(Option<SourceType>, Option<ItemTemplate>, bool)>,

    /// The attribute that is selected to be added when creating or editing an item template.
    /// This is mapped by item `Id`.
    // #[serde(skip)] // TODO: used during dev.
    pub item_cu_add_attr: HashMap<Id, AddAttribute>,
}
