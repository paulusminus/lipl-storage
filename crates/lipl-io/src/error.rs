use std::path::PathBuf;

use lipl_core::{reexport::UUIDError, ModelError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Yaml: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("UUID: {0}")]
    UUID(#[from] UUIDError),

    #[error("Model: {0}")]
    Model(#[from] ModelError),

    #[error("Non existing directory {0}")]
    NonExistingDirectory(PathBuf),

    #[error("No key {0}")]
    NoKey(String),

    #[error("No path {0}")]
    NoPath(PathBuf),
}