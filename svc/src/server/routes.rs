use crate::server::{
    ServerState, delete_attr_template, delete_item_template, get_all_attr_templates, get_all_item_templates, login_user,
    upsert_attr_template, upsert_item_template,
};
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub fn create_router(state: ServerState) -> Router {
    //
    let tracing_layer = TraceLayer::new_for_http();
    let cors_layer = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    Router::new()
        // .route("/api/healthcheck", get(health_check))
        .route("/api/login", post(login_user))
        .route("/api/attribute_templates", post(upsert_attr_template))
        .route("/api/attribute_templates", get(get_all_attr_templates))
        .route("/api/attribute_templates/{id}/delete", post(delete_attr_template))
        .route("/api/item_templates", post(upsert_item_template))
        .route("/api/item_templates", get(get_all_item_templates))
        .route("/api/item_templates/{id}/delete", post(delete_item_template))
        .layer(tracing_layer)
        .layer(cors_layer)
        .with_state(state)
}
