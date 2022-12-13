use thiserror::Error;
use lipl_fs_repo::FileRepoError;
use lipl_postgres_repo::PostgresRepoError;
use lipl_core::ModelError;

use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Postgres: {0}")]
    Postgres(#[from] PostgresRepoError),

    #[error("File: {0}")]
    File(#[from] FileRepoError),

    #[error("Model: {0}")]
    Model(#[from] ModelError),
}

impl Reject for RepoError {}