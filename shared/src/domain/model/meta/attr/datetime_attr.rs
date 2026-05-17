use crate::domain::model::{Id, meta::AttrTemplate};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateTimeAttribute {
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: NaiveDateTime,

    /// Its (optional) template id.
    pub tmpl_id: Option<Id>,

    /// Its owner (item) id.
    pub owner_id: Id,
}

impl DateTimeAttribute {
    pub fn new(
        id: Id,
        name: String,
        value: NaiveDateTime,
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

    pub fn now_value() -> NaiveDateTime {
        Local::now().naive_local()
    }
}

impl From<AttrTemplate> for DateTimeAttribute {
    fn from(at: AttrTemplate) -> Self {
        let value: NaiveDateTime =
            NaiveDateTime::parse_from_str(&at.default_value, "%Y-%m-%d %H:%M:%S%.3f")
                .unwrap_or_else(|_| Local::now().naive_local());

        Self::new(
            Id::default(), // its id
            at.name,       // its name
            value,         // its value
            Some(at.id),   // its template id
            Id::default(), // its owner id
        )
    }
}
