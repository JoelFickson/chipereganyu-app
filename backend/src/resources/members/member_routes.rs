use axum::{Extension, Json, response::IntoResponse, Router, routing::post};
use sea_orm::DatabaseConnection;
use serde_json::json;

use crate::models::user::{LoginInfo, NewUser};
use crate::resources::members::members_service::members_service::create_user;

async fn create_user_account(Extension(db): Extension<DatabaseConnection>, Json(payload): Json<NewUser>) -> impl IntoResponse {
    println!("{:?} user data.", payload);


    let user = NewUser {
        phone: payload.phone,
        name: payload.name,
        password: payload.password,
        photo: None,
    };

    create_user(db, user).await.expect("TODO: panic message");

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
