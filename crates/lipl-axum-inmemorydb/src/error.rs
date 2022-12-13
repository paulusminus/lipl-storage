use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,

    #[error("Occupied")]
    Occupied
}