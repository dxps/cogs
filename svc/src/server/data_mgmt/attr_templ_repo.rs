use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::meta::AttrTemplate,
};
use sqlx::{PgPool, Row, postgres::PgRow};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AttrTemplateRepo {
    dbcp: Arc<PgPool>,
}

impl AttrTemplateRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_all(&self) -> AppResult<Vec<AttrTemplate>> {
        //
        let data = sqlx::query("SELECT * FROM attr_templates")
            .fetch_all(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err.to_string()))
            .map(|rows| {
                rows.iter()
                    .map(|row| from_row(row).unwrap())
                    .collect::<Vec<AttrTemplate>>()
            })?;
        Ok(data)
    }

    pub async fn upsert_attr_templ(&self, attr_templ: &AttrTemplate) -> AppResult<()> {
        //
        log::debug!("upsert_attr_templ: {attr_templ:#?}");

        sqlx::query(
            "INSERT INTO attr_templates (id, name, description, value_type, default_value, required) 
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET name = $2, description = $3, value_type = $4, default_value = $5, required = $6",
        )
        .bind(attr_templ.id.0)
        .bind(&attr_templ.name)
        .bind(&attr_templ.description)
        .bind(&attr_templ.value_type.to_string())
        .bind(&attr_templ.default_value)
        .bind(&attr_templ.is_required)
        .execute(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from(err.to_string()))
        .map(|_| ())
    }
}

fn from_row(row: &PgRow) -> Result<AttrTemplate, sqlx::Error> {
    Ok(AttrTemplate {
        id: row.get::<i64, _>("id").into(),
        name: row.get("name"),
        description: row.get("description"),
        value_type: row.get::<String, _>("value_type").into(),
        default_value: row.get("default_value"),
        is_required: row.get("required"),
    })
}
