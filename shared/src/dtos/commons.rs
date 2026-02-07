use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IdDto {
    pub id: Id,
}
