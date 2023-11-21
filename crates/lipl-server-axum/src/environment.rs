use std::sync::Arc;

/// Configure repo from environment

use crate::{Result, Error, ToRepo};

pub enum RepoType {
    #[cfg(feature = "postgres")]
    Postgres(String),
    #[cfg(feature = "memory")]
    Memory(bool),
    #[cfg(feature = "fs")]
    Fs(String),
}

#[async_trait::async_trait]
impl ToRepo for RepoType {
    async fn to_repo(self) ->  lipl_core::Result<Arc<dyn lipl_core::LiplRepo>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(connection) => {
                let pool = lipl_storage_postgres_axum::connection_pool(&connection).await?;
                Ok(Arc::new(pool))
            }
            #[cfg(feature = "memory")]
            Self::Memory(include_sample) => {
                lipl_storage_memory::MemoryRepoConfig { sample_data: include_sample, transaction_log: None}.to_repo().await
            }
            #[cfg(feature = "fs")]
            Self::Fs(data_dir) => {
                lipl_storage_fs::FileRepoConfig { path: data_dir}.to_repo().await
            }
        }
    }
}

fn var(key: &'static str) -> Result<String> {
    std::env::var(key).map_err(Error::from)
}

fn include_sample_data() -> Result<bool> {
    var("LIPL_STORAGE_MEMORY_SAMPLE").and_then(|s| s.parse::<bool>().map_err(Error::from))
}

pub fn repo_type() -> Result<RepoType> {
    var("LIPL_STORAGE_REPO_TYPE").and_then(|s| {
        #[cfg(feature = "postgres")]
        if s.trim().to_lowercase() == "postgres" {
            return postgres_connection().map(RepoType::Postgres);
        }
        
        #[cfg(feature = "fs")]
        if s.trim().to_lowercase() == "fs" {
            return file_path().map(RepoType::Fs);
            
        }

        #[cfg(feature = "memory")]
        if s.trim().to_lowercase() == "memory" {
            return include_sample_data().map(RepoType::Memory);
        }

        Err(Error::InvalidConfiguration)
    })
}

#[cfg(feature = "postgres")]
fn postgres_connection() -> Result<String> {
    var("LIPL_STORAGE_POSTGRES_CONNECTION")
}

#[cfg(feature = "fs")]
fn file_path() -> Result<String> {
    var("LIPL_STORAGE_FS_DIR")
}
