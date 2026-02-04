use crate::server::{AuthUserAccount, SESSION_CURRENT_USER_KEY, SESSION_MAX_LIFESPAN, ServerState};
use axum::{Json, extract::State};
use axum_session::Session;
use axum_session_sqlx::SessionPgPool;
use cogs_shared::{
    app::AppError,
    dtos::{ErrorResponse, LoginRequest, LoginResponse},
};
use http::StatusCode;
use log::debug;

pub async fn login(
    State(state): State<ServerState>,
    session: Session<SessionPgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<ErrorResponse>)> {
    //
    debug!("[login] Received '{:?}'.", payload);

    let user_account = state
        .user_mgmt
        .authenticate_user(payload.username, payload.password)
        .await
        .map_err(|err| match err {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, Json(msg.into())),
            _ => {
                debug!("Login error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Internal server error".to_owned(),
                    }),
                )
            }
        })?;

    session.set_store(true);
    session.set(SESSION_CURRENT_USER_KEY, AuthUserAccount::from(user_account.clone()));

    let response = LoginResponse {
        session: session.get_session_id(),
        expires_in_seconds: SESSION_MAX_LIFESPAN.num_seconds(),
        user: Some(user_account),
        error: None,
    };
    Ok((StatusCode::OK, Json(response)))
}
