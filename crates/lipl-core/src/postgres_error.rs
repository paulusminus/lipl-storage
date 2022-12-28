use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostgresRepoError {
    #[error("Postgres: {0}")]
    Postgres(#[from] tokio_postgres::Error),

    #[error("Uuid: {0}")]
    Uuid(#[from] uuid::Error),

    // #[error("Pool: {0}")]
    // Pool(#[from] bb8_postgres::PoolError),

    // #[error("Pool build: {0}")]
    // PoolBuild(#[from] bb8_postgres::BuildError),

    #[error("Connection: {0}")]
    Connection(#[from] bb8_postgres::bb8::RunError<tokio_postgres::Error>),
}