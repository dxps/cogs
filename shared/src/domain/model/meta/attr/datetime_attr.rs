use crate::domain::model::{Id, meta::AttrTemplate};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateTimeAttribute {
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: DateTime<chrono::Utc>,

    /// Its (optional) template id.
    pub tmpl_id: Option<Id>,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl DateTimeAttribute {
    pub fn new(
        id: Id,
        name: String,
        value: DateTime<Utc>,
        tmpl_id: Option<Id>,
        owner_id: Id,
    ) -> Self {
        Self {
            id,
            name,
            value,
            tmpl_id,
            owner_id,
        }
    }
}

impl From<AttrTemplate> for DateTimeAttribute {
    fn from(at: AttrTemplate) -> Self {
        let value: DateTime<Utc> = DateTime::parse_from_rfc3339(&at.default_value)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Self::new(
            Id::default(), // its id
            at.name,       // its name
            value,         // its value
            Some(at.id),   // its template id
            Id::default(), // its owner id
        )
    }
}
