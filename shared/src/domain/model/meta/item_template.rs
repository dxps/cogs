use crate::domain::model::{Id, meta::AttrTemplate};

#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
/// A template for an item .
pub struct ItemTemplate {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Vec<AttrTemplate>,
    pub listing_attr_def_id: Id,
}
