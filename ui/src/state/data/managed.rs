use cogs_shared::domain::model::{
    Id,
    meta::{AttrTemplate, AttributeValueType},
};

/// The attribute template to be created or edited.
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManagedAttrTemplate {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_required: bool,
}

impl ManagedAttrTemplate {
    pub fn reset(&mut self) {
        self.id = Id::from(0);
        self.name = "".into();
        self.description = "".into();
        self.value_type = AttributeValueType::Text;
        self.default_value = "".into();
        self.is_required = false;
    }
}

impl From<AttrTemplate> for ManagedAttrTemplate {
    fn from(val: AttrTemplate) -> Self {
        Self {
            id: val.id,
            name: val.name,
            description: val.description.unwrap_or_default(),
            value_type: val.value_type,
            default_value: val.default_value,
            is_required: val.is_required,
        }
    }
}

impl From<&String> for ManagedAttrTemplate {
    fn from(value: &String) -> Self {
        serde_json::from_str::<Self>(value.as_str()).unwrap_or_default()
    }
}
