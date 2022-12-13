use bb8::RunError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
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
    #[error("Postgres: {0}")]
    Postgres(#[from] tokio_postgres::Error),

    #[error("Connection: {0}")]
    Connection(#[from] RunError<tokio_postgres::Error>),
}
