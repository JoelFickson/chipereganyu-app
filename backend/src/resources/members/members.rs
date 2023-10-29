use axum::{
    routing::{get, post},
    Router,
};
use log::debug;

pub struct User {
    id: String,
    name: String,
    phone: String,
    photo: String,
    password: String,
}
struct NewUser {
    name: String,
    phone: String,
    photo: String,
    password: String,
}

struct LoginInfo {
    phone: String,
    password: String,
}

async fn create_user_account(Json(payload): Json<NewUser>) -> impl IntoResponse {
    debug(payload);
    Json(json!({ "error": false, "message":"We are working on it..." }))
}
async fn login_user(Json(payload): Json<LoginInfo>) -> impl IntoResponse {
    debug(payload);
    Json(json!({ "error": false, "message":"We are working on it..." }))
}

pub async fn export_routes() -> Router {
    Router::new()
        .route("/register", post(create_user_account))
        .route("/login", post(login_user));
}
