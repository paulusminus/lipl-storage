use std::env::VarError;

use crate::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File {0:?} has invalid filestem")]
    Filestem(Option<String>),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Toml serialization error: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("Toml deserialization error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("Lyric with id {1} not found. Cannot add to playlist with id {0}")]
    PlaylistInvalidMember(String, String),

    #[error("Cannot find directory {0:?}")]
    CannotFindDirectory(Option<String>),

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

    #[error("Canceled future channel")]
    Canceled(Box<dyn std::error::Error + Send + Sync>),

    #[error("Stopped on request")]
    Stop,

    #[error("Reqwest: {0}")]
    Reqwest(Box<dyn std::error::Error + Send + Sync>),

    #[error("Not Found")]
    NotFound(Uuid),

    #[error("Occupied")]
    Occupied,

    #[error(transparent)]
    Warp(Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Axum(std::io::Error),

    #[error(transparent)]
    Json(Box<dyn std::error::Error + Send + Sync>),

    #[error("Parse error for {0}")]
    Parse(String),

    #[error("Join error for {0}")]
    Join(Box<dyn std::error::Error + Send + Sync>),

    #[error("Postgres: {0}")]
    Postgres(Box<dyn std::error::Error + Send + Sync>),

    #[error("Postgres connection: {0}")]
    Connection(Box<dyn std::error::Error + Send + Sync>),

    #[error("No results")]
    NoResults,

    #[error("Redis: {0}")]
    Redis(Box<dyn std::error::Error + Send + Sync>),

    #[error("Key: {0}")]
    Key(String),

    #[error("Environment variable: {0}")]
    Var(#[from] VarError),

    #[error("Receive error: {0}")]
    Mpsc(#[from] std::sync::mpsc::RecvError),
}

pub fn postgres_error<E>(error: E) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    Error::Postgres(Box::new(error))
}

pub fn redis_error<E>(error: E) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    Error::Redis(Box::new(error))
}

pub fn reqwest_error<E>(error: E) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    Error::Redis(Box::new(error))
}

pub trait ErrorExtension<T> {
    fn into_lipl_err(self) -> Result<T, Error>;
}

impl<T, E> ErrorExtension<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn into_lipl_err(self) -> Result<T, Error> {
        self.map_err(|e| e.into())
    }
}
