use crate::domain::model::{Id, meta::AttributeValueType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
/// A template for an attribute.
pub struct AttrTemplate {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_required: bool,
}
