use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::meta::ItemTemplate,
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ItemTemplateRepo {
    dbcp: Arc<PgPool>,
}

impl ItemTemplateRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    /// Insert or update an item template.
    pub async fn upsert(&self, item_templ: &ItemTemplate) -> AppResult<()> {
        //
        let mut txn = self
            .dbcp
            .begin()
            .await
            .map_err(|e| AppError::from(e.to_string()))?;

        if let Err(e) = sqlx::query("INSERT INTO item_templates (id, name, description, listing_attr_templ_id) VALUES ($1, $2, $3, $4)")
            .bind(item_templ.id.0)
            .bind(item_templ.name.clone())
            .bind(item_templ.description.clone())
            .bind(item_templ.listing_attr.id.0)
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await.map_err(|e| AppError::from(e.to_string()))?;
            log::error!("Failed to add item template: '{}'.", e);
            return AppResult::Err(AppError::from(e.to_string()));
        }

        for (index, attr_def) in item_templ.attributes.clone().iter().enumerate() {
            if let Err(e) =
                sqlx::query("INSERT INTO item_templates_attr_templates_xref (item_templ_id, attr_templ_id, show_index) VALUES ($1, $2, $3)")
                    .bind(item_templ.id.0)
                    .bind(attr_def.id.0)
                    .bind((index + 1) as i16)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await.map_err(|e| AppError::from(e.to_string()))?;
                log::error!("Failed to add item template's attribute templates: {}", e);
                return AppResult::Err(e.to_string().into());
            }
        }

        txn.commit()
            .await
            .map_err(|e| AppError::from(e.to_string()))?;

        AppResult::Ok(())
    }
}
