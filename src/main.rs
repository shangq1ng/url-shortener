pub mod config;
pub mod error;
pub mod handlers;
pub mod models;

use crate::config::config::Config;
use anyhow::Error;
use axum::Router;
use axum::routing::post;
use dotenvy::dotenv;
use tracing::{debug, error};
use crate::handlers::handlers::shorten_url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match dotenv() {
        Ok(_) => debug!("Successfully loaded .env!"),
        Err(e) => error!("Error loading .env!: {}", e),
    }
    let config = Config::new().await?;
    let listener = tokio::net::TcpListener::bind(("localhost", config.port)).await?;
    debug!("Listening on port {}", listener.local_addr()?);

    let app: Router = Router::new()
        .route("/shorten", post(shorten_url))
        .with_state(config);

    axum::serve(listener, app).await?;

    Ok(())
}
