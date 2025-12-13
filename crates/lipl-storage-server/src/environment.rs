use crate::{Error, Result, ToRepo};
use lipl_core::LiplRepo;
use std::sync::Arc;

fn var(key: &'static str) -> Result<String> {
    std::env::var(key).map_err(Error::from)
}

#[cfg(feature = "memory")]
fn include_sample_data() -> Result<bool> {
    var("LIPL_STORAGE_MEMORY_SAMPLE").and_then(|s| s.parse::<bool>().map_err(Error::from))
}

pub fn repo() -> Result<Arc<dyn LiplRepo>> {
    var("LIPL_STORAGE_REPO_TYPE").and_then(|s| {
        let repo_type = s.trim().to_lowercase();
        let r = repo_type.as_str();

        #[cfg(feature = "postgres")]
        if r == "postgres" {
            use lipl_storage_postgres::connection_pool;
            let s = postgres_connection()?;
            let pool = connection_pool(&s)?;
            return Ok(Arc::new(pool) as Arc<dyn LiplRepo>);
        }

        #[cfg(feature = "fs")]
        if r == "fs" {
            use lipl_storage_fs::FileRepoConfig;
            let s = file_path();
            let repo = s.parse::<FileRepoConfig>()?.to_repo()?;
            return Ok(Arc::new(repo));
        }

        #[cfg(feature = "memory")]
        if r == "memory" {
            use lipl_storage_memory::MemoryRepoConfig;
            let s = include_sample_data()?;
            let repo = MemoryRepoConfig {
                sample_data: s,
                transaction_log: None,
            }
            .to_repo()?;
            return Ok(Arc::new(repo));
        }

        #[cfg(feature = "redis")]
        if r == "redis" {
            use lipl_storage_redis::RedisRepoConfig;
            let s = redis_connection()?;
            let repo = s.parse::<RedisRepoConfig<_>>()?.to_repo()?;
            return Ok(Arc::new(repo));
        }

        Err(Error::InvalidConfiguration)
    })
}

#[cfg(feature = "postgres")]
fn postgres_connection() -> Result<String> {
    var("LIPL_STORAGE_POSTGRES_CONNECTION")
}

#[cfg(feature = "redis")]
fn redis_connection() -> Result<String> {
    var("LIPL_STORAGE_REDIS_CONNECTION")
}

#[cfg(feature = "fs")]
fn file_path() -> String {
    var("LIPL_STORAGE_FS_DIR").unwrap_or(".".to_owned())
}
