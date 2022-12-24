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
    pub async fn to_repo(self) -> anyhow::Result<Arc<dyn LiplRepo>> {
        match self {
            RepoConfig::File(file) => {
                let repo = FileRepo::new(file.path.clone())?;
                Ok(Arc::new(repo))
            },
            RepoConfig::Postgres(postgres) => {
                let repo = PostgresRepo::new(postgres.clone()).await?;
                Ok(Arc::new(repo))
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
