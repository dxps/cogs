use crate::domain::model::{Id, meta::AttrTemplate};
use chrono::{NaiveDate, Utc};
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

    pub fn now_value() -> NaiveDate {
        Utc::now().date_naive()
    }
}

impl From<AttrTemplate> for DateAttribute {
    fn from(at: AttrTemplate) -> Self {
        let value = NaiveDate::from_ymd_opt(at.default_value.parse().unwrap(), 1, 1)
            .unwrap_or_else(|| {
                log::error!(
                    "Failed to parse value '{}' for date attribute '{}'.",
                    at.default_value,
                    at.name,
                );
                Utc::now().date_naive()
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
