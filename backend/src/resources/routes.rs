use axum::{routing::get, Router};
use crate::resources::health::health_service;
pub fn merge_routes() -> Router {
    Router::new()
        .route("/", get(health_service::health_check))
        .route("/health", get(health_service::health_check))
}
