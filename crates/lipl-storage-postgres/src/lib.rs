use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use futures_util::TryFutureExt;
use lipl_core::{
    Error, Lyric, ObjectStream, Playlist, Repo, RepoConfig, Result, Summary, postgres_error,
};
use serde::Serialize;
use tokio_postgres::{
    NoTls, Row,
    types::{ToSql, Type},
};

use crate::{
    db::{lyric, playlist},
    pg_stream::{PgStream, objects},
};

mod convert;
mod db;
mod pg_stream;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub const CREATE_DB: &str = include_str!("create_db.sql");

#[derive(Clone)]
pub struct PostgresConnectionPool {
    inner: ConnectionPool,
}

impl From<ConnectionPool> for PostgresConnectionPool {
    fn from(pool: ConnectionPool) -> Self {
        Self { inner: pool }
    }
}

impl PostgresConnectionPool {
    async fn execute<'a>(
        &'a self,
        sql: &'static str,
        types: &'a [Type],
        params: &'a [&(dyn ToSql + Sync)],
    ) -> Result<u64> {
        let connection = self.inner.get().await.map_err(postgres_error)?;
        let statement = connection
            .prepare_typed(sql, types)
            .await
            .map_err(postgres_error)?;
        let count = connection
            .execute(&statement, params)
            .await
            .map_err(postgres_error)?;
        Ok(count)
    }

    async fn batch_execute(&self, sql: &str) -> Result<()> {
        let connection = self.inner.get().await.map_err(postgres_error)?;
        connection.batch_execute(sql).map_err(postgres_error).await
    }

    async fn query_stream<O>(
        &self,
        sql: &'static str,
        convert: fn(Row) -> Result<O>,
    ) -> Result<PgStream<O>> {
        let connection = self.inner.get().await.map_err(postgres_error)?;
        connection
            .query_raw(sql, &[] as &[&str])
            .await
            .map_err(postgres_error)
            .map(objects(convert))
    }

    async fn query<'a, F, T>(
        &'a self,
        sql: &'static str,
        types: &'static [Type],
        convert: F,
        params: &'a [&(dyn ToSql + Sync)],
    ) -> Result<Vec<T>>
    where
        F: Fn(Row) -> Result<T> + Copy + 'a,
        T: Serialize,
    {
        let connection = self.inner.get().await.map_err(postgres_error)?;
        let statement = connection
            .prepare_typed(sql, types)
            .await
            .map_err(postgres_error)?;
        let rows = connection
            .query(&statement, params)
            .await
            .map_err(postgres_error)?;
        convert::to_list(convert)(rows)
    }

    async fn query_one<'a, F, T>(
        &'a self,
        sql: &'static str,
        types: &'a [Type],
        convert: F,
        params: &'a [&(dyn ToSql + Sync)],
    ) -> Result<T>
    where
        F: Fn(Row) -> Result<T> + Copy + 'a,
        T: Serialize,
    {
        let connection = self.inner.get().await.map_err(postgres_error)?;
        let statement = connection
            .prepare_typed(sql, types)
            .await
            .map_err(postgres_error)?;
        match connection
            .query_opt(&statement, params)
            .await
            .map_err(postgres_error)?
        {
            Some(row) => convert(row),
            _ => Err(Error::NoResults),
        }
    }
}

pub struct PostgresConfig {
    connection_string: String,
}

impl From<String> for PostgresConfig {
    fn from(connection_string: String) -> Self {
        PostgresConfig { connection_string }
    }
}

impl RepoConfig for PostgresConfig {
    type Repo = PostgresConnectionPool;
    async fn to_repo(self) -> Result<Self::Repo> {
        connection_pool(&self.connection_string).await
    }
}

pub async fn connection_pool(connection: &str) -> Result<PostgresConnectionPool> {
    let manager = PostgresConnectionManager::new_from_stringlike(connection, NoTls)
        .map_err(postgres_error)?;

    let pool = Pool::builder()
        .build(manager)
        .await
        .map_err(postgres_error)
        .unwrap();
    let postgres_connection_pool = PostgresConnectionPool::from(pool);
    tracing::info!("About to execute database creation script");
    postgres_connection_pool
        .batch_execute(CREATE_DB)
        .await
        .unwrap();
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

impl ObjectStream for PostgresConnectionPool {
    type Stream<O> = PgStream<O>;

    async fn lyric_summaries(&self) -> Result<Self::Stream<Summary>> {
        self.query_stream(lyric::LIST, convert::to_summary).await
    }

    async fn lyrics(&self) -> Result<Self::Stream<Lyric>> {
        self.query_stream(lyric::LIST_FULL, convert::to_lyric).await
    }

    async fn playlist_summaries(&self) -> Result<Self::Stream<Summary>> {
        self.query_stream(playlist::LIST, convert::to_summary).await
    }

    async fn playlists(&self) -> Result<Self::Stream<Playlist>> {
        self.query_stream(playlist::LIST_FULL, convert::to_playlist)
            .await
    }
}
