use thiserror::{Error};

use crate::{Uuid};


#[derive(Error, Debug)]
pub enum Error {
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

    #[error("Key not found: {0}")]
    NoKey(String),

    #[error("Send failed for {0}")]
    SendFailed(String),

    #[cfg(feature = "file")]
    #[error("Canceled")]
    Canceled(#[from] futures::channel::oneshot::Canceled),

    #[error("Stopped on request")]
    Stop,

    #[cfg(feature = "postgres")]
    #[error("Postgres: {0}")]
    Postgres(#[from] crate::PostgresRepoError),

    #[cfg(feature = "file")]
    #[error("File: {0}")]
    File(#[from] crate::FileRepoError),

    #[cfg(feature = "reqwest")]
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Not Found")]
    NotFound(Uuid),

    #[error("Occupied")]
    Occupied,
}
