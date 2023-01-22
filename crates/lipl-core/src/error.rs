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
    Postgres(#[from] PostgresRepoError),

    #[cfg(feature = "file")]
    #[error("File: {0}")]
    File(#[from] FileRepoError),

    #[cfg(feature = "reqwest")]
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[cfg(feature = "redis")]
    #[error("Redis: {0}")]
    Redis(#[from] RedisRepoError),

    #[error("Not Found")]
    NotFound(Uuid),

    #[error("Occupied")]
    Occupied,

    #[error(transparent)]
    Warp(Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Axum(Box<dyn std::error::Error + Send + Sync>),
}

#[cfg(feature = "file")]
#[derive(Error, Debug)]
pub enum FileRepoError {
    #[error("File {0:?} has invalid filestem")]
    Filestem(Option<String>),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Lyric with id {1} not found. Cannot add to playlist with id {0}")]
    PlaylistInvalidMember(String, String),

    #[error("Cannot find directory {0:?}")]
    CannotFindDirectory(Option<String>),

    #[error("Send failed")]
    SendFailed,

    #[error("Canceled")]
    Canceled(#[from] futures::channel::oneshot::Canceled),

    #[error("Parse error for {0}")]
    Parse(String),

    #[error("Join error for {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("No Path: {0}")]
    NoPath(String),
}

#[cfg(feature = "file")]
impl Default for FileRepoError {
    fn default() -> Self {
        FileRepoError::Parse("Hallo".to_owned())
    }
}

#[cfg(feature = "postgres")]
#[derive(Debug, Error)]
pub enum PostgresRepoError {
    #[error("Postgres: {0}")]
    Postgres(#[from] bb8_postgres::tokio_postgres::Error),

    #[error("Uuid: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Connection: {0}")]
    Connection(#[from] bb8_postgres::bb8::RunError<bb8_postgres::tokio_postgres::Error>),

    #[error("No results")]
    NoResults,
}

#[cfg(feature = "redis")]
#[derive(Debug, Error)]
pub enum RedisRepoError {
    #[error("Redis: {0}")]
    Redis(#[from] bb8_redis::redis::RedisError),

    #[error("Key: {0}")]
    Key(String),

    #[error("")]
    Run(#[from] bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>)
}
