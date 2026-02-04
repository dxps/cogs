use cogs_shared::{app::AppError, domain::model::Id};
use uuid::Uuid;

pub fn new_id() -> Id {
    Id(Uuid::now_v7())
}

pub fn new_app_error_from_sqlx(err: sqlx::Error, msg: Option<String>) -> AppError {
    log::trace!("mapping AppError from sqlx err={:?}", err);

    let msg = match &err {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => {
            // FYI: For now, any specifically unhandled error is considered as internal error.
            if let Some(db_err) = err.as_database_error() {
                if let Some(code) = db_err.code() {
                    if code.as_ref() == "23505" {
                        return AppError::AlreadyExists(msg.unwrap_or_default());
                    }
                }
            }
            AppError::InternalErr
        }
    };

    AppError::from(msg)
}
