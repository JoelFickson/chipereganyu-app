use std::net::SocketAddr;

use crate::resources::routes;

pub async fn initialize_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8777));

    axum::Server::bind(&addr)
        .serve(routes::merge_routes().into_make_service())
        .await
        .unwrap();
}
