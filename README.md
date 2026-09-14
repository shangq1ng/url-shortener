# url-shortener

A URL shortener built in Rust.

## Stack
- axum
- tower-http
- dotenvy
- tokio
- sqlx (postgres)
- thiserror
- anyhow
- nanoid

## How to run
- `cp .env.example .env`
- Insert your credentials in .env 
- `docker compose up -d`
- `cargo run` 