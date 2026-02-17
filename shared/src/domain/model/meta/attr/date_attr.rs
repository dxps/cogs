use crate::domain::model::{Id, meta::AttrTemplate};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateAttribute {
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: NaiveDate,

    /// Its (optional) template id.
    pub tmpl_id: Option<Id>,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl DateAttribute {
    pub fn new(id: Id, name: String, value: NaiveDate, tmpl_id: Option<Id>, owner_id: Id) -> Self {
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
    fn from(at: AttrTemplate) -> Self {
        let value = NaiveDate::from_ymd_opt(at.default_value.parse().unwrap(), 1, 1)
            .unwrap_or_else(|| {
                log::error!(
                    "Failed to parse default value '{}' for date attribute '{}'.",
                    at.default_value,
                    at.name,
                );
                NaiveDate::from_ymd_opt(2026, 1, 1).unwrap() // Defaults to 2026-01-01, if parsing fails.
            });

        Self::new(
            Id::default(), // its id
            at.name,       // its name
            value,         // its default value
            Some(at.id),   // its template id
            Id::default(), // owner id
        )
    }
}
