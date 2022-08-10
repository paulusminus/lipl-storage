use std::fmt::Debug;

use async_trait::{async_trait};
use deadpool_postgres::{Pool};
use futures_util::TryFutureExt;
use lipl_types::{time_it, Lyric, LiplRepo, Playlist, Summary, Uuid};
use parts::{to_text, to_parts};
use tokio_postgres::{Row};

use crate::db::crud;
use crate::macros::query;
pub use error::PostgresRepoError;

mod db;
mod error;
pub mod pool;
mod macros;

type Result<T> = std::result::Result<T, PostgresRepoError>;

#[derive(Clone)]
pub struct PostgresRepo {
    pool: Pool,
    connection_string: String,
}

impl Debug for PostgresRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Postgres {}", self.connection_string)
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
        query,
        Vec<Row>,
        crud::UPSERT_PLAYLIST,
        crud::UPSERT_PLAYLIST_TYPES,
        identity,
        id: uuid::Uuid,
        title: &str,
        members: Vec<uuid::Uuid>,
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

    query!{
        playlists,
        query,
        Vec<Playlist>,
        crud::SELECT_PLAYLISTS,
        crud::SELECT_PLAYLISTS_TYPES,
        to_playlists,
    }

    query!{
        playlist_detail,
        query_one,
        Playlist,
        crud::SELECT_PLAYLIST_DETAIL,
        crud::SELECT_PLAYLIST_DETAIL_TYPES,
        to_playlist,
        id: uuid::Uuid,
    }

    query! (
        playlist_summaries,
        query,
        Vec<Summary>,
        crud::SELECT_PLAYLIST_SUMMARIES,
        crud::SELECT_PLAYLIST_SUMMARIES_TYPES,
        to_summaries,
    );
}

fn get_id(row: &Row) -> Result<Uuid> {
    Ok(row.try_get::<&str, uuid::Uuid>("id").map(Uuid::from)?)
}

fn get_title(row: &Row) -> Result<String> {
    Ok(row.try_get::<&str, String>("title")?)
}

fn get_parts(row: &Row) -> Result<Vec<Vec<String>>> {
    Ok(
        to_parts(row.try_get::<&str, String>("parts")?)
    )
}

fn get_members(row: &Row) -> Result<Vec<Uuid>> {
    Ok(
        row.try_get::<&str, Vec<uuid::Uuid>>("members")?
        .into_iter().map(Uuid::from)
        .collect()
    )
}

fn to_lyric(row: Row) -> Result<Lyric> {
    Ok(
        Lyric {
            id: get_id(&row)?,
            title: get_title(&row)?,
            parts: get_parts(&row)?,
        }
    )    
}

fn to_playlist(row: Row) -> Result<Playlist> {
    Ok(
        Playlist {
            id: get_id(&row)?,
            title: get_title(&row)?,
            members: get_members(&row)?,
        }
    )
}

fn to_lyrics(rows: Vec<Row>) -> Result<Vec<Lyric>> {
    rows
    .into_iter()
    .map(to_lyric)
    .collect::<Result<Vec<_>>>()
}

fn to_playlists(rows: Vec<Row>) -> Result<Vec<Playlist>> {
    rows
    .into_iter()
    .map(to_playlist)
    .collect::<Result<Vec<_>>>()
}

fn to_summary(row: Row) -> Result<Summary> {
    Ok(
        Summary {
            id: get_id(&row)?,
            title: get_title(&row)?,
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

#[async_trait]
impl LiplRepo for PostgresRepo {

    #[tracing::instrument]
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>> {
        time_it!(
            self.lyrics()
        )
    }

    #[tracing::instrument]
    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        time_it!(
            self.lyric_summaries()
        )
    }

    #[tracing::instrument]
    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric> {
        time_it!(
            self.lyric_detail(id.inner())
        )
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
        time_it!(
            self.lyric_delete(id.inner()).map_ok(|_| {})
        )
    }

    #[tracing::instrument]
    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>> {
        time_it!(
            self.playlists()
        )
    }

    #[tracing::instrument]
    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        time_it!(
            self.playlist_summaries()
        )
    }

    #[tracing::instrument]
    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist> {
        time_it!(
            self.playlist_detail(id.inner())
        )
    }

    #[tracing::instrument]
    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist> {
        time_it!(
            self.upsert_playlist(
                playlist.id.inner(),
                &playlist.title,
                playlist.members.into_iter().map(|uuid| uuid.inner()).collect()
            )
            .and_then(|_| self.get_playlist(playlist.id))
        )
    }

    #[tracing::instrument]
    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()> {
        time_it!(
            self.playlist_delete(id.inner()).map_ok(|_| {})
        )
    }

    #[tracing::instrument]
    async fn stop(&self) -> anyhow::Result<()> {
        time_it!(async { anyhow::Ok::<()>(())})
    }
}

fn ignore<T>(_: T) { }
