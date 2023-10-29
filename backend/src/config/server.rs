use std::net::SocketAddr;
use crate::resources::routes::merge_routes;


pub async fn initialize_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8777));

    let routes = merge_routes().await;

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
