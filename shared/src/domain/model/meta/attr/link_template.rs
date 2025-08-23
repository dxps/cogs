use crate::domain::model::{Id, meta::ItemTemplate};
use serde::{Deserialize, Serialize};

/// A template for a link.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LinkTemplate {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub target: ItemTemplate,
    pub is_required: bool,
}
