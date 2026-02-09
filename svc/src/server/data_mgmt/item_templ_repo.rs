use crate::utils::uuid_from;
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{
        Id,
        meta::{AttrTemplate, ItemTemplate},
    },
};
use sqlx::{PgPool, Row};
use std::{collections::HashMap, sync::Arc};
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
    pub async fn upsert(&self, item_tmpl: &ItemTemplate) -> AppResult<()> {
        let mut txn = self.dbcp.begin().await.map_err(|e| AppError::from(e.to_string()))?;

        if let Err(e) = sqlx::query(
            "INSERT INTO item_templates (id, name, description, listing_attr_templ_id) 
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET name = $2, description = $3, listing_attr_templ_id = $4",
        )
        .bind(uuid_from(&item_tmpl.id))
        .bind(item_tmpl.name.clone())
        .bind(item_tmpl.description.clone())
        .bind(uuid_from(&item_tmpl.listing_attr.id))
        .execute(&mut *txn)
        .await
        {
            txn.rollback().await.map_err(|e| AppError::from(e.to_string()))?;
            log::error!("Failed to add item template: '{}'.", e);
            return Err(AppError::from(e.to_string()));
        }

        if !item_tmpl.id.is_zero() {
            if let Err(e) = sqlx::query("DELETE FROM item_templates_attr_templates_xref WHERE item_templ_id = $1")
                .bind(uuid_from(&item_tmpl.id))
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await.map_err(|e| AppError::from(e.to_string()))?;
                log::error!(
                    "Failed to delete item template's attribute templates during its upsert: {}",
                    e
                );
                return Err(e.to_string().into());
            }
        }

        for (index, attr_def) in item_tmpl.attributes.iter().enumerate() {
            if let Err(e) = sqlx::query(
                "INSERT INTO item_templates_attr_templates_xref (item_templ_id, attr_templ_id, show_index) VALUES ($1, $2, $3)",
            )
            .bind(uuid_from(&item_tmpl.id))
            .bind(uuid_from(&attr_def.id))
            .bind((index + 1) as i16)
            .execute(&mut *txn)
            .await
            {
                txn.rollback().await.map_err(|e| AppError::from(e.to_string()))?;
                log::error!("Failed to add item template's attribute templates: {}", e);
                return Err(e.to_string().into());
            }
        }

        txn.commit().await.map_err(|e| AppError::from(e.to_string()))?;
        Ok(())
    }

    /// Retrieve all item templates with their attribute templates.
    pub async fn get_all(&self) -> AppResult<Vec<ItemTemplate>> {
        let rows = sqlx::query(
            r#"
            SELECT
                it.id                           AS it_id,
                it.name                         AS it_name,
                it.description                  AS it_description,
                it.listing_attr_templ_id        AS it_listing_attr_templ_id,

                -- listing attribute template
                lat.id                          AS lat_id,
                lat.name                        AS lat_name,
                lat.description                 AS lat_description,
                lat.value_type                  AS lat_value_type,
                lat.default_value               AS lat_default_value,
                lat.required                    AS lat_required,

                -- regular attributes (nullable because LEFT JOIN)
                x.show_index                    AS x_show_index,
                at.id                           AS at_id,
                at.name                         AS at_name,
                at.description                  AS at_description,
                at.value_type                   AS at_value_type,
                at.default_value                AS at_default_value,
                at.required                     AS at_required
            FROM item_templates it
            INNER JOIN attr_templates lat                    ON lat.id = it.listing_attr_templ_id
            LEFT JOIN item_templates_attr_templates_xref x   ON x.item_templ_id = it.id
            LEFT JOIN attr_templates at                      ON at.id = x.attr_templ_id
            ORDER BY it.name ASC, x.show_index ASC
            "#,
        )
        .fetch_all(self.dbcp.as_ref())
        .await
        .map_err(|e| AppError::from(e.to_string()))?;

        // group by item_template id
        let mut grouped: HashMap<Uuid, ItemTemplate> = HashMap::new();

        for row in rows {
            let it_id: Uuid = row.try_get("it_id").map_err(|e| AppError::from(e.to_string()))?;

            let entry = grouped.entry(it_id).or_insert_with(|| ItemTemplate {
                id: Id::from(it_id.to_string()),
                name: row.try_get::<String, _>("it_name").unwrap_or_default(),
                description: row.try_get::<String, _>("it_description").unwrap_or_default(),
                listing_attr: AttrTemplate {
                    id: Id::from(row.try_get::<Uuid, _>("lat_id").unwrap_or_default().to_string()),
                    name: row.try_get::<String, _>("lat_name").unwrap_or_default(),
                    description: row.try_get::<String, _>("lat_description").unwrap_or_default(),
                    value_type: row.get::<String, _>("lat_value_type").into(),
                    default_value: row.get("lat_default_value"),
                    is_required: row.try_get::<bool, _>("lat_required").unwrap_or(false),
                },
                attributes: vec![],
            });

            // if joined regular attr exists, append in show_index order
            let at_id: Option<Uuid> = row.try_get("at_id").ok();
            if let Some(attr_id) = at_id {
                entry.attributes.push(AttrTemplate {
                    id: Id::from(attr_id.to_string()),
                    name: row.try_get::<String, _>("at_name").unwrap_or_default(),
                    description: row.try_get::<String, _>("at_description").unwrap_or_default(),
                    value_type: row.get::<String, _>("at_value_type").into(),
                    default_value: row.get("at_default_value"),
                    is_required: row.try_get::<bool, _>("at_is_mandatory").unwrap_or(false),
                });
            }
        }

        let mut data: Vec<ItemTemplate> = grouped.into_values().collect();
        data.sort_by(|a, b| a.name.cmp(&b.name)); // stable output
        Ok(data)
    }

    /// Delete an item template.
    pub async fn delete(&self, id: Id) -> AppResult<()> {
        //
        sqlx::query("DELETE FROM item_templates WHERE id = $1")
            .bind(uuid_from(&id))
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err.to_string()))?;
        Ok(())
    }
}
