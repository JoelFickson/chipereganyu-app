
use axum::Router;
use crate::resources::groups::group_routes::groups_routes;
use crate::resources::health::health_routes::health_routes;
use crate::resources::members::member_routes::members_routes;

pub async fn merge_routes() -> Router {
    health_routes()
        .await
        .merge(members_routes().await)
        .merge(groups_routes().await)
}
