use crate::server::{AuthSession, ServerState, respond_not_found};
use axum::{
    Json,
    extract::{self, Path, State},
    response::IntoResponse,
};
use cogs_shared::domain::model::{
    Id,
    meta::{AttrTemplate, ItemTemplate},
};
use http::StatusCode;
use serde_json::json;

pub async fn upsert_attr_template(
    _auth_session: AuthSession,
    State(state): State<ServerState>,
    extract::Json(input): extract::Json<AttrTemplate>,
) -> impl IntoResponse {
    //
    log::debug!("[api::upsert_attr_template] input: {input:?}");
    match state.data_mgmt.upsert_attr_template(input).await {
        Ok(id) => (StatusCode::OK, Json(json!({ "id": id }))),
        Err(err) => respond_not_found(err),
    }
}

pub async fn get_all_attr_templates(_auth_session: AuthSession, State(state): State<ServerState>) -> impl IntoResponse {
    //
    match state.data_mgmt.get_all_attr_templates().await {
        Ok(attr_templs) => {
            log::info!("[api::get_all_attr_templates] Got {} entries.", attr_templs.len());
            (StatusCode::OK, Json(json!(attr_templs)))
        }
        Err(err) => respond_not_found(err),
    }
}

pub async fn delete_attr_template(
    _auth_session: AuthSession,
    State(state): State<ServerState>,
    Path(id): Path<Id>,
) -> impl IntoResponse {
    //
    match state.data_mgmt.delete_attr_template(id).await {
        Ok(()) => (StatusCode::OK, Json::default()),
        Err(err) => respond_not_found(err),
    }
}

pub async fn upsert_item_template(
    _auth_session: AuthSession,
    State(state): State<ServerState>,
    extract::Json(input): extract::Json<ItemTemplate>,
) -> impl IntoResponse {
    //
    log::debug!("[api::upsert_item_template] input: {input:?}");
    match state.data_mgmt.upsert_item_template(input).await {
        Ok(id) => (StatusCode::OK, Json(json!({ "id": id }))),
        Err(err) => respond_not_found(err),
    }
}

pub async fn get_all_item_templates(_auth_session: AuthSession, State(state): State<ServerState>) -> impl IntoResponse {
    //
    match state.data_mgmt.get_all_item_templates().await {
        Ok(attr_templs) => {
            log::info!("[api::get_all_item_templates] Got {} entries.", attr_templs.len());
            (StatusCode::OK, Json(json!(attr_templs)))
        }
        Err(err) => respond_not_found(err),
    }
}

pub async fn delete_item_template(
    _auth_session: AuthSession,
    State(state): State<ServerState>,
    Path(id): Path<Id>,
) -> impl IntoResponse {
    //
    log::info!("[api::delete_item_template] For id {id}.",);
    match state.data_mgmt.delete_item_template(id).await {
        Ok(()) => (StatusCode::ACCEPTED, Json::default()),
        Err(err) => respond_not_found(err),
    }
}
