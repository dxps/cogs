use sqlx::{Error, FromRow, Row, postgres::PgRow};

use crate::domain::model::Id;

impl FromRow<'_, PgRow> for Id {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Id::from(row.get::<i64, _>("id")))
    }
}
