use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessLevel {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
}

impl AccessLevel {
    pub fn new(id: Id, name: impl Into<String>, description: Option<impl Into<String>>) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.map(Into::into),
        }
    }
}
