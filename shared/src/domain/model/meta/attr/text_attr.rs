use crate::domain::model::{Id, meta::AttrTemplate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAttribute {
    //
    /// Its identifier.
    pub id: Id,

    /// Its name (inherited from its definition).
    pub name: String,

    /// Its value.
    pub value: String,

    /// Its (optional) template id.
    pub tmpl_id: Option<Id>,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl TextAttribute {
    pub fn new(id: Id, name: String, value: String, tmpl_id: Option<Id>, owner_id: Id) -> Self {
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
    fn from(at: AttrTemplate) -> Self {
        Self::new(
            Id::default(),    // its id
            at.name,          // its name
            at.default_value, // its default value
            Some(at.id),      // its template id
            Id::default(),    // owner id
        )
    }
}
