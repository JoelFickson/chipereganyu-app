mod config;
mod models;
mod resources;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    config::server::initialize_server().await;
    config::database::database::setup_database().await?;

    Ok(())
}
