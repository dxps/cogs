use axum::{
    Json,
    extract::{self, State},
    response::IntoResponse,
};
use cogs_shared::dtos::LoginRequest;
use http::StatusCode;
use serde_json::json;

use crate::{
    AppError,
    server::{AuthSession, ServerState, respond_internal_server_error, respond_unauthorized},
};

pub async fn login_user(
    auth_session: AuthSession,
    State(state): State<ServerState>,
    extract::Json(input): extract::Json<LoginRequest>,
) -> impl IntoResponse {
    //
    let res = state
        .user_mgmt
        .authenticate_user(input.username, input.password)
        .await;
    match res.error {
        None => {
            let account = res.account.unwrap();
            auth_session.login_user(account.id.clone());
            (StatusCode::OK, Json(json!(account)))
        }
        Some(err) => match err {
            AppError::Unauthorized(_) => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
