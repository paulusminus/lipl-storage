use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Http: {0}")]
    Http(#[from] hyper::http::Error),

    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Uri: {0}")]
    Uri(#[from] hyper::http::uri::InvalidUri),
}