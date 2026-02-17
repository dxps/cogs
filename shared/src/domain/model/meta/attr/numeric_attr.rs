use crate::domain::model::{Id, meta::AttrTemplate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NumericAttribute {
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: Decimal,

    /// Its (optional) template id.
    pub tmpl_id: Option<Id>,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl NumericAttribute {
    pub fn new(id: Id, name: String, value: Decimal, tmpl_id: Option<Id>, owner_id: Id) -> Self {
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
    fn from(at: AttrTemplate) -> Self {
        let value = match Decimal::from_str_exact(&at.default_value) {
            Ok(v) => v,
            Err(e) => {
                log::error!(
                    "Failed to parse default value '{}' for numeric attribute '{}': {}.",
                    at.default_value,
                    at.name,
                    e
                );
                Decimal::new(0, 0) // default to 0 if parsing fails
            }
        };
        Self::new(
            Id::default(), // its id
            at.name,       // its name
            value,         // its default value
            Some(at.id),   // its template id
            Id::default(), // owner id
        )
    }
}
