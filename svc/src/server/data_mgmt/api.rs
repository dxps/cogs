use axum::{
    Json,
    extract::{self, State},
    response::IntoResponse,
};
use cogs_shared::domain::model::meta::AttrTemplate;
use http::StatusCode;
use serde_json::json;

use crate::server::{AuthSession, ServerState, respond_not_found};

pub async fn upsert_attr_templ(
    _auth_session: AuthSession,
    State(state): State<ServerState>,
    extract::Json(input): extract::Json<AttrTemplate>,
) -> impl IntoResponse {
    //
    log::debug!("upsert_attr_templ: {input:#?}");
    match state.data_mgmt.upsert_attr_templ(input).await {
        Ok(id) => (StatusCode::OK, Json(json!({ "id": id }))),
        Err(err) => respond_not_found(err),
    }
}

pub async fn get_all_attr_templ(
    _auth_session: AuthSession,
    State(state): State<ServerState>,
) -> impl IntoResponse {
    //
    match state.data_mgmt.get_all_attr_templ().await {
        Ok(attr_templs) => (StatusCode::OK, Json(json!(attr_templs))),
        Err(err) => respond_not_found(err),
    }
}
