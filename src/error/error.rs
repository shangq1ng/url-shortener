use std::env::VarError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;
#[derive(Debug, Error)]
pub enum GenericErrors {
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
            Self::ConfigError(e) => {
                error!("Config error {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            },
            Self::Conflict => StatusCode::CONFLICT,
            Self::NotFound => StatusCode::NOT_FOUND,
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
           },
           _ => GenericErrors::DatabaseError,
       }
    }
}