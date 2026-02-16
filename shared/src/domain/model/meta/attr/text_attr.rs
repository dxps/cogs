use crate::domain::model::{Id, meta::AttrTemplate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextAttribute {
    //
    /// Its identifier.
    pub id: Id,

    /// Its name (inherited from its definition).
    pub name: String,

    /// Its value.
    pub value: String,

    /// Its template id.
    pub tmpl_id: Id,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl TextAttribute {
    pub fn new(id: Id, name: String, value: String, tmpl_id: Id, owner_id: Id) -> Self {
        Self {
            id,
            name,
            value,
            tmpl_id,
            owner_id,
        }
    }
}

impl From<AttrTemplate> for TextAttribute {
    fn from(attr_def: AttrTemplate) -> Self {
        Self::new(
            Id::default(),          // its id
            attr_def.name,          // its name
            attr_def.default_value, // its default value
            attr_def.id,            // its template id
            Id::default(),          // owner id
        )
    }
}
