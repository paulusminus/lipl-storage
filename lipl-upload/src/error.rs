#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("Hyper http error: {0}")]
    Http(#[from] hyper::http::Error),

    #[error("Invalid uri error: {0}")]
    Uri(#[from] hyper::http::uri::InvalidUri),

    #[error("Https connection error: {0}")]
    Https(&'static str),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error)
}