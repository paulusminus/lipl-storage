use futures::channel::oneshot::Canceled;
use thiserror::{Error};

pub type ModelResult<T> = core::result::Result<T, ModelError>;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("File {0:?} has invalid filestem")]
    Filestem(Option<String>),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Yaml Error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Lyric with id {1} not found. Cannot add to playlist with id {0}")]
    PlaylistInvalidMember(String, String),

    #[error("Cannot find directory {0:?}")]
    CannotFindDirectory(Option<String>),

    #[error("Bincode serialization failed: {0}")]
    BincodeError(#[from] Box<bincode::ErrorKind>),

    #[error("Decode error: {0}")]
    Bs58DecodeError(#[from] bs58::decode::Error),

    #[error("Ebcode error: {0}")]
    Bs58EncodeError(#[from] bs58::encode::Error),

    #[error("Uuid error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("No Path error: {0}")]
    NoPath(std::path::PathBuf),

    #[error("Argument error: {0}")]
    Argument(&'static str),

    #[error("Directory does not exist: {0}")]
    NonExistingDirectory(std::path::PathBuf),

    #[error("No Path error: {0}")]
    NoKey(String),

    #[error("Send failed for {0}")]
    SendFailed(String),

    #[error("Canceled")]
    Canceled(#[from] Canceled),

    #[error("Stopped on request")]
    Stop,
}