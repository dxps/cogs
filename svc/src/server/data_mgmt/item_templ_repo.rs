use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{
        Id,
        meta::{AttrTemplate, AttributeValueType, ItemTemplate},
    },
};
use sqlx::{PgPool, Row, postgres::PgRow};
use std::sync::Arc;
use uuid::Uuid;

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
        let mut txn = self.dbcp.begin().await.map_err(|e| AppError::from(e.to_string()))?;

        if let Err(e) = sqlx::query(
            "INSERT INTO item_templates (id, name, description, listing_attr_templ_id) 
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET name = $2, description = $3, listing_attr_templ_id = $4",
        )
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

        if !item_templ.id.is_zero() {
            if let Err(e) = sqlx::query("DELETE FROM item_templates_attr_templates_xref WHERE item_templ_id = $1")
                .bind(item_templ.id.0)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await.map_err(|e| AppError::from(e.to_string()))?;
                log::error!(
                    "Failed to delete item template's attribute templates during its upsert: {}",
                    e
                );
                return AppResult::Err(e.to_string().into());
            }
        }

        for (index, attr_def) in item_templ.attributes.clone().iter().enumerate() {
            if let Err(e) = sqlx::query(
                "INSERT INTO item_templates_attr_templates_xref (item_templ_id, attr_templ_id, show_index) VALUES ($1, $2, $3)",
            )
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

        txn.commit().await.map_err(|e| AppError::from(e.to_string()))?;

        AppResult::Ok(())
    }

    /// Retrieve all item templates.
    pub async fn get_all(&self) -> AppResult<Vec<ItemTemplate>> {
        //
        let data = sqlx::query("SELECT * FROM item_templates ORDER BY name ASC")
            .fetch_all(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err.to_string()))
            .map(|rows| rows.iter().map(|row| from_row(row).unwrap()).collect::<Vec<ItemTemplate>>())?;
        Ok(data)
    }

    /// Delete an item template.
    pub async fn delete(&self, id: Id) -> AppResult<()> {
        //
        sqlx::query("DELETE FROM item_templates WHERE id = $1")
            .bind(id.0)
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err.to_string()))?;
        Ok(())
    }
}

fn from_row(row: &PgRow) -> Result<ItemTemplate, sqlx::Error> {
    Ok(ItemTemplate {
        id: Id(row.get::<Uuid, _>("id")),
        name: row.get("name"),
        description: row.get("description"),
        // For the purpose of this use case, we just need
        // to fill it in with minimal data that we already have.
        listing_attr: AttrTemplate {
            id: Id(row.get::<Uuid, _>("listing_attr_templ_id")),
            name: "".into(),
            description: "".into(),
            value_type: AttributeValueType::Text,
            default_value: "".into(),
            is_required: false,
        },
        attributes: Vec::new(),
    })
}
