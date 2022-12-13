use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostgresRepoError {
    #[error("Postgres: {0}")]
    Postgres(#[from] tokio_postgres::Error),

    #[error("Uuid: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Pool: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),

    #[error("Pool build: {0}")]
    PoolBuild(#[from] deadpool_postgres::BuildError),
}