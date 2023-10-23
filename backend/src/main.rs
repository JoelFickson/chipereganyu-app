mod config {
    pub mod server;
}

mod resources {

    pub mod routes;
    pub mod health{
         pub mod health_service;
     }
}


#[tokio::main]
async fn main() {
    config::server::initialize_server().await;
}
