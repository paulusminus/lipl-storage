use futures_util::TryFutureExt;
use lipl_core::{Lyric, ObjectsStream, Playlist, RepoConfig, Result, Summary};
use turso::{Builder, IntoParams, Row};

use crate::{
    db::{lyric, playlist},
    object_stream::{ObjectStream, objects},
};

mod convert;
mod db;
mod object_stream;
mod row_stream;

pub const CREATE_DB: &str = include_str!("create_db.sql");

trait ErrInto<T> {
    fn err_into(self) -> Result<T>;
}

impl<T, E> ErrInto<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn err_into(self) -> Result<T> {
        self.map_err(|e| lipl_core::Error::Turso(Box::new(e)))
    }
}

trait OkInto<E, T> {
    fn ok_into(self) -> std::result::Result<T, E>;
}

impl<E, S, T> OkInto<E, T> for std::result::Result<S, E>
where
    S: Into<T>,
{
    fn ok_into(self) -> std::result::Result<T, E> {
        self.map(Into::into)
    }
}

#[derive(Clone, Debug)]
pub struct TursoDatabase {
    inner: turso::Connection,
}

impl From<turso::Connection> for TursoDatabase {
    fn from(inner: turso::Connection) -> Self {
        Self { inner }
    }
}

impl TursoDatabase {
    async fn execute(&self, sql: &'static str, params: impl IntoParams) -> Result<u64> {
        self.inner
            .prepare(sql)
            .and_then(|mut statement| async move { statement.execute(params).await })
            .await
            .err_into()
    }

    pub async fn batch_execute(&self, sql: &str) -> Result<()> {
        self.inner.execute_batch(sql).await.err_into()
    }

    async fn query<T>(
        &self,
        sql: &'static str,
        convert: fn(Row) -> Result<T>,
        params: impl IntoParams,
    ) -> Result<ObjectStream<T>> {
        self.inner
            .prepare(sql)
            .and_then(|mut statement| async move { statement.query(params).await })
            .await
            .err_into()
            .map(objects(convert))
    }

    async fn query_one<T>(
        &self,
        sql: &'static str,
        convert: fn(Row) -> Result<T>,
        params: impl IntoParams,
    ) -> Result<T> {
        self.inner
            .prepare(sql)
            .and_then(|mut statement| async move { statement.query_row(params).await })
            .await
            .err_into()
            .and_then(convert)
    }

    pub async fn schema(&self) -> Result<()> {
        self.batch_execute(include_str!("create_db.sql")).await
    }

    pub async fn clear(&self) -> Result<()> {
        self.batch_execute(include_str!("delete_data_db.sql")).await
    }
}

pub struct TursoConfig {
    path: String,
}

impl From<String> for TursoConfig {
    fn from(path: String) -> Self {
        Self { path }
    }
}

impl RepoConfig for TursoConfig {
    type Repo = TursoDatabase;
    async fn to_repo(self) -> Result<Self::Repo> {
        fn connect(db: turso::Database) -> turso::Result<turso::Connection> {
            db.connect()
        }

        Builder::new_local(&self.path)
            .experimental_materialized_views(true)
            .build()
            .await
            .and_then(connect)
            .ok_into()
            .err_into()
    }
}

impl ObjectsStream for TursoDatabase {
    type Stream<O> = ObjectStream<O>;
    async fn lyric_summaries(&self) -> Result<Self::Stream<Summary>> {
        self.query(lyric::LIST, convert::to_summary, ()).await
    }

    async fn lyrics(&self) -> Result<Self::Stream<Lyric>> {
        self.query(lyric::LIST_FULL, convert::to_lyric, ()).await
    }

    async fn playlists(&self) -> Result<Self::Stream<Playlist>> {
        self.query(playlist::LIST_FULL, convert::to_playlist, ())
            .await
    }

    async fn playlist_summaries(&self) -> Result<Self::Stream<Summary>> {
        self.query(playlist::LIST, convert::to_summary, ()).await
    }
}
