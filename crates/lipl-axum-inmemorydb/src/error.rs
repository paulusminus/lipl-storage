use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,

    #[error("Occupied")]
    Occupied,

    #[error("Yaml: {0}")]
    Yaml(#[from] lipl_core::reexport::serde_yaml::Error),
}