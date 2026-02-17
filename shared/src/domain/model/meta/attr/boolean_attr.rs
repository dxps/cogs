use crate::domain::model::{Id, meta::AttrTemplate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BooleanAttribute {
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: bool,

    /// Its (optional) template id.
    pub tmpl_id: Option<Id>,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl BooleanAttribute {
    pub fn new(id: Id, name: String, value: bool, tmpl_id: Option<Id>, owner_id: Id) -> Self {
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
    fn from(at: AttrTemplate) -> Self {
        let value = at.default_value == "true";
        Self::new(
            Id::default(), // its id
            at.name,       // its name
            value,         // its default value
            Some(at.id),   // its template id
            Id::default(), // owner id
        )
    }
}
