use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

//////////////
// AppError //
//////////////

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum AppError {
    //
    #[error("{0} already exists")]
    AlreadyExists(String),

    /// Commonly used to indicate that an item deletion cannot be done since
    /// it is referred (mainly at the database level through a foreign key).
    #[error("dependencies exist")]
    DependenciesExist,

    #[error("")]
    Ignorable,

    #[error("internal error")]
    InternalErr,

    /// Generic error.
    #[error("{0}")]
    Err(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("The pair of name and description must be unique.")]
    NameDescriptionNotUnique,

    #[error("not found")]
    NotFound,
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        Self::Err(s.to_string())
    }
}

impl FromStr for AppError {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <AppError as FromStr>::Err> {
        Ok(Self::from(s))
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::from(err.to_string())
    }
}

// /////////////
// AppUseCase //
////////////////

#[derive(Debug)]
pub enum AppUseCase {
    UserRegistration,
    UserLogin,
}

#[derive(Debug, Clone, Error)]
pub enum TodoAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl TodoAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            TodoAppError::NotFound => StatusCode::NOT_FOUND,
            TodoAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
