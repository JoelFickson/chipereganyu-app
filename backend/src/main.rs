mod config;


mod models;
mod resources;

#[tokio::main]
async fn main() {
    config::server::initialize_server().await;
}
