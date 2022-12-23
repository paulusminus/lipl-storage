use std::{str::FromStr, future::IntoFuture, pin::Pin};
use anyhow::bail;
use futures::{TryFutureExt, Future, FutureExt};
use lipl_fs_repo::{FileRepo, FileRepoConfig};
use lipl_postgres_repo::{PostgresRepo, PostgresRepoConfig};

#[derive(Clone)]
pub enum RepoConfig {
    Postgres(PostgresRepoConfig),
    File(FileRepoConfig),
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

impl IntoFuture for RepoConfig {
    type Output = anyhow::Result<Repo>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        match self {
            RepoConfig::File(config) => {
                async move { 
                    FileRepo::new(config.path)
                }
                .map_ok(Repo::File)
                .boxed()
            },
            RepoConfig::Postgres(config) => {
                PostgresRepo::new(config)
                .map_ok(Repo::Postgres)
                .boxed()
            }
        }
    }
}
