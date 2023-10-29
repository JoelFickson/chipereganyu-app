use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use serde_json::json;
use crate::models::groups::{Group, NewMember};


async fn create_group(Json(payload): Json<Group>) -> impl IntoResponse {
    Json(json!({
        "error": false,
        "message":"We are working on it",
        "data": payload
    }))
}

async fn add_member_to_group(Json(payload): Json<NewMember>) -> impl IntoResponse {
    let mut response = json!({
        "error": false,
        "message": "We are working on it",
        "data": {}
    });

    if let Some(id) = payload.id {
        response["data"]["id"] = json!(id);
    }

    if let Some(phone) = payload.phone {
        response["data"]["phone"] = json!(phone);
    }

    Json(response)
}

pub async fn groups_routes() -> Router {
    Router::new().nest(
        "/groups",
        Router::new()
            .route("/add_member", post(add_member_to_group))
            .route("/create_group", post(create_group)),
    )
}
