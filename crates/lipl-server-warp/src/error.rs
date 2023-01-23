use thiserror::Error;

use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum RepoError {
    #[cfg(feature = "postgres")]
    #[error("Postgres: {0}")]
    Postgres(#[from] lipl_repo_postgres::PostgresRepoError),

    #[cfg(feature = "file")]
    #[error("File: {0}")]
    File(#[from] lipl_repo_fs::FileRepoError),

    #[error("Model: {0}")]
    Model(#[from] lipl_core::Error),

    // #[error("Backend: {0}")]
    // Backend(#[from] anyhow::Error),

    #[error("Warp: {0}")]
    Warp(#[from] warp::Error),
}

impl Reject for RepoError {}