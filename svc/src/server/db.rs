use crate::{
    AppResult,
    app_err_uc::{AppError, AppUseCase},
};
use sqlx::{PgPool, postgres::PgPoolOptions};

static DB_POOL: std::sync::OnceLock<PgPool> = std::sync::OnceLock::new();

pub async fn db_pool_init() -> AppResult<PgPool> {
    //
    let db_url = std::env::var("DATABASE_URL").map_err(|err| {
        log::error!(
            "DATABASE_URL environment variable is not set. Reason: '{}'.",
            err
        );
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

// ////////////////////////////////////////////////////////
// Implementations of sqlx's FromRow trait for AppError. //
// ////////////////////////////////////////////////////////

impl From<sqlx::Error> for AppError {
    //
    fn from(err: sqlx::Error) -> Self {
        //
        let mut app_err = AppError::Ignorable;
        log::debug!("from(sqlx:Error): err={:?}", err);
        if err.as_database_error().is_some() {
            // FYI: For now, any db error is considered as internal error.
            app_err = AppError::InternalErr
        } else if let sqlx::Error::RowNotFound = err {
            app_err = AppError::NotFound
        }
        app_err
    }
}

impl From<(sqlx::Error, AppUseCase)> for AppError {
    //
    fn from(ctx: (sqlx::Error, AppUseCase)) -> Self {
        //
        let err = &ctx.0;
        let uc = &ctx.1;
        match uc {
            AppUseCase::UserRegistration => match &err.as_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        // 23505 is postgres specific code for duplicate entry (aka "unique_violation").
                        // See: https://www.postgresql.org/docs/16/errcodes-appendix.html.
                        "23505" => AppError::AlreadyExists("".into()),
                        _ => log_and_return_internal_err(ctx),
                    },
                    None => log_and_return_internal_err(ctx),
                },
                None => log_and_return_internal_err(ctx),
            },

            AppUseCase::UserLogin => match &err {
                sqlx::Error::RowNotFound => AppError::Unauthorized("wrong credentials".into()),
                _ => log_and_return_internal_err(ctx),
            },
        }
    }
}

fn log_and_return_internal_err(ctx: (sqlx::Error, AppUseCase)) -> AppError {
    log::debug!(
        "InternalErr due to sql err={:?} on usecase:{:?}.",
        ctx.0,
        ctx.1
    );
    AppError::InternalErr
}

impl From<(sqlx::Error, AppUseCase, String)> for AppError {
    //
    fn from(ctx: (sqlx::Error, AppUseCase, String)) -> Self {
        //
        let err = &ctx.0;
        let uc_info = &ctx.2;
        match ctx.1 {
            AppUseCase::UserRegistration => match &err.as_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::AlreadyExists(uc_info.clone()),
                        _ => log_and_return_internal_err_ext(ctx),
                    },
                    None => log_and_return_internal_err_ext(ctx),
                },
                None => log_and_return_internal_err_ext(ctx),
            },
            AppUseCase::UserLogin => match &err {
                sqlx::Error::RowNotFound => AppError::Unauthorized("wrong credentials".into()),
                _ => log_and_return_internal_err_ext(ctx),
            },
        }
    }
}

fn log_and_return_internal_err_ext(ctx: (sqlx::Error, AppUseCase, String)) -> AppError {
    log::debug!(
        "InternalErr due to sql err={:?} on usecase:{:?} and info:'{}'.",
        ctx.0,
        ctx.1,
        ctx.2
    );
    AppError::InternalErr
}
