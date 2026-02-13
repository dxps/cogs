use crate::utils::uuid_from;
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{
        Id,
        meta::{AttrTemplate, ItemTemplate, ItemTemplateLink},
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
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    /// Insert or update an item template (+ replace attributes and links xrefs).
    pub async fn upsert(&self, item_tmpl: &ItemTemplate) -> AppResult<()> {
        let mut txn = self.dbcp.begin().await.map_err(|e| AppError::from(e.to_string()))?;

        // Upsert main row
        if let Err(e) = sqlx::query(
            r#"
            INSERT INTO item_templates (id, name, description, listing_attr_templ_id)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
                SET name = EXCLUDED.name,
                    description = EXCLUDED.description,
                    listing_attr_templ_id = EXCLUDED.listing_attr_templ_id
            "#,
        )
        .bind(uuid_from(&item_tmpl.id))
        .bind(item_tmpl.name.clone())
        .bind(item_tmpl.description.clone())
        .bind(uuid_from(&item_tmpl.listing_attr.id))
        .execute(&mut *txn)
        .await
        {
            let _ = txn.rollback().await;
            log::error!("Failed to upsert item template '{}': {}", item_tmpl.id, e);
            return Err(AppError::from(e.to_string()));
        }

        // Replace attrs xref rows
        if let Err(e) = sqlx::query("DELETE FROM item_templates_attr_templates_xref WHERE item_templ_id = $1")
            .bind(uuid_from(&item_tmpl.id))
            .execute(&mut *txn)
            .await
        {
            let _ = txn.rollback().await;
            log::error!(
                "Failed to clear item template attribute links during upsert '{}': {}",
                item_tmpl.id,
                e
            );
            return Err(AppError::from(e.to_string()));
        }

        for (index, attr_def) in item_tmpl.attributes.iter().enumerate() {
            if let Err(e) = sqlx::query(
                r#"
                INSERT INTO item_templates_attr_templates_xref (item_templ_id, attr_templ_id, show_index)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(uuid_from(&item_tmpl.id))
            .bind(uuid_from(&attr_def.id))
            .bind((index + 1) as i16)
            .execute(&mut *txn)
            .await
            {
                let _ = txn.rollback().await;
                log::error!(
                    "Failed to insert item template attribute link '{}', attr '{}': {}",
                    item_tmpl.id,
                    attr_def.id,
                    e
                );
                return Err(AppError::from(e.to_string()));
            }
        }

        // Replace link rows
        if let Err(e) = sqlx::query("DELETE FROM item_template_links WHERE item_templ_id = $1")
            .bind(uuid_from(&item_tmpl.id))
            .execute(&mut *txn)
            .await
        {
            let _ = txn.rollback().await;
            log::error!("Failed to clear item template links during upsert '{}': {}", item_tmpl.id, e);
            return Err(AppError::from(e.to_string()));
        }

        for (index, link) in item_tmpl.links.iter().enumerate() {
            if let Err(e) = sqlx::query(
                r#"
                INSERT INTO item_template_links (item_templ_id, link_name, target_item_templ_id, show_index)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(uuid_from(&item_tmpl.id))
            .bind(link.name.clone())
            .bind(uuid_from(&link.item_template_id))
            .bind((index + 1) as i16)
            .execute(&mut *txn)
            .await
            {
                let _ = txn.rollback().await;
                log::error!(
                    "Failed to insert item template link '{}': {} -> {} ({})",
                    item_tmpl.id,
                    link.name,
                    link.item_template_id,
                    e
                );
                return Err(AppError::from(e.to_string()));
            }
        }

        txn.commit().await.map_err(|e| AppError::from(e.to_string()))?;

        Ok(())
    }

    /// Retrieve all item templates with attributes and links.
    pub async fn get_all(&self) -> AppResult<Vec<ItemTemplate>> {
        // 1) Base + attrs
        let attr_rows = sqlx::query(
            r#"
            SELECT
                it.id                           AS it_id,
                it.name                         AS it_name,
                it.description                  AS it_description,

                lat.id                          AS lat_id,
                lat.name                        AS lat_name,
                lat.description                 AS lat_description,
                lat.value_type                  AS lat_value_type,
                lat.default_value               AS lat_default_value,
                lat.required                    AS lat_required,

                x.show_index                    AS x_show_index,
                at.id                           AS at_id,
                at.name                         AS at_name,
                at.description                  AS at_description,
                at.value_type                   AS at_value_type,
                at.default_value                AS at_default_value,
                at.required                     AS at_required
            FROM item_templates it
            INNER JOIN attr_templates lat                  ON lat.id = it.listing_attr_templ_id
            LEFT JOIN item_templates_attr_templates_xref x ON x.item_templ_id = it.id
            LEFT JOIN attr_templates at                    ON at.id = x.attr_templ_id
            ORDER BY it.name ASC, x.show_index ASC
            "#,
        )
        .fetch_all(self.dbcp.as_ref())
        .await
        .map_err(|e| AppError::from(e.to_string()))?;

        let mut grouped: HashMap<Uuid, ItemTemplate> = HashMap::new();

        for row in attr_rows {
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
                links: vec![],
            });

            let at_id: Option<Uuid> = row.try_get("at_id").ok();
            if let Some(attr_id) = at_id {
                entry.attributes.push(AttrTemplate {
                    id: Id::from(attr_id.to_string()),
                    name: row.try_get::<String, _>("at_name").unwrap_or_default(),
                    description: row.try_get::<String, _>("at_description").unwrap_or_default(),
                    value_type: row.get::<String, _>("at_value_type").into(),
                    default_value: row.get("at_default_value"),
                    // FIX: was at_is_mandatory in your code; column is at_required
                    is_required: row.try_get::<bool, _>("at_required").unwrap_or(false),
                });
            }
        }

        // 2) Links
        let link_rows = sqlx::query(
            r#"
            SELECT
                l.item_templ_id         AS item_templ_id,
                l.link_name             AS link_name,
                l.target_item_templ_id  AS target_item_templ_id,
                l.show_index            AS show_index
            FROM item_template_links l
            ORDER BY l.item_templ_id, l.show_index
            "#,
        )
        .fetch_all(self.dbcp.as_ref())
        .await
        .map_err(|e| AppError::from(e.to_string()))?;

        for row in link_rows {
            let owner_id: Uuid = row.try_get("item_templ_id").map_err(|e| AppError::from(e.to_string()))?;

            if let Some(entry) = grouped.get_mut(&owner_id) {
                let target_id: Uuid = row
                    .try_get("target_item_templ_id")
                    .map_err(|e| AppError::from(e.to_string()))?;

                let name: String = row.try_get("link_name").unwrap_or_default();

                entry.links.push(ItemTemplateLink {
                    name,
                    item_template_id: Id::from(target_id.to_string()),
                });
            }
        }

        let mut data: Vec<ItemTemplate> = grouped.into_values().collect();
        data.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(data)
    }

    /// Delete an item template.
    pub async fn delete(&self, id: Id) -> AppResult<()> {
        sqlx::query("DELETE FROM item_templates WHERE id = $1")
            .bind(uuid_from(&id))
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err.to_string()))?;
        Ok(())
    }
}
