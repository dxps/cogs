use crate::domain::model::Id;
use sqlx::{
    Decode, Encode, Error, FromRow, Postgres, Row, Type, database::Database, encode::IsNull,
    error::BoxDynError, postgres::PgRow,
};

impl FromRow<'_, PgRow> for Id {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Id::new_from(row.get("id")))
    }
}

impl Type<Postgres> for Id {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <&[u8] as sqlx::Type<Postgres>>::type_info()
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <&[u8] as sqlx::Type<Postgres>>::compatible(ty)
    }
}

impl<'r> Encode<'r, Postgres> for Id {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'r>,
    ) -> Result<IsNull, BoxDynError> {
        let bytes: &[u8] = self.0.as_bytes();
        // TODO: Still not working: ...Id (SQL type `BYTEA`) is not compatible with SQL type `CHAR`".
        //       That's when doing something like this (ex from ent repo):
        //       id: row.get("id")
        //       instead of this:
        //       id: Id::from(row.get::<&str, &str>("id"))
        <&[u8] as Encode<'r, Postgres>>::encode_by_ref(&bytes, buf)
    }
}

impl<'r> Decode<'r, Postgres> for Id {
    fn decode(value: <Postgres as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let id = value.as_str()?;
        Ok(Id::new_from(id.to_string()))
    }
}
