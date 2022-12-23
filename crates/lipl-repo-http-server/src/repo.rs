use async_trait::async_trait;
use lipl_core::{LiplRepo, Lyric, Summary, Uuid, Playlist};
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

macro_rules! dispatch {
    ($self: ident, $method:ident $(,$param:expr)*) => {
        match $self {
            Repo::File(file) => file.$method($($param),*).await,
            Repo::Postgres(postgres) => postgres.$method($($param)*).await
        }        
    };
}

#[async_trait]
impl LiplRepo for Repo {
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>> {
        dispatch!(self, get_lyrics)
    }

    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        dispatch!(self, get_lyric_summaries)
    }

    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric> {
        dispatch!(self, get_lyric, id)
    }

    async fn post_lyric(&self, lyric: Lyric) -> anyhow::Result<Lyric> {
        dispatch!(self, post_lyric, lyric)
    }

    async fn delete_lyric(&self, id: Uuid) -> anyhow::Result<()> {
        dispatch!(self, delete_lyric, id)
    }

    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>> {
        dispatch!(self, get_playlists)
    }

    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        dispatch!(self, get_playlist_summaries)
    }

    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist> {
        dispatch!(self, get_playlist, id)
    }

    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist> {
        dispatch!(self, post_playlist, playlist)
    }

    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()> {
        dispatch!(self, delete_playlist, id)
    }

    async fn stop(&self) -> anyhow::Result<()> {
        dispatch!(self, stop)
    }
}
