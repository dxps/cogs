use crate::server::{UserAccountsRepo, UserMgmt};
use axum::extract::{FromRef, FromRequestParts};
use http::{StatusCode, request::Parts};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ServerState {
    pub user_mgmt: Arc<UserMgmt>,
}

impl ServerState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        //
        let user_mgmt = Arc::new(UserMgmt::new(Arc::new(UserAccountsRepo::new(
            db_pool.clone(),
        ))));
        Self { user_mgmt }
    }
}

impl<S> FromRequestParts<S> for ServerState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
