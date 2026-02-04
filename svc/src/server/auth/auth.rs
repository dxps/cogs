use crate::server::UserAccountsRepo;
use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum_session::{SessionConfig, SessionLayer, SessionMode};
use axum_session_auth::*;
use axum_session_sqlx::{SessionPgPool, SessionPgSessionStore};
use chrono::Duration;
use cogs_shared::domain::model::{Id, UserAccount};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub const SESSION_NAME: &str = "Authorization";
pub const SESSION_TABLE: &str = "user_sessions";
pub const SESSION_MAX_LIFESPAN: Duration = Duration::days(1);
pub const SESSION_CURRENT_USER_KEY: &str = "current_user";

pub async fn init_auth_layer(pg_pool: &PgPool) -> AuthSessionLayer<AuthUserAccount, Id, SessionPgPool, PgPool> {
    let auth_config = AuthConfig::<Id>::default().with_anonymous_user_id(Some(Id::default()));
    AuthSessionLayer::<AuthUserAccount, Id, SessionPgPool, PgPool>::new(Some(pg_pool.clone())).with_config(auth_config)
}

pub async fn init_session_layer(pg_pool: &PgPool) -> SessionLayer<SessionPgPool> {
    // `rest_mode` feature of axum_session is used. This disables cookies and uses the header values instead.
    // The header name used for the session id is what is configured as the session name (`with_session_name(...)`).
    let session_config = SessionConfig::default()
        .with_mode(SessionMode::OptIn)
        .with_table_name(SESSION_TABLE)
        .with_session_name(SESSION_NAME)
        .with_max_lifetime(SESSION_MAX_LIFESPAN)
        .with_purge_database_update(chrono::Duration::minutes(5));
    let session_store = SessionPgSessionStore::new(Some(pg_pool.clone().into()), session_config)
        .await
        .unwrap();
    SessionLayer::new(session_store)
}

// ---------------------------------
//          AuthUserAccount
// ---------------------------------

/// To mitigate the orphan rule, we implement the traits for UserAccount here.
/// Both `UserAccount` and `Authentication` are defined outside of this crate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUserAccount(pub UserAccount);

impl From<UserAccount> for AuthUserAccount {
    fn from(user: UserAccount) -> Self {
        AuthUserAccount(user)
    }
}

#[async_trait]
impl Authentication<AuthUserAccount, Id, PgPool> for AuthUserAccount {
    async fn load_user(user_id: Id, pool: Option<&PgPool>) -> Result<AuthUserAccount, anyhow::Error> {
        let pool = pool.unwrap();
        UserAccountsRepo::get_by_id(&user_id, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.0.is_anonymous
    }

    fn is_active(&self) -> bool {
        !self.0.is_anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.0.is_anonymous
    }
}

#[async_trait]
impl HasPermission<PgPool> for AuthUserAccount {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.0.permissions.contains(&perm.to_string())
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSession layer was not found!")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSession layer was not found!",
        )
            .into_response()
    }
}
