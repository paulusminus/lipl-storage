// use async_trait::async_trait;
// use axum_core::extract::{FromRef, FromRequestParts};
use bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use futures_util::{TryFutureExt, Future};
// use http::request::Parts;
use serde::Serialize;
use tokio_postgres::{NoTls, types::{Type, ToSql}, Row};

pub use crate::error::Error;

mod convert;
mod error;
mod lyric;
mod playlist;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;
pub type Result<T> = std::result::Result<T, Error>;

pub const CREATE_DB: &str = include_str!("create_db.sql");

pub struct PostgresConnection {
    inner: Pool<PostgresConnectionManager<NoTls>>,
}

impl PostgresConnection {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { inner: pool }
    }

    fn execute<'a>(
        &'a self,
        sql: &'static str,
        types: &'a[Type],
        params: &'a[&(dyn ToSql + Sync)]
    ) -> impl Future<Output = Result<()>> + 'a
    {
        async move {
            let connection = &self.inner.get().await?;
            connection
                .prepare_typed(sql, types)
                .and_then(|statement| async move { connection.execute(&statement, params).await })
                .map_err(Error::from)
                .await
                .map(|_| ())
        }
    }

    fn query<'a, F, T>(
        &'a self,
        sql: &'static str,
        types: &'a[Type],
        convert: F,
        params: &'a[&(dyn ToSql + Sync)]
    ) -> impl Future<Output = Result<Vec<T>>> + 'a
    where F: Fn(Row) -> Result<T> + Copy + 'a, T: Serialize,
    {
        async move {
            let connection = &self.inner.get().await?;
            connection
                .prepare_typed(sql, types)
                .and_then(|statement| async move { 
                    connection.query(&statement, params).await
                })
                .map_err(Error::from)
                .await
                .and_then(convert::to_list(convert))
        }
    }

    fn query_one<'a, F, T>(
        &'a self,
        sql: &'static str,
        types: &'a[Type],
        convert: F,
        params: &'a[&(dyn ToSql + Sync)]
    ) -> impl Future<Output = Result<T>> + 'a
    where F: Fn(Row) -> Result<T> + Copy + 'a, T: Serialize,
    {
        async move {
            let connection = &self.inner.get().await?;
            connection
                .prepare_typed(sql, types)
                .and_then(|statement| async move { connection.query_one(&statement, params).await })
                .map_err(Error::from)
                .await
                .and_then(convert)
        }
    }
}

pub async fn connection_pool(connection: &'static str) -> Result<ConnectionPool> {
    let manager = PostgresConnectionManager::new_from_stringlike(connection, NoTls)?;
    let pool = Pool::builder().build(manager).await?;
    
    tracing::info!("About to execute database creation script");
    pool.get().await?.batch_execute(CREATE_DB).await?;
    tracing::info!("Finished executing database creation script");

    Ok(pool)
}

// #[async_trait]
// impl<'a, S> FromRequestParts<S> for PostgresConnection<'a>
// where
//     ConnectionPool: FromRef<S>,
//     S: Send + Sync,
// {
//     type Rejection = Error;

//     async fn from_request_parts(
//         _parts: &mut Parts,
//         state: &S,
//     ) -> Result<Self> {
//         ConnectionPool::from_ref(state)
//             .get_owned()
//             .await
//             .map_err(Error::from)
//             .map(Self::new)
//     }
// }
