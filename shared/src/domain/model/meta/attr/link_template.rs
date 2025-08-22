use crate::domain::model::{Id, meta::ItemTemplate};

#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
/// A template for a link.
pub struct LinkTemplate {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub target: ItemTemplate,
    pub is_required: bool,
}
