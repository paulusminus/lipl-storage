use crate::create_router;
use crate::{Error, RepoConfig, Result};
use axum::Router;

fn var(key: &'static str) -> Result<String> {
    std::env::var(key).map_err(Error::from)
}

#[cfg(feature = "memory")]
fn include_sample_data() -> Result<bool> {
    var("LIPL_STORAGE_MEMORY_SAMPLE").and_then(|s| s.parse::<bool>().map_err(Error::from))
}

pub async fn repo() -> Result<Router> {
    let repo_type = var("LIPL_STORAGE_REPO_TYPE")?;
    let trimmed = repo_type.trim().to_lowercase();
    let r = trimmed.as_str();

    async fn to_router<T>(repo_config: T) -> Result<Router>
    where
        T: RepoConfig + Send + Sync + 'static,
        <T as RepoConfig>::Repo: Send + Sync + 'static,
    {
        repo_config
            .to_repo()
            .await
            .map_err(Into::into)
            .map(create_router)
    }

    #[cfg(feature = "postgres")]
    if r == "postgres" {
        use lipl_storage_postgres::PostgresConfig;
        let s = postgres_connection()?;
        return to_router(PostgresConfig::from(s)).await;
    }

    #[cfg(feature = "fs")]
    if r == "fs" {
        use lipl_storage_fs::FileRepoConfig;
        return to_router(file_path().parse::<FileRepoConfig>()?).await;
    }

    #[cfg(feature = "memory")]
    if r == "memory" {
        use lipl_storage_memory::MemoryRepoConfig;
        let sample_data = include_sample_data()?;
        return to_router(MemoryRepoConfig {
            sample_data,
            transaction_log: None,
        })
        .await;
    }

    #[cfg(feature = "redis")]
    if r == "redis" {
        use lipl_storage_redis::RedisRepoConfig;
        return to_router(redis_connection()?.parse::<RedisRepoConfig<_>>()?).await;
    }

    #[cfg(feature = "turso")]
    if r == "turso" {
        use lipl_storage_turso::TursoConfig;
        return to_router(turso_connection().map(TursoConfig::from)?).await;
    }

    Err(Error::InvalidConfiguration)
}

#[cfg(feature = "postgres")]
fn postgres_connection() -> Result<String> {
    var("LIPL_STORAGE_POSTGRES_CONNECTION")
}

#[cfg(feature = "redis")]
fn redis_connection() -> Result<String> {
    var("LIPL_STORAGE_REDIS_CONNECTION")
}

#[cfg(feature = "turso")]
fn turso_connection() -> Result<String> {
    var("LIPL_STORAGE_TURSO_DATABASE_PATH")
}

#[cfg(feature = "fs")]
fn file_path() -> String {
    var("LIPL_STORAGE_FS_DIR").unwrap_or(".".to_owned())
}

pub fn username() -> Result<String> {
    std::env::var("LIPL_USERNAME").map_err(Into::into)
}

pub fn password() -> Result<String> {
    std::env::var("LIPL_PASSWORD").map_err(Into::into)
}

pub fn www_root() -> String {
    var("WWW_ROOT").unwrap_or(".".to_owned())
}
