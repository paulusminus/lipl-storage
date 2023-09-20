use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use futures_util::{Future, TryFutureExt};
use lipl_core::{postgres_error, Error, LiplRepo, Result};
use serde::Serialize;
use tokio_postgres::{NoTls, types::{Type, ToSql}, Row};

mod convert;
mod db;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub const CREATE_DB: &str = include_str!("create_db.sql");

#[derive(Clone)]
pub struct PostgresConnectionPool {
    inner: ConnectionPool,
}

impl From<ConnectionPool> for PostgresConnectionPool {
    fn from(pool: ConnectionPool) -> Self {
        Self {
            inner: pool,
        }
    }
}

impl PostgresConnectionPool {
    fn execute<'a>(
        &'a self,
        sql: &'static str,
        types: &'a[Type],
        params: &'a[&(dyn ToSql + Sync)],
    ) -> impl Future<Output = Result<u64>> + 'a
    {
        async move {
            let connection = self.inner.get().await.map_err(postgres_error)?;
            let statement = connection.prepare_typed(sql, types).await.map_err(postgres_error)?;
            let count = connection.execute(&statement, params).await.map_err(postgres_error)?;
            Ok(count)
        }
    }

    async fn batch_execute(&self, sql: &str) -> Result<()> {
        let connection = self.inner.get().await.map_err(postgres_error)?;
        connection.batch_execute(sql).map_err(postgres_error).await
    }

    fn query<'a, F, T>(
        &'a self,
        sql: &'static str,
        types: &'static[Type],
        convert: F,
        params: &'a[&(dyn ToSql + Sync)]
    ) -> impl Future<Output = Result<Vec<T>>> + 'a
    where F: Fn(Row) -> Result<T> + Copy + 'a, T: Serialize,
    {
        async move {
            let connection = self.inner.get().await.map_err(postgres_error)?;
            let statement = connection.prepare_typed(sql, types).await.map_err(postgres_error)?;
            let rows = connection.query(&statement, params).await.map_err(postgres_error)?;
            convert::to_list(convert)(rows)
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
            let connection = self.inner.get().await.map_err(postgres_error)?;
            let statement = connection.prepare_typed(sql, types).await.map_err(postgres_error)?;
            if let Some(row) = connection.query_opt(&statement, params).await.map_err(postgres_error)? {
                convert(row)
            }
            else {
                Err(Error::NoResults)
            }
        }
    }
}

pub async fn connection_pool(connection: &str) -> Result<PostgresConnectionPool> {
    let manager = PostgresConnectionManager::new_from_stringlike(connection, NoTls).map_err(postgres_error)?;
    let pool = Pool::builder().build(manager).await.map_err(postgres_error)?;
    
    let postgres_connection_pool = PostgresConnectionPool::from(pool);
    tracing::info!("About to execute database creation script");
    postgres_connection_pool.batch_execute(CREATE_DB).await?;
    tracing::info!("Finished executing database creation script");

    tracing::info!("Warm up cache");
    
    if let Err(error) = postgres_connection_pool.get_lyrics().await {
        tracing::error!("Failed to get lyrics for warming up cache: {}", error);
    }

    if let Err(error) = postgres_connection_pool.get_playlists().await {
        tracing::error!("Failed to get playlists for warming up cache: {}", error);
    }

    tracing::info!("Warm up cache finished");

    Ok(postgres_connection_pool)
}
