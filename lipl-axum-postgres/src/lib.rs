use async_trait::async_trait;
use axum_core::{extract::{FromRequestParts, FromRef}};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use http::request::Parts;
use tokio_postgres::NoTls;

pub use crate::error::Error;

mod convert;
mod error;
mod ext;
mod lyric;
mod playlist;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct PostgresConnection<'a> {
    inner: PooledConnection<'a, PostgresConnectionManager<NoTls>>
}

impl<'a> PostgresConnection<'a> {
    pub fn new(pool: PooledConnection<'a, PostgresConnectionManager<NoTls>>) -> Self {
        Self {
            inner: pool
        }
    }
}

pub async fn connection_pool(connection: &'static str) -> Result<ConnectionPool> {
    let manager = PostgresConnectionManager::new_from_stringlike(connection, NoTls)?;
    let pool = Pool::builder().build(manager).await?;
    Ok(pool)
}

#[async_trait]
impl<'a, S> FromRequestParts<S> for PostgresConnection<'a>
where
    ConnectionPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> std::result::Result<Self, Self::Rejection> {
        ConnectionPool::from_ref(state)
            .get_owned()
            .await
            .map_err(Error::from)
            .map(Self::new)
    }
}
