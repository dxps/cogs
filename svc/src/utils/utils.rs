use cogs_shared::{app::AppError, domain::model::Id};
use uuid::Uuid;

pub fn new_id() -> Id {
    Id(Uuid::now_v7().to_string())
}

pub fn uuid_from(id: &Id) -> Uuid {
    if let Ok(uuid) = Uuid::parse_str(&id.0) {
        return uuid;
    } else {
        log::error!("Cannot parse id '{}' as uuid.", id.0);
        Uuid::default()
    }
}

pub fn new_app_error_from_sqlx(err: sqlx::Error, msg: Option<String>) -> AppError {
    //
    let mut aerr = AppError::InternalErr;
    match &err {
        sqlx::Error::RowNotFound => aerr = AppError::NotFound,
        _ => {
            // FYI: For now, any specifically unhandled error is considered as internal error.
            if let Some(db_err) = err.as_database_error() {
                if let Some(code) = db_err.code() {
                    if code.as_ref() == "23505" {
                        aerr = AppError::AlreadyExists(msg.unwrap_or_default());
                    }
                }
            }
        }
    };
    if !matches!(aerr, AppError::AlreadyExists(_)) {
        log::debug!("Mapped sqlx err={:?} to AppError={:?}", err, aerr);
    }
    aerr
}
