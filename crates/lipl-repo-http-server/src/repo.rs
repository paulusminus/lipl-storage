use lipl_core::{LiplRepo};
use std::{str::FromStr, sync::Arc};
use anyhow::bail;
use lipl_fs_repo::{FileRepo, FileRepoConfig};
use lipl_postgres_repo::{PostgresRepo, PostgresRepoConfig};

#[derive(Clone)]
pub enum RepoConfig {
    Postgres(PostgresRepoConfig),
    File(FileRepoConfig),
}

impl RepoConfig {
    pub async fn to_repo(&self) -> anyhow::Result<Repo> {
        match self {
            RepoConfig::File(file) => {
                FileRepo::new(file.path.clone()).map(Repo::File)
            },
            RepoConfig::Postgres(postgres) => {
                PostgresRepo::new(postgres.clone()).await.map(Repo::Postgres)
            }
        }
    }
}

impl FromStr for RepoConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(':').collect::<Vec<&str>>();
        if splitted.len() == 2 {
            let repo_dir = splitted[1].to_owned();
            if splitted[0] == "file" {
                repo_dir.parse::<FileRepoConfig>().map(RepoConfig::File)
            }
            else if splitted[0] == "postgres" {
                repo_dir.parse::<PostgresRepoConfig>().map(RepoConfig::Postgres)
            }
            else {
                bail!("Unknown prefix for db connection string")
            }
        }
        else {
            bail!("Problem with separator (none or too many)")
        }            
    }
}

#[derive(Clone)]
pub enum Repo {
    Postgres(PostgresRepo),
    File(FileRepo),
}

impl Repo {
    pub fn to_lipl_repo(self) -> Arc<dyn LiplRepo> {
        match self {
            Repo::File(repo) => Arc::new(repo),
            Repo::Postgres(repo) => Arc::new(repo),
        }
    }
}
