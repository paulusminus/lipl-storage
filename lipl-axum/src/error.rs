use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct ErrorReport {
    error: String,
}

impl<E: std::error::Error> From<E> for ErrorReport {
    fn from(error: E) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Hyper: {0}")]
    Hyper(#[from] hyper::Error),

    #[cfg(feature = "postgres")]
    #[error("Postgres: {0}")]
    Postgres(#[from] lipl_axum_postgres::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorReport::from(self)),
        )
            .into_response()
    }
}
