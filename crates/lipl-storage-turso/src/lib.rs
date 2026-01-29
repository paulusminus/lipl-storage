use lipl_core::{RepoConfig, Result, postgres_error};
use serde::Serialize;
use tokio_stream::wrappers::ReceiverStream;
use turso::{Builder, IntoParams, Row};

mod convert;
mod db;

pub const CREATE_DB: &str = include_str!("create_db.sql");

#[derive(Clone)]
pub struct TursoDatabase {
    inner: turso::Connection,
}

impl TursoDatabase {
    async fn execute(&self, sql: &'static str, params: impl IntoParams) -> Result<u64> {
        let mut statement = self.inner.prepare(sql).await.map_err(postgres_error)?;
        let count = statement.execute(params).await.map_err(postgres_error)?;
        Ok(count)
    }

    pub async fn batch_execute(&self, sql: &str) -> Result<()> {
        self.inner.execute_batch(sql).await.map_err(postgres_error)
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
        let mut statement = self.inner.prepare(sql).await.map_err(postgres_error)?;
        let rows = statement.query(params).await.map_err(postgres_error)?;

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
        let mut statement = self.inner.prepare(sql).await.map_err(postgres_error)?;
        let result = statement.query_row(params).await.map_err(postgres_error)?;
        convert(result)
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
        let db = Builder::new_local(&self.local)
            .build()
            .await
            .map_err(postgres_error)?;
        db.connect()
            .map(|c| TursoDatabase { inner: c })
            .map_err(postgres_error)
    }
}

#[cfg(test)]
mod test {
    use super::TursoConfig;
    use lipl_core::{Repo, RepoConfig};

    pub const TEST_DATABASE_NAME: &str =
        "/home/paul/Code/rust/lipl-storage/crates/lipl-storage-turso/data/lipl.sqlite";

    #[tokio::test]
    async fn create_database() {
        let config = TursoConfig::from(TEST_DATABASE_NAME.to_owned());
        let repo = config.to_repo().await.unwrap();
        repo.batch_execute(include_str!("create_db.sql"))
            .await
            .unwrap();

        // Test query functionality
    }

    #[tokio::test]
    async fn copy_memory_database() {
        let memory_repo = lipl_storage_memory::MemoryRepoConfig {
            sample_data: true,
            transaction_log: None,
        }
        .to_repo()
        .await
        .unwrap();

        let turso_repo = TursoConfig::from(TEST_DATABASE_NAME.to_owned())
            .to_repo()
            .await
            .unwrap();

        // Clear existing data
        turso_repo
            .batch_execute(include_str!("delete_data_db.sql"))
            .await
            .unwrap();

        // Copy data from memory to Turso
        for lyric in memory_repo.get_lyrics().await.unwrap() {
            dbg!(&lyric);
            turso_repo.upsert_lyric(lyric).await.unwrap();
        }

        for playlist in memory_repo.get_playlists().await.unwrap() {
            dbg!(&playlist);
            turso_repo.upsert_playlist(playlist).await.unwrap();
        }
    }
}
