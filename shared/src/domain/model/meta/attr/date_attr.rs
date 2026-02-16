use crate::domain::model::{Id, meta::AttrTemplate};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateAttribute {
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: NaiveDate,

    /// Its template id.
    pub tmpl_id: Id,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl DateAttribute {
    pub fn new(id: Id, name: String, value: NaiveDate, tmpl_id: Id, owner_id: Id) -> Self {
        Self {
            id,
            name,
            value,
            tmpl_id,
            owner_id,
        }
    }
}

impl From<AttrTemplate> for DateAttribute {
    fn from(attr_def: AttrTemplate) -> Self {
        let value = NaiveDate::from_ymd_opt(attr_def.default_value.parse().unwrap(), 1, 1)
            .unwrap_or_else(|| {
                log::error!(
                    "Failed to parse default value '{}' for date attribute '{}'.",
                    attr_def.default_value,
                    attr_def.name,
                );
                NaiveDate::from_ymd_opt(2026, 1, 1).unwrap() // Defaults to 2026-01-01, if parsing fails.
            });

        Self::new(
            Id::default(), // its id
            attr_def.name, // its name
            value,         // its default value
            attr_def.id,   // its template id
            Id::default(), // owner id
        )
    }
}
