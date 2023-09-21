use lipl_core::{LiplRepo, ToRepo};
use std::{str::FromStr, sync::Arc};

#[cfg(feature = "file")]
const PREFIX_FILE: &str = "file:";

#[cfg(feature = "memory")]
const PREFIX_MEMORY: &str = "memory:";

#[cfg(feature = "postgres")]
const PREFIX_POSTGRES: &str = "postgres:";

#[cfg(feature = "redis")]
const PREFIX_REDIS: &str = "redis:";

#[derive(Clone)]
pub enum RepoConfig {
    #[cfg(feature = "postgres")]
    Postgres(Box<lipl_storage_postgres::PostgresRepoConfig>),

    #[cfg(feature = "file")]
    File(Box<lipl_storage_fs::FileRepoConfig>),

    #[cfg(feature = "redis")]
    Redis(Box<lipl_storage_redis::redis_repo::RedisRepoConfig<String>>),

    #[cfg(feature = "memory")]
    Memory(Box<lipl_storage_memory::MemoryRepoConfig>),
}

impl RepoConfig {
    pub async fn build_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        match self {
            #[cfg(feature = "file")]
            RepoConfig::File(config) => {
                config.to_repo().await
            },

            #[cfg(feature = "postgres")]
            RepoConfig::Postgres(config) => {
                config.to_repo().await
            },

            #[cfg(feature = "redis")]
            RepoConfig::Redis(config) => {
                config.to_repo().await
            }

            #[cfg(feature = "memory")]
            RepoConfig::Memory(config) => {
                config.to_repo().await
            }
        }
    }
}

impl FromStr for Box<RepoConfig> {
    type Err = lipl_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<RepoConfig>().map(Box::new)
    }
}

impl FromStr for RepoConfig {
    type Err = lipl_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "file")]
        if s.starts_with(PREFIX_FILE) {
            return 
                s.strip_prefix(PREFIX_FILE)
                .unwrap()
                .parse::<lipl_storage_fs::FileRepoConfig>()
                .map(Box::new)
                .map(RepoConfig::File);
        }

        #[cfg(feature = "postgres")]
        if s.starts_with(PREFIX_POSTGRES) {
            return 
                s.strip_prefix(PREFIX_POSTGRES)
                    .unwrap()
                    .parse::<lipl_storage_postgres::PostgresRepoConfig>()
                    .map(Box::new)
                    .map(RepoConfig::Postgres);
        }

        #[cfg(feature = "redis")]
        if s.starts_with(PREFIX_REDIS) {
            return 
                s.strip_prefix(PREFIX_REDIS)
                    .unwrap()
                    .parse::<lipl_storage_redis::redis_repo::RedisRepoConfig<String>>()
                    .map(Box::new)
                    .map(RepoConfig::Redis);
        }

        #[cfg(feature = "memory")]
        if s.starts_with(PREFIX_MEMORY) {
            return
                s.strip_prefix(PREFIX_MEMORY)
                    .unwrap()
                    .parse::<lipl_storage_memory::MemoryRepoConfig>()
                    .map(Box::new)
                    .map(RepoConfig::Memory);
        }

        Err(lipl_core::Error::Argument("Invalid argument"))
    }
}
