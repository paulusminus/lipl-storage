use std::{env::VarError, str::ParseBoolError};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
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

    #[error("Environment: {0}")]
    Environment(#[from] VarError),

    #[error("Invalid configuration from environment")]
    InvalidConfiguration,

    #[error("Environment parse: {0}")]
    EnvironmentParse(#[from] ParseBoolError),

    #[error("Core: {0}")]
    Core(#[from] lipl_core::Error),
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
