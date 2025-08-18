use crate::domain::model::Id;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct IdDto {
    pub id: Id,
}
