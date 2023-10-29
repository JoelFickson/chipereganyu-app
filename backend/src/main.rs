mod config;
mod models;
mod resources;


use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {

    
    let DATABASE_URL = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = sea_orm::Database::connect(&DATABASE_URL).await?;
    Migrator::up(&connection, None).await?;

    config::server::initialize_server().await;
}
