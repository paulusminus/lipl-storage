use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("UUID: {0}")]
    Uuid(#[from] lipl_core::reexport::uuid::Error),

    #[error("Model: {0}")]
    Model(#[from] lipl_core::ModelError),

    #[error("Non existing directory {0}")]
    NonExistingDirectory(PathBuf),

    #[error("No key {0}")]
    NoKey(String),

    #[error("No path {0}")]
    NoPath(PathBuf),
}