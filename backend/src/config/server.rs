use std::net::SocketAddr;

use dotenv::dotenv;

use crate::config;
use crate::resources::routes::merge_routes;

pub async fn initialize_server() -> Result<(), anyhow::Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8777));

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();


    config::database::database::setup_database(&database_url).await?;
    let url = std::env::var("DATABASE_URL").unwrap();

    let routes = merge_routes(url)
        .await;

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
