use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::env::VarError;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum GenericErrors {
    #[error("Content-Type Not Allowed")]
    ContentTypeNotAllowed,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("Unsupported Url")]
    UnsupportedUrl,
    #[error("Conversion Error")]
    ConversionError(#[from] VarError),
    #[error("Not Found")]
    NotFound,
    #[error("Conflict")]
    Conflict,
    #[error("Database Error")]
    DatabaseError,
    #[error("Config Error")]
    ConfigError(#[from] anyhow::Error),
}

impl IntoResponse for GenericErrors {
    fn into_response(self) -> Response {
        let current_status = match self {
            Self::ContentTypeNotAllowed => StatusCode::NOT_ACCEPTABLE,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::ConfigError(e) => {
                error!("Config error {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Conflict => StatusCode::CONFLICT,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnsupportedUrl => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        current_status.into_response()
    }
}

impl From<sqlx::Error> for GenericErrors {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(e) if e.is_unique_violation() => GenericErrors::Conflict,
            sqlx::Error::RowNotFound => GenericErrors::NotFound,
            sqlx::Error::ColumnNotFound(e) => {
                error!("Column not found {}", e);
                GenericErrors::NotFound
            }
            _ => GenericErrors::DatabaseError,
        }
    }
}
