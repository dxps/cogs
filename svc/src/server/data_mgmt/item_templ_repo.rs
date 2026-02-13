use crate::utils::uuid_from;
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{
        Id,
        meta::{AttrTemplate, ItemTemplate, ItemTemplateLink},
    },
};
use sqlx::PgPool;
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

        sqlx::query!(
            r#"
            INSERT INTO item_templates (id, name, description, listing_attr_templ_id)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
                SET name = EXCLUDED.name,
                    description = EXCLUDED.description,
                    listing_attr_templ_id = EXCLUDED.listing_attr_templ_id
            "#,
            uuid_from(&item_tmpl.id),
            item_tmpl.name,
            item_tmpl.description,
            uuid_from(&item_tmpl.listing_attr.id),
        )
        .execute(&mut *txn)
        .await
        .map_err(|e| AppError::from(e.to_string()))?;

        sqlx::query!(
            r#"DELETE FROM item_templates_attr_templates_xref WHERE item_templ_id = $1"#,
            uuid_from(&item_tmpl.id),
        )
        .execute(&mut *txn)
        .await
        .map_err(|e| AppError::from(e.to_string()))?;

        for (index, attr) in item_tmpl.attributes.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO item_templates_attr_templates_xref
                    (item_templ_id, attr_templ_id, show_index)
                VALUES
                    ($1, $2, $3)
                "#,
                uuid_from(&item_tmpl.id),
                uuid_from(&attr.id),
                (index as i16) + 1,
            )
            .execute(&mut *txn)
            .await
            .map_err(|e| AppError::from(e.to_string()))?;
        }

        sqlx::query!(
            r#"DELETE FROM item_template_links WHERE item_templ_id = $1"#,
            uuid_from(&item_tmpl.id),
        )
        .execute(&mut *txn)
        .await
        .map_err(|e| AppError::from(e.to_string()))?;

        for (index, link) in item_tmpl.links.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO item_template_links
                    (item_templ_id, link_name, target_item_templ_id, show_index)
                VALUES
                    ($1, $2, $3, $4)
                "#,
                uuid_from(&item_tmpl.id),
                link.name,
                uuid_from(&link.item_template_id),
                (index as i16) + 1,
            )
            .execute(&mut *txn)
            .await
            .map_err(|e| AppError::from(e.to_string()))?;
        }

        txn.commit().await.map_err(|e| AppError::from(e.to_string()))?;

        Ok(())
    }

    pub async fn get_all(&self) -> AppResult<Vec<ItemTemplate>> {
        let attr_rows = sqlx::query_as!(
            ItemTemplateAttrRow,
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

                at.id                           AS at_id,
                at.name                         AS at_name,
                at.description                  AS at_description,
                at.value_type                   AS at_value_type,
                at.default_value                AS at_default_value,
                at.required                     AS at_required
            FROM item_templates it
            INNER JOIN attr_templates lat
                ON lat.id = it.listing_attr_templ_id
            LEFT JOIN item_templates_attr_templates_xref x
                ON x.item_templ_id = it.id
            LEFT JOIN attr_templates at
                ON at.id = x.attr_templ_id
            ORDER BY it.name ASC, x.show_index ASC
            "#
        )
        .fetch_all(self.dbcp.as_ref())
        .await
        .map_err(|e| AppError::from(e.to_string()))?;

        let mut grouped: HashMap<Uuid, ItemTemplate> = HashMap::new();

        for row in attr_rows {
            let entry = grouped.entry(row.it_id).or_insert_with(|| ItemTemplate {
                id: Id::from(row.it_id.to_string()),
                name: row.it_name.clone(),
                description: row.it_description.unwrap_or_default(),
                listing_attr: AttrTemplate {
                    id: Id::from(row.lat_id.to_string()),
                    name: row.lat_name.clone(),
                    description: row.lat_description.unwrap_or_default(),
                    value_type: row.lat_value_type.clone().into(),
                    default_value: row.lat_default_value.clone().unwrap_or_default(),
                    is_required: row.lat_required.unwrap_or(false),
                },
                attributes: vec![],
                links: vec![],
            });

            if let Some(attr_id) = row.at_id {
                entry.attributes.push(AttrTemplate {
                    id: Id::from(attr_id.to_string()),
                    name: row.at_name.unwrap_or_default(),
                    description: row.at_description.unwrap_or_default(),
                    value_type: row.at_value_type.unwrap_or_default().into(),
                    default_value: row.at_default_value.unwrap_or_default(),
                    is_required: row.at_required.unwrap_or(false),
                });
            }
        }

        let link_rows = sqlx::query_as!(
            ItemTemplateLinkRow,
            r#"
            SELECT
                l.item_templ_id         AS item_templ_id,
                l.link_name             AS link_name,
                l.target_item_templ_id  AS target_item_templ_id
            FROM item_template_links l
            ORDER BY l.item_templ_id, l.show_index
            "#
        )
        .fetch_all(self.dbcp.as_ref())
        .await
        .map_err(|e| AppError::from(e.to_string()))?;

        for row in link_rows {
            if let Some(entry) = grouped.get_mut(&row.item_templ_id) {
                entry.links.push(ItemTemplateLink {
                    name: row.link_name,
                    item_template_id: Id::from(row.target_item_templ_id.to_string()),
                });
            }
        }

        let mut data: Vec<ItemTemplate> = grouped.into_values().collect();
        data.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(data)
    }

    pub async fn delete(&self, id: Id) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM item_templates WHERE id = $1"#, uuid_from(&id),)
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|e| AppError::from(e.to_string()))?;
        Ok(())
    }
}

#[derive(Debug)]
struct ItemTemplateAttrRow {
    it_id: Uuid,
    it_name: String,
    it_description: Option<String>,

    lat_id: Uuid,
    lat_name: String,
    lat_description: Option<String>,
    lat_value_type: String,
    lat_default_value: Option<String>,
    lat_required: Option<bool>,

    at_id: Option<Uuid>,
    at_name: Option<String>,
    at_description: Option<String>,
    at_value_type: Option<String>,
    at_default_value: Option<String>,
    at_required: Option<bool>,
}

#[derive(Debug)]
struct ItemTemplateLinkRow {
    item_templ_id: Uuid,
    link_name: String,
    target_item_templ_id: Uuid,
}
