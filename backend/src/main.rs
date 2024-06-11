mod config;
mod models;
mod resources;

pub mod utils;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    config::server::initialize_server().await?;

    println!("Welcome to the money saving app");

    Ok(())
}
