use crate::utils::{new_app_error_from_sqlx, uuid_from};
use cogs_shared::{
    app::{AppError, AppResult},
    domain::model::{AccessLevel, Id},
};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AccessLevelRepo {
    dbcp: Arc<PgPool>,
}

impl AccessLevelRepo {
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_all(&self) -> AppResult<Vec<AccessLevel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description
            FROM access_levels
            ORDER BY name ASC
            "#,
        )
        .fetch_all(self.dbcp.as_ref())
        .await
        .map_err(|err| new_app_error_from_sqlx(err, Some("failed to get access levels".to_string())))?;

        Ok(rows
            .into_iter()
            .map(|row| AccessLevel {
                id: Id::from(row.get::<Uuid, _>("id").to_string()),
                name: row.get("name"),
                description: row.get("description"),
            })
            .collect())
    }

    pub async fn insert(&self, access_level: &AccessLevel) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO access_levels (id, name, description)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(uuid_from(&access_level.id))
        .bind(&access_level.name)
        .bind(&access_level.description)
        .execute(self.dbcp.as_ref())
        .await
        .map_err(|err| new_app_error_from_sqlx(err, Some(access_level.name.clone())))?;

        Ok(())
    }

    pub async fn update(&self, access_level: &AccessLevel) -> AppResult<()> {
        self.ensure_not_read_only(&access_level.id).await?;

        let result = sqlx::query(
            r#"
            UPDATE access_levels
            SET name = $2,
                description = $3
            WHERE id = $1
            "#,
        )
        .bind(uuid_from(&access_level.id))
        .bind(&access_level.name)
        .bind(&access_level.description)
        .execute(self.dbcp.as_ref())
        .await
        .map_err(|err| new_app_error_from_sqlx(err, Some(access_level.name.clone())))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    pub async fn delete(&self, id: Id) -> AppResult<()> {
        self.ensure_not_read_only(&id).await?;

        let result = sqlx::query(r#"DELETE FROM access_levels WHERE id = $1"#)
            .bind(uuid_from(&id))
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| new_app_error_from_sqlx(err, Some("failed to delete access level".to_string())))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    async fn ensure_not_read_only(&self, id: &Id) -> AppResult<()> {
        let row = sqlx::query(
            r#"
            SELECT name, read_only
            FROM access_levels
            WHERE id = $1
            "#,
        )
        .bind(uuid_from(id))
        .fetch_optional(self.dbcp.as_ref())
        .await
        .map_err(|err| new_app_error_from_sqlx(err, Some("failed to get access level".to_string())))?;

        let Some(row) = row else {
            return Err(AppError::NotFound);
        };

        if row.get::<bool, _>("read_only") {
            return Err(AppError::ReadOnly(row.get("name")));
        }

        Ok(())
    }
}
