use lipl_core::{RepoConfig, Result};
use serde::Serialize;
use tokio_stream::wrappers::ReceiverStream;
use turso::{Builder, IntoParams, Row};

mod convert;
mod db;
// mod turso_connection_ext;

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

#[derive(Clone, Debug)]
pub struct TursoDatabase {
    inner: turso::Connection,
}

impl TursoDatabase {
    async fn execute(&self, sql: &'static str, params: impl IntoParams) -> Result<u64> {
        let mut statement = self.inner.prepare(sql).await.err_into()?;
        let count = statement.execute(params).await.err_into()?;
        Ok(count)
    }

    pub async fn batch_execute(&self, sql: &str) -> Result<()> {
        self.inner.execute_batch(sql).await.err_into()
    }

    async fn query<T>(
        &self,
        sql: &'static str,
        convert: fn(Row) -> Result<T>,
        params: impl IntoParams,
    ) -> Result<ReceiverStream<Result<T>>>
    where
        T: Serialize + Send + Sync + 'static,
    {
        let mut statement = self.inner.prepare(sql).await.err_into()?;
        let rows = statement.query(params).await.err_into()?;

        convert::to_list(convert)(rows)
    }

    async fn query_one<T>(
        &self,
        sql: &'static str,
        convert: fn(Row) -> Result<T>,
        params: impl IntoParams,
    ) -> Result<T>
    where
        T: Serialize,
    {
        let mut statement = self.inner.prepare(sql).await.err_into()?;
        let result = statement.query_row(params).await.err_into()?;
        convert(result)
    }

    pub async fn schema(&self) -> Result<()> {
        self.batch_execute(include_str!("create_db.sql")).await
    }

    pub async fn clear(&self) -> Result<()> {
        self.batch_execute(include_str!("delete_data_db.sql")).await
    }
}

pub struct TursoConfig {
    local: String,
}

impl From<String> for TursoConfig {
    fn from(path: String) -> Self {
        TursoConfig { local: path }
    }
}

impl RepoConfig for TursoConfig {
    type Repo = TursoDatabase;
    async fn to_repo(self) -> Result<Self::Repo> {
        Builder::new_local(&self.local)
            .build()
            .await
            .err_into()
            .and_then(|db| db.connect().map(|c| TursoDatabase { inner: c }).err_into())
    }
}
