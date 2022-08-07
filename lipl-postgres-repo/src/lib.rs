use async_trait::{async_trait};
use deadpool_postgres::{Pool};
use lipl_types::{Lyric, LiplRepo, Playlist, Summary, Uuid};
use parts::{to_parts, to_text};
use tokio_postgres::{Row};
pub use connection::{ConnectionBuilder};

use crate::db::crud;
use crate::macros::query;

mod connection;
mod db;
mod error;
pub mod pool;
mod macros;

type Result<T> = std::result::Result<T, error::Error>;

fn lyric_try_from(row: Row) -> Result<Lyric> {
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

fn summary_try_from(row: Row) -> Result<Summary> {
    let uuid = row.try_get::<&str, uuid::Uuid>("id")?;
    let title = row.try_get::<&str, String>("title")?;
    Ok(
        Summary {
            id: uuid.into(),
            title,
        }
    )    
}

#[derive(Clone)]
pub struct PostgresRepo {
    pool: Pool,
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

        Ok(Self { pool })
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
        delete_lyric,
        execute,
        u64,
        crud::DELETE_LYRIC,
        crud::DELETE_LYRIC_TYPES,
        identity,
        id: uuid::Uuid,
    );

    query! (
        delete_playlist,
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
    lyric_try_from(row)
}

fn to_lyrics(rows: Vec<Row>) -> Result<Vec<Lyric>> {
    let mut result = vec![];
    for row in rows {
        let lyric = to_lyric(row)?;
        result.push(lyric);
    }
    Ok(result)
}

fn to_summary(row: Row) -> Result<Summary> {
    summary_try_from(row)
}

fn to_summaries(rows: Vec<Row>) -> Result<Vec<Summary>> {
    let mut result = vec![];
    for row in rows {
        let summary = to_summary(row)?;
        result.push(summary);
    }
    Ok(result)
}

fn identity<T>(t: T) -> Result<T> {
    Ok(t)
}

#[async_trait]
impl LiplRepo for PostgresRepo {
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>> {
        let lyrics = self.lyrics().await?;
        Ok(lyrics)
    }

    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        self.lyric_summaries().await
    }

    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric> {
        self.lyric_detail(id.inner()).await
    }

    async fn post_lyric(&self, lyric: Lyric) -> anyhow::Result<Lyric> {
        let text = to_text(&lyric.parts[..]);
        self.upsert_lyric(lyric.id.inner(), &lyric.title, &text).await.map(ignore)?;
        self.lyric_detail(lyric.id.inner()).await
    }

    async fn delete_lyric(&self, id: Uuid) -> anyhow::Result<()> {
        self.delete_lyric(id.inner()).await.map(ignore)
    }

    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>> {
        let mut result = vec![];
        let summaries = self.get_playlist_summaries().await?;
        for summary in summaries {
            let playlist = self.get_playlist(summary.id).await?;
            result.push(playlist);
        }
        Ok(result)
    }

    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        self.playlist_summaries().await
    }

    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist> {
        let members = self.playlist_members(id.inner()).await?;
        let ids = members.into_iter().map(|s| s.id).collect::<Vec<_>>();
        let summary = self.playlist_summary(id.inner()).await?;
        let playlist = Playlist {
            id: summary.id,
            title: summary.title,
            members: ids,
        };
        Ok(playlist)
    }

    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist> {
        self.upsert_playlist(playlist.id.inner(), &playlist.title).await.map(ignore)?;
        self.set_playlist_members(
            playlist.id.inner(),
            playlist.members.iter().map(|uuid| uuid.inner()).collect::<Vec<_>>()
        )
        .await?;
        // let playlist = self.get_playlist(playlist.id).await?;
        Ok(playlist)
    }

    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()> {
        self.delete_playlist(id.inner()).await.map(ignore)
    }

    async fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

fn ignore<T>(_: T) {
    ()
}