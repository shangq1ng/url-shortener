use crate::config::config::Config;
use crate::error::error::GenericErrors;
use crate::models::db::Db;
use crate::models::url::{UrlShortenRequestDTO, UrlShortenResponseDTO};
use axum::Json;
use axum::extract::{Path, State};
use axum::response::Redirect;
use nanoid::nanoid;

pub async fn shorten_url(
    State(state): State<Config>,
    Json(payload): Json<UrlShortenRequestDTO>,
) -> Result<Json<UrlShortenResponseDTO>, GenericErrors> {
    let code = nanoid!(5);
    sqlx::query_as!(
        Db,
        "INSERT INTO links (short_code, original_url)
        VALUES ($1, $2)",
        code,
        payload.url,
    )
    .execute(&state.db)
    .await?;

    let txt = format!("http://localhost:{}/{}", state.port, code);
    Ok(Json(UrlShortenResponseDTO { short_code: txt }))
}

pub async fn redirect(
    State(state): State<Config>,
    Path(payload): Path<String>,
) -> Result<Redirect, GenericErrors> {
    let query = sqlx::query_as!(
        Db,
        "SELECT id, short_code, original_url, created_at FROM links WHERE short_code = $1",
        payload,
    )
    .fetch_optional(&state.db)
    .await?;

    match query {
        Some(v) => Ok(Redirect::to(&v.original_url)),
        None => Err(GenericErrors::NotFound),
    }
}
