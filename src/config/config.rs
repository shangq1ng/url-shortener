use crate::error::error::GenericErrors;
use anyhow::{Context, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub db: PgPool,
    pub port: u16,
}

impl Config {
    pub async fn new() -> Result<Config, GenericErrors> {
        let db_url = env::var("DATABASE_URL").context("DATABASE_URL not set")?;
        let port = env::var("PORT")?
            .parse::<u16>()
            .context("PORT is not a number")?;

        let db = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .context("Failed to connect to database")?;

        Ok(Self { db, port })
    }
}
