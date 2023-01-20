use lipl_core::{LiplRepo};
use std::{str::FromStr, sync::Arc};
use anyhow::bail;

#[derive(Clone)]
pub enum RepoConfig {
    #[cfg(feature = "postgres")]
    Postgres(Box<lipl_repo_postgres::PostgresRepoConfig>),

    #[cfg(feature = "file")]
    File(Box<lipl_repo_fs::FileRepoConfig>),

    #[cfg(feature = "redis")]
    Redis,

    #[cfg(feature = "memory")]
    Memory,
}

impl RepoConfig {
    pub async fn build_repo(self) -> anyhow::Result<Arc<dyn LiplRepo>> {
        match self {
            #[cfg(feature = "postgres")]
            RepoConfig::File(file) => {
                let repo = lipl_repo_fs::FileRepo::new(file.path)?;
                Ok(Arc::new(repo))
            },

            #[cfg(feature = "file")]
            RepoConfig::Postgres(postgres) => {
                let repo = lipl_repo_postgres::PostgresRepo::new(*postgres.clone()).await?;
                Ok(Arc::new(repo))
            },

            #[cfg(feature = "redis")]
            RepoConfig::Redis => {
                let repo = lipl_repo_redis::RedisRepoConfig::default().to_repo().await?;
                Ok(repo)
            }

            #[cfg(feature = "memory")]
            RepoConfig::Memory => {
                Ok(
                    Arc::new(
                        lipl_repo_memory::MemoryRepo::default()
                    )
                )
            }
        }
    }
}

impl FromStr for Box<RepoConfig> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<RepoConfig>().map(Box::new)
    }
}

impl FromStr for RepoConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(':').collect::<Vec<&str>>();
        if splitted.len() == 2 {
            let repo_dir = splitted[1].to_owned();

            #[cfg(feature = "file")]
            if splitted[0] == "file" {
                return repo_dir.parse::<FileRepoConfig>().map(Box::new).map(RepoConfig::File);
            }

            #[cfg(feature = "postgres")]
            if splitted[0] == "postgres" {
                return repo_dir.parse::<PostgresRepoConfig>().map(Box::new).map(RepoConfig::Postgres);
            }

            #[cfg(feature = "redis")]
            if splitted[0] == "redis" {
                return Ok(RepoConfig::Redis);
            }

            bail!("Unknown prefix for db connection string")
        }
        else {
            bail!("Problem with separator (none or too many)")
        }            
    }
}
