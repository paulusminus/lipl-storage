use thiserror::Error;

use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Postgres: {0}")]
    Postgres(#[from] lipl_postgres_repo::PostgresRepoError),

    #[error("File: {0}")]
    File(#[from] lipl_fs_repo::FileRepoError),

    #[error("Model: {0}")]
    Model(#[from] lipl_core::Error),

    #[error("Backend: {0}")]
    Backend(#[from] anyhow::Error)
}

impl Reject for RepoError {}