use thiserror::Error;
use serde_yaml::Error as YamlError;
use std::io::Error as IOError;
use zip::result::ZipError;
use uuid::Error as UuidError;
use bs58::decode::Error as DecodeError;
use bs58::encode::Error as EncodeError;

pub type LiplResult<T> = Result<T, LiplError>;

#[derive(Error, Debug)]
pub enum LiplError {

    #[error("Yaml error: {0}")]
    Yaml(#[from] YamlError),

    #[error("IO error: {0}")]
    IO (#[from] IOError),

    #[error("Zip error: {0}")]
    Zip (#[from] ZipError),

    #[error("Uuid error: {0}")]
    Uuid (#[from] UuidError),

    #[error("Encode error: {0}")]
    Encode (#[from] EncodeError),

    #[error("Decode error: {0}")]
    Decode (#[from] DecodeError),

    #[error("No Path error: {0}")]
    NoPath(String),

    #[error("Argument error: {0}")]
    Argument(String),
}
