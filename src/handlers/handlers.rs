use crate::config::config::Config;
use crate::error::error::GenericErrors;
use crate::models::url::UrlRequestDTO;
use axum::Json;
use axum::extract::State;
use nanoid::nanoid;
use serde_json::{Value, json};

pub async fn shorten_url(
    State(state): State<Config>,
    Json(payload): Json<UrlRequestDTO>,
) -> Result<Json<Value>, GenericErrors> {
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

    let val = json!({
        "url": txt,
    });

    Ok(Json(val))
}
