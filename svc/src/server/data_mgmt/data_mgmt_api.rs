use crate::server::{ServerState, respond_internal_server_error, respond_not_found};
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
    State(state): State<ServerState>,
    extract::Json(input): extract::Json<AttrTemplate>,
) -> impl IntoResponse {
    //
    log::debug!("Upserting attr template {input:?} ...");
    match state.data_mgmt.upsert_attr_template(input).await {
        Ok(id) => (StatusCode::OK, Json(json!({ "id": id }))),
        Err(err) => {
            // TODO: We should return a more specific error:
            // e.g. Either not found (if the id is non-zero) or internal server error.
            respond_internal_server_error(err)
        }
    }
}

pub async fn get_all_attr_templates(State(state): State<ServerState>) -> impl IntoResponse {
    //
    match state.data_mgmt.get_all_attr_templates().await {
        Ok(attr_templs) => {
            log::debug!("Got {} attr templates.", attr_templs.len());
            (StatusCode::OK, Json(json!(attr_templs)))
        }
        Err(err) => respond_not_found(err),
    }
}

pub async fn delete_attr_template(State(state): State<ServerState>, Path(id): Path<Id>) -> impl IntoResponse {
    //
    match state.data_mgmt.delete_attr_template(id).await {
        Ok(()) => (StatusCode::OK, Json::default()),
        Err(err) => respond_not_found(err),
    }
}

pub async fn upsert_item_template(
    State(state): State<ServerState>,
    extract::Json(input): extract::Json<ItemTemplate>,
) -> impl IntoResponse {
    //
    log::debug!("Upserting item template {input:?} ...");
    match state.data_mgmt.upsert_item_template(input).await {
        Ok(id) => (StatusCode::OK, Json(json!({ "id": id }))),
        Err(err) => match err {
            cogs_shared::app::AppError::NotFound => respond_not_found(err),
            _ => respond_internal_server_error(err),
        },
    }
}

pub async fn get_all_item_templates(State(state): State<ServerState>) -> impl IntoResponse {
    //
    match state.data_mgmt.get_all_item_templates().await {
        Ok(attr_templs) => {
            log::debug!("Got {} item templates.", attr_templs.len());
            (StatusCode::OK, Json(json!(attr_templs)))
        }
        Err(err) => respond_not_found(err),
    }
}

pub async fn delete_item_template(State(state): State<ServerState>, Path(id): Path<Id>) -> impl IntoResponse {
    //
    log::debug!("Delete item_template w/ id {id} ...",);
    match state.data_mgmt.delete_item_template(id).await {
        Ok(()) => (StatusCode::ACCEPTED, Json::default()),
        Err(err) => respond_not_found(err),
    }
}
