use crate::domain::model::{Id, meta::AttrTemplate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericAttribute {
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: Decimal,

    /// Its template id.
    pub tmpl_id: Id,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl NumericAttribute {
    pub fn new(id: Id, name: String, value: Decimal, tmpl_id: Id, owner_id: Id) -> Self {
        Self {
            id,
            name,
            value,
            tmpl_id,
            owner_id,
        }
    }
}

impl From<AttrTemplate> for NumericAttribute {
    fn from(attr_tmpl: AttrTemplate) -> Self {
        let value = match Decimal::from_str_exact(&attr_tmpl.default_value) {
            Ok(v) => v,
            Err(e) => {
                log::error!(
                    "Failed to parse default value '{}' for numeric attribute '{}': {}. Defaulting to 0.",
                    attr_tmpl.default_value,
                    attr_tmpl.name,
                    e
                );
                Decimal::new(0, 0) // default to 0 if parsing fails
            }
        };
        Self::new(
            Id::default(),  // its id
            attr_tmpl.name, // its name
            value,          // its default value
            attr_tmpl.id,   // its template id
            Id::default(),  // owner id
        )
    }
}
