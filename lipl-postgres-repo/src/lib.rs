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
            for sql in db::DROP.iter() {
                pool.get()
                .map_err(PostgresRepoError::from)
                .and_then(
                    |pool| async move {
                        pool
                        .execute(*sql, &[])
                        .map_err(PostgresRepoError::from)
                        .map_ok(to_unit)
                        .await
                    }
                )
                .await?;
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
        to_ok,
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
        to_ok,
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
        to_ok,
        id: uuid::Uuid,
    );

    query! (
        playlist_delete,
        execute,
        u64,
        crud::DELETE_PLAYLIST,
        crud::DELETE_PLAYLIST_TYPES,
        to_ok,
        id: uuid::Uuid,
    );

    query! (
        lyric_summaries,
        query,
        Vec<Summary>,
        crud::SELECT_LYRIC_SUMMARIES,
        crud::SELECT_LYRIC_SUMMARIES_TYPES,
        try_convert_vec(to_summary),
    );

    query! (
        lyrics,
        query,
        Vec<Lyric>,
        crud::SELECT_LYRICS,
        crud::SELECT_LYRICS_TYPES,
        try_convert_vec(to_lyric),
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
        try_convert_vec(to_playlist),
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
        try_convert_vec(to_summary),
    );
}

fn get_id(row: &Row) -> Result<Uuid> {
    row.try_get::<&str, uuid::Uuid>("id")
    .map_err(PostgresRepoError::from)
    .map(Uuid::from)
}

fn get_title(row: &Row) -> Result<String> {
    row.try_get::<&str, String>("title")
    .map_err(PostgresRepoError::from)
    .map(std::convert::identity)
}

fn get_parts(row: &Row) -> Result<Vec<Vec<String>>> {
    row.try_get::<&str, String>("parts")
    .map_err(PostgresRepoError::from)
    .map(to_parts)
}

fn get_members(row: &Row) -> Result<Vec<Uuid>> {
    row.try_get::<&str, Vec<uuid::Uuid>>("members")
    .map_err(PostgresRepoError::from)
    .map(convert_vec(Uuid::from))
}

fn convert_vec<F, T, U>(f: F) -> impl Fn(Vec<T>) -> Vec<U>
where F: Fn(T) -> U + Copy
{
    move |v| v.into_iter().map(f).collect()
}

fn try_convert_vec<F, T, U>(f: F) -> impl Fn(Vec<T>) -> Result<Vec<U>>
where F: Fn(T) -> Result<U> + Copy
{
    move |v| v.into_iter().map(f).collect()
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

fn to_summary(row: Row) -> Result<Summary> {
    Ok(
        Summary {
            id: get_id(&row)?,
            title: get_title(&row)?,
        }
    )
}

fn to_ok<T>(t: T) -> Result<T> {
    Ok(t)
}

#[async_trait]
impl LiplRepo<PostgresRepoError> for PostgresRepo {

    #[tracing::instrument]
    async fn get_lyrics(&self) -> Result<Vec<Lyric>> {
        time_it!(
            self.lyrics()
        )
    }

    #[tracing::instrument]
    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        time_it!(
            self.lyric_summaries()
        )
    }

    #[tracing::instrument]
    async fn get_lyric(&self, id: Uuid) -> Result<Lyric> {
        time_it!(
            self.lyric_detail(id.inner())
        )
    }

    #[tracing::instrument]
    async fn post_lyric(&self, lyric: Lyric) -> Result<Lyric> {
        time_it!(
            self.upsert_lyric(
                lyric.id.inner(),
                &lyric.title,
                &to_text(&lyric.parts[..])
            )
            .and_then(
                |_| self.lyric_detail(lyric.id.inner())
            )
        )
    }

    #[tracing::instrument]
    async fn delete_lyric(&self, id: Uuid) -> Result<()> {
        time_it!(
            self.lyric_delete(id.inner())
            .map_ok(to_unit)
        )
    }

    #[tracing::instrument]
    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        time_it!(
            self.playlists()
        )
    }

    #[tracing::instrument]
    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        time_it!(
            self.playlist_summaries()
        )
    }

    #[tracing::instrument]
    async fn get_playlist(&self, id: Uuid) -> Result<Playlist> {
        time_it!(
            self.playlist_detail(id.inner())
        )
    }

    #[tracing::instrument]
    async fn post_playlist(&self, playlist: Playlist) -> Result<Playlist> {
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
    async fn delete_playlist(&self, id: Uuid) -> Result<()> {
        time_it!(
            self.playlist_delete(id.inner())
            .map_ok(to_unit)
        )
    }

    #[tracing::instrument]
    async fn stop(&self) -> Result<()> {
        time_it!(
            async {
                Ok::<(), PostgresRepoError>(())
            }
        )
    }
}

fn to_unit<T>(_: T) { }
