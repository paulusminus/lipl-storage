use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bb8::RunError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_postgres::Error as PgError;

#[derive(Deserialize, Serialize)]
struct ErrorReport {
    error: String,
}

impl From<Error> for ErrorReport {
    fn from(error: Error) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Connection: {0}")]
    Connection(#[from] RunError<PgError>),

    #[error("Postgres: {0}")]
    Postgres(#[from] PgError),

    #[error("Hyper: {0}")]
    Hyper(#[from] hyper::Error),
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
