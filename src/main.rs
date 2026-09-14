pub mod config;
pub mod error;
pub mod handlers;
pub mod models;

use crate::config::config::Config;
use anyhow::Error;
use axum::Router;
use axum::routing::{get, post};
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tracing::{debug, error};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::handlers::handlers::{redirect, shorten_url};

#[tokio::main]
async fn main() -> Result<(), Error> {
    match dotenv() {
        Ok(_) => debug!("Successfully loaded .env!"),
        Err(e) => error!("Error loading .env!: {}", e),
    }
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,tower_http=debug,tower_http=debug",env!("CARGO_CRATE_NAME")).into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new().await?;
    let listener = tokio::net::TcpListener::bind(("localhost", config.port)).await?;
    debug!("Listening on port {}", listener.local_addr()?);

    let app: Router = Router::new()
        .route("/shorten", post(shorten_url))
        .route("/{code}", get(redirect))
        .with_state(config)
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, app).await?;

    Ok(())
}
