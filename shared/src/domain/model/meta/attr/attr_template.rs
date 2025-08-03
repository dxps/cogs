use crate::domain::model::{Id, meta::AttributeValueType};

#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
/// A template for an attribute.
pub struct AttrTemplate {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_required: bool,
    pub tag_id: Option<Id>,
}
