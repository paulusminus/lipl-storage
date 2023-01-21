use lipl_core::{LiplRepo, ToRepo};
use std::{str::FromStr, sync::Arc};

#[derive(Clone)]
pub enum RepoConfig {
    #[cfg(feature = "postgres")]
    Postgres(Box<lipl_repo_postgres::PostgresRepoConfig>),

    #[cfg(feature = "file")]
    File(Box<lipl_repo_fs::FileRepoConfig>),

    #[cfg(feature = "redis")]
    Redis(Box<lipl_repo_redis::redis_repo::RedisRepoConfig<String>>),

    #[cfg(feature = "memory")]
    Memory(Box<lipl_repo_memory::MemoryRepoConfig>),
}

impl RepoConfig {
    pub async fn build_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        match self {
            #[cfg(feature = "file")]
            RepoConfig::File(config) => {
                config.to_repo().await
                // let repo = lipl_repo_fs::FileRepo::new(file.path)?;
                // Ok(Arc::new(repo))
            },

            #[cfg(feature = "postgres")]
            RepoConfig::Postgres(config) => {
                config.to_repo().await
                // let repo = lipl_repo_postgres::PostgresRepo::new(*postgres.clone()).await?;
                // Ok(Arc::new(repo))
            },

            #[cfg(feature = "redis")]
            RepoConfig::Redis(config) => {
                config.to_repo().await
                // let repo = lipl_repo_redis::RedisRepoConfig::default().to_repo().await?;
                // Ok(repo)
            }

            #[cfg(feature = "memory")]
            RepoConfig::Memory(config) => {
                config.to_repo().await
                // Ok(
                //     Arc::new(
                //         lipl_repo_memory::MemoryRepo::default()
                //     )
                // )
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
        if s.starts_with("file:") {
            return 
                s.strip_prefix("file:")
                .unwrap()
                .parse::<lipl_repo_fs::FileRepoConfig>()
                .map(Box::new)
                .map(RepoConfig::File);
        }

        #[cfg(feature = "postgres")]
        if s.starts_with("postgres:") {
            return 
                s.strip_prefix("postgres:")
                    .unwrap()
                    .parse::<lipl_repo_postgres::PostgresRepoConfig>()
                    .map(Box::new)
                    .map(RepoConfig::Postgres);
        }

        #[cfg(feature = "redis")]
        if s.starts_with("redis:") {
            return 
                s.strip_prefix("redis:")
                    .unwrap()
                    .parse::<lipl_repo_redis::redis_repo::RedisRepoConfig<String>>()
                    .map(Box::new)
                    .map(RepoConfig::Redis);
        }

        #[cfg(feature = "memory")]
        if s.starts_with("memory:") {
            return
                s.strip_prefix("memory:")
                    .unwrap()
                    .parse::<lipl_repo_memory::MemoryRepoConfig>()
                    .map(Box::new)
                    .map(RepoConfig::Memory);
        }

        return Err(lipl_core::Error::Argument("Invalid argument"));
    }
}
