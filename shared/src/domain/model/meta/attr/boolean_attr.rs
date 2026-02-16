use crate::domain::model::{Id, meta::AttrTemplate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanAttribute {
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: bool,

    /// Its template identifier.
    pub tmpl_id: Id,

    /// Its owner identifier.
    pub owner_id: Id,
}

impl BooleanAttribute {
    pub fn new(id: Id, name: String, value: bool, tmpl_id: Id, owner_id: Id) -> Self {
        Self {
            id,
            name,
            value,
            tmpl_id,
            owner_id,
        }
    }
}

impl From<AttrTemplate> for BooleanAttribute {
    fn from(attr_def: AttrTemplate) -> Self {
        let value = attr_def.default_value == "true";
        Self::new(
            Id::default(), // its id
            attr_def.name, // its name
            value,         // its default value
            attr_def.id,   // its template id
            Id::default(), // owner id
        )
    }
}
