use axum::{routing::post, Router, Json, response::IntoResponse};
use serde_json::json;
use crate::models::user::{NewUser, LoginInfo};




async fn create_user_account(Json(payload): Json<NewUser>) -> impl IntoResponse {
    println!("{:?} user data.", payload);
    Json(json!({
        "error": false,
        "message":"We are working on it...",
         "data": payload
    }))
}

async fn login_user(Json(payload): Json<LoginInfo>) -> impl IntoResponse {
    println!("{:?}.", payload);
    Json(json!({
        "error": false,
        "message":"We are working on it...",
         "data": payload
    }))
}

pub async fn members_routes() -> Router {
    Router::new()
        .nest("/user",
              Router::new()
            .route("/register", post(create_user_account))
            .route("/login", post(login_user)),
        )
}
