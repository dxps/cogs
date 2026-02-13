use crate::domain::model::{Id, meta::AttrTemplate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
/// A template for an item .
pub struct ItemTemplate {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub attributes: Vec<AttrTemplate>,
    pub listing_attr: AttrTemplate,
    pub links: Vec<ItemTemplateLink>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ItemTemplateLink {
    pub name: String,
    pub item_template_id: Id,
}
