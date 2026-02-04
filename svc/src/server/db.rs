use cogs_shared::app::{AppError, AppResult};
use sqlx::{PgPool, postgres::PgPoolOptions};

static DB_POOL: std::sync::OnceLock<PgPool> = std::sync::OnceLock::new();

pub async fn db_pool_init() -> AppResult<PgPool> {
    //
    let db_url = std::env::var("DATABASE_URL").map_err(|err| {
        log::error!("DATABASE_URL environment variable is not set. Reason: '{}'.", err);
        AppError::Err("DATABASE_URL environment variable is not set.".into())
    })?;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(db_url.as_str())
        .await
        .map_err(|_| AppError::Err("Failed to connect to database".into()))?;

    DB_POOL.set(pool.clone()).unwrap();
    Ok(pool)
}

pub fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().expect("db pool is not initialized")
}
