use bb8_postgres::bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use futures_util::{Future, TryFutureExt};
use lipl_core::{LyricDb, PlaylistDb};
use serde::Serialize;
use tokio_postgres::{NoTls, types::{Type, ToSql}, Row};

// pub use crate::error::Error;

mod convert;
// mod error;
mod lyric;
mod playlist;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;
type Result<T> = std::result::Result<T, lipl_core::PostgresRepoError>;

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
        params: &'a[&(dyn ToSql + Sync)]
    ) -> impl Future<Output = Result<()>> + 'a
    {
        async move {
            let connection = self.inner.get().await?;
            let statement = connection.prepare_typed(sql, types).await?;
            connection.execute(&statement, params).await?;
            Ok(())
        }
    }

    async fn batch_execute(&self, sql: &str) -> Result<()> {
        let connection = self.inner.get().await?;
        connection.batch_execute(sql).err_into().await
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
            let connection = self.inner.get().await?;
            let statement = connection.prepare_typed(sql, types).await?;
            let rows = connection.query(&statement, params).await?;
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
            let connection = self.inner.get().await?;
            let statement = connection.prepare_typed(sql, types).await?;
            let row = connection.query_one(&statement, params).await?;
            convert(row)
        }
    }
}

pub async fn connection_pool(connection: &'static str) -> Result<PostgresConnectionPool> {
    let manager = PostgresConnectionManager::new_from_stringlike(connection, NoTls)?;
    let pool = Pool::builder().build(manager).await?;
    
    let postgres_connection_pool = PostgresConnectionPool::from(pool);
    tracing::info!("About to execute database creation script");
    postgres_connection_pool.batch_execute(CREATE_DB).await?;
    tracing::info!("Finished executing database creation script");

    tracing::info!("Warm up cache");
    
    if let Err(error) = postgres_connection_pool.lyric_list().await {
        tracing::error!("Failed to get lyrics for warming up cache: {}", error);
    }

    if let Err(error) = postgres_connection_pool.playlist_list().await {
        tracing::error!("Failed to get playlists for warming up cache: {}", error);
    }

    tracing::info!("Warm up cache finished");

    Ok(postgres_connection_pool)
}
