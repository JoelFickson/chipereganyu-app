use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

async fn health_check() -> impl IntoResponse {
    Json(json!({ "error": false, "message":"Backend service is up and running" }))
}

pub async fn health_routes() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
}
