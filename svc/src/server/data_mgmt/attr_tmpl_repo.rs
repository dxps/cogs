use crate::utils::uuid_from;
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{Id, meta::AttrTemplate},
};
use sqlx::{PgPool, types::Uuid};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AttrTemplateRepo {
    dbcp: Arc<PgPool>,
}

impl AttrTemplateRepo {
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    /// Retrieve all attribute templates.
    pub async fn get_all(&self) -> AppResult<Vec<AttrTemplate>> {
        let rows = sqlx::query_as!(
            AttrTemplateRow,
            r#"
            SELECT
                id,
                name,
                description,
                value_type,
                default_value,
                required
            FROM attr_templates
            ORDER BY name ASC
            "#
        )
        .fetch_all(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from(err.to_string()))?;

        let data = rows
            .into_iter()
            .map(|r| AttrTemplate {
                id: Id::from(r.id.to_string()),
                name: r.name,
                description: r.description.unwrap_or_default(),
                value_type: r.value_type.into(),
                default_value: r.default_value.unwrap_or_default(), // model is String
                is_required: r.required.unwrap_or(false),
            })
            .collect::<Vec<_>>();

        Ok(data)
    }

    /// Insert or update an attribute template.
    pub async fn upsert(&self, attr_templ: &AttrTemplate) -> AppResult<()> {
        log::debug!("upsert_attr_templ: {attr_templ:?}");

        sqlx::query!(
            r#"
            INSERT INTO attr_templates (id, name, description, value_type, default_value, required)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE
                SET name = EXCLUDED.name,
                    description = EXCLUDED.description,
                    value_type = EXCLUDED.value_type,
                    default_value = EXCLUDED.default_value,
                    required = EXCLUDED.required
            "#,
            uuid_from(&attr_templ.id),
            &attr_templ.name,
            &attr_templ.description,
            attr_templ.value_type.to_string(),
            &attr_templ.default_value, // String in model
            attr_templ.is_required,
        )
        .execute(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from(err.to_string()))?;

        Ok(())
    }

    /// Delete an attribute template.
    pub async fn delete(&self, id: Id) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM attr_templates WHERE id = $1"#, uuid_from(&id),)
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err.to_string()))?;

        Ok(())
    }
}

#[derive(Debug)]
struct AttrTemplateRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    value_type: String,
    default_value: Option<String>,
    required: Option<bool>,
}
