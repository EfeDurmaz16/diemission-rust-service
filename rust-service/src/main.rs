mod config;
mod error;
mod models;
mod node_client;
mod pdf;
mod routes;

use std::net::SocketAddr;

use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{
    config::Config,
    node_client::NodeApiClient,
    routes::{router, AppState},
};

#[tokio::main]
async fn main() {
    // Load .env first so RUST_LOG from it reaches the subscriber below.
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let config = Config::from_env();
    let node = NodeApiClient::new(&config).expect("failed to build Node API client");
    let app = router(AppState { node });

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!(
        %addr,
        node_api = %config.node_api_base_url,
        "rust-service listening"
    );

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind port");

    axum::serve(listener, app).await.expect("server error");
}
