use crate::domain::model::{Id, meta::AttributeValueType};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
/// A template for an attribute.
pub struct AttrTemplate {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_required: bool,
}

impl Hash for AttrTemplate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl AttrTemplate {
    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }
}
