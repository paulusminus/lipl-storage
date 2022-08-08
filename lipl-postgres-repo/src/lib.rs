use std::fmt::Debug;
use std::time::{Instant};

use async_trait::{async_trait};
use deadpool_postgres::{Pool};
use lipl_types::{Lyric, LiplRepo, Playlist, Summary, Uuid};
use parts::{to_parts, to_text};
use tokio_postgres::{Row};

use crate::db::crud;
use crate::macros::query;

mod db;
mod error;
pub mod pool;
mod macros;

type Result<T> = std::result::Result<T, error::Error>;

#[derive(Clone)]
pub struct PostgresRepo {
    pool: Pool,
    connection_string: String,
}

impl Debug for PostgresRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Postgres repo: {}", self.connection_string)
    }
}

impl PostgresRepo {
    pub async fn new(connection_string: String, clear: bool) -> Result<Self> {
        let pool = pool::get(&connection_string, 16)?;
        if clear {
            for sql in db::DROP {
                pool.get().await?.execute(*sql, &[]).await?;
            }
        }

        for sql in db::CREATE {
            pool.get().await?.execute(*sql, &[]).await?;
        };

        Ok(Self { pool, connection_string })
    }

    query! (
        upsert_lyric,
        execute,
        u64,
        crud::UPSERT_LYRIC,
        crud::UPSERT_LYRIC_TYPES,
        identity,
        id: uuid::Uuid,
        title: &str,
        text: &str,
    );

    query! (
        upsert_playlist,
        execute,
        u64,
        crud::UPSERT_PLAYLIST,
        crud::UPSERT_PLAYLIST_TYPES,
        identity,
        id: uuid::Uuid,
        title: &str,
    );

    query! (
        lyric_delete,
        execute,
        u64,
        crud::DELETE_LYRIC,
        crud::DELETE_LYRIC_TYPES,
        identity,
        id: uuid::Uuid,
    );

    query! (
        playlist_delete,
        execute,
        u64,
        crud::DELETE_PLAYLIST,
        crud::DELETE_PLAYLIST_TYPES,
        identity,
        id: uuid::Uuid,
    );

    query! (
        lyric_summaries,
        query,
        Vec<Summary>,
        crud::SELECT_LYRIC_SUMMARIES,
        crud::SELECT_LYRIC_SUMMARIES_TYPES,
        to_summaries,
    );

    query! (
        lyrics,
        query,
        Vec<Lyric>,
        crud::SELECT_LYRICS,
        crud::SELECT_LYRICS_TYPES,
        to_lyrics,
    );

    query! (
        lyric_detail,
        query_one,
        Lyric,
        crud::SELECT_LYRIC_DETAIL,
        crud::SELECT_LYRIC_DETAIL_TYPES,
        to_lyric,
        id: uuid::Uuid,
    );

    query! (
        playlist_summaries,
        query,
        Vec<Summary>,
        crud::SELECT_PLAYLIST_SUMMARIES,
        crud::SELECT_PLAYLIST_SUMMARIES_TYPES,
        to_summaries,
    );

    query! (
        playlist_summary,
        query_one,
        Summary,
        crud::SELECT_PLAYLIST_SUMMARY,
        crud::SELECT_PLAYLIST_SUMMARY_TYPES,
        to_summary,
        id: uuid::Uuid,
    );


    query! (
        playlist_members,
        query,
        Vec<Summary>,
        crud::SELECT_PLAYLIST_MEMBERS,
        crud::SELECT_PLAYLIST_MEMBERS_TYPES,
        to_summaries,
        id: uuid::Uuid,
    );

    query! (
        set_playlist_members,
        execute,
        u64,
        crud::SET_PLAYLIST_MEMBERS,
        crud::SET_PLAYLIST_MEMBERS_TYPES,
        identity,
        id: uuid::Uuid,
        members: Vec<uuid::Uuid>,
    );


}


fn to_lyric(row: Row) -> Result<Lyric> {
    let uuid = row.try_get::<&str, uuid::Uuid>("id")?;
    let title = row.try_get::<&str, String>("title")?;
    let parts = row.try_get::<&str, String>("parts")?;
    Ok(
        Lyric {
            id: uuid.into(),
            title,
            parts: to_parts(parts),
        }
    )    
}

fn to_lyrics(rows: Vec<Row>) -> Result<Vec<Lyric>> {
    rows
    .into_iter()
    .map(to_lyric)
    .collect::<Result<Vec<_>>>()
}

fn to_summary(row: Row) -> Result<Summary> {
    let uuid = row.try_get::<&str, uuid::Uuid>("id")?;
    let title = row.try_get::<&str, String>("title")?;
    Ok(
        Summary {
            id: uuid.into(),
            title,
        }
    )
}

fn to_summaries(rows: Vec<Row>) -> Result<Vec<Summary>> {
    rows
    .into_iter()
    .map(to_summary)
    .collect::<Result<Vec<_>>>()
}

fn identity<T>(t: T) -> Result<T> {
    Ok(t)
}

macro_rules! time_it {
    ($process:expr) => {{
        let now = Instant::now();
        let result = $process.await?;
        tracing::info!(elapsed_microseconds = now.elapsed().as_micros());
        Ok(result)    
    }};
}

#[async_trait]
impl LiplRepo for PostgresRepo {

    #[tracing::instrument]
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>> {
        time_it!(self.lyrics())
    }

    #[tracing::instrument]
    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        time_it!(self.lyric_summaries())
    }

    #[tracing::instrument]
    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric> {
        time_it!(self.lyric_detail(id.inner()))
    }

    #[tracing::instrument]
    async fn post_lyric(&self, lyric: Lyric) -> anyhow::Result<Lyric> {
        time_it!(async {
            let text = to_text(&lyric.parts[..]);
            self.upsert_lyric(lyric.id.inner(), &lyric.title, &text).await.map(ignore)?;
            self.lyric_detail(lyric.id.inner()).await
        })
    }

    #[tracing::instrument]
    async fn delete_lyric(&self, id: Uuid) -> anyhow::Result<()> {
        time_it!(async {
            self.lyric_delete(id.inner()).await?;
            Ok::<(), anyhow::Error>(())
        })
    }

    #[tracing::instrument]
    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>> {
        time_it!(
            async {
                let mut result = vec![];
                let summaries = self.get_playlist_summaries().await?;
                for summary in summaries {
                    let playlist = self.get_playlist(summary.id).await?;
                    result.push(playlist);
                }
                Ok::<Vec<Playlist>, anyhow::Error>(result)        
            }
        )
    }

    #[tracing::instrument]
    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        time_it!(self.playlist_summaries())
    }

    #[tracing::instrument]
    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist> {
        time_it!(async {
            let members = self.playlist_members(id.inner()).await?;
            let ids = members.into_iter().map(|s| s.id).collect::<Vec<_>>();
            let summary = self.playlist_summary(id.inner()).await?;
            Ok::<Playlist, anyhow::Error>(Playlist {
                id: summary.id,
                title: summary.title,
                members: ids,
            })
        })
    }

    #[tracing::instrument]
    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist> {
        time_it!(async {
            self.upsert_playlist(playlist.id.inner(), &playlist.title).await.map(ignore)?;
            self.set_playlist_members(
                playlist.id.inner(),
                playlist.members.iter().map(|uuid| uuid.inner()).collect::<Vec<_>>()
            )
            .await?;
            self.get_playlist(playlist.id).await
        })
    }

    #[tracing::instrument]
    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()> {
        time_it!(async {
            self.playlist_delete(id.inner()).await?;
            Ok::<(), anyhow::Error>(())
        })
    }

    #[tracing::instrument]
    async fn stop(&self) -> anyhow::Result<()> {
        time_it!(async { Ok::<(), anyhow::Error>(())})
    }
}

fn ignore<T>(_: T) {
    
}