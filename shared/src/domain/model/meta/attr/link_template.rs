use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

/// A template for a link.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LinkTemplate {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub target_item_template_id: Id,
    pub is_required: bool,
}
