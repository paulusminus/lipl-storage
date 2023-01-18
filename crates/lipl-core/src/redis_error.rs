use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisRepoError {
    #[error("Redis: {0}")]
    Redis(#[from] bb8_redis::redis::RedisError),

    #[error("Key: {0}")]
    Key(String),

    #[error("")]
    Run(#[from] bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>)
}
