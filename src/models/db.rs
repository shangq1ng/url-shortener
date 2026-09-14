use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
pub struct Db {
    pub id: i32,
    pub short_code: String,
    pub original_url: String,
    pub created_at: Option<NaiveDateTime>,
}