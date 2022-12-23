use std::fmt::{Debug};
use std::future::ready;

use async_trait::{async_trait};
use deadpool_postgres::{Pool};
use futures_util::{TryFutureExt};
use lipl_core::{Lyric, LiplRepo, Playlist, Summary, Uuid, into_anyhow_error};
use parts::{to_text, to_parts};
use tokio_postgres::{Row};

use crate::db::crud;
use crate::macros::query;
pub use error::PostgresRepoError;

mod constant;
mod db;
mod error;
pub mod pool;
mod macros;

type PostgresResult<T> = std::result::Result<T, PostgresRepoError>;

#[derive(Clone)]
pub struct PostgresRepoConfig {
    pub connection_string: String,
    pub clear: bool,
    pub pool: Pool,
}

impl PostgresRepoConfig {
    pub fn clear(self, clear: bool) -> Self {
        Self {
            connection_string: self.connection_string,
            clear,
            pool: self.pool,
        }
    }
}

impl std::str::FromStr for PostgresRepoConfig {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        pool::get(s, constant::POOL_MAX_SIZE)
            .map_err(anyhow::Error::from)
            .map(|pool| PostgresRepoConfig { connection_string: s.into(), clear: false, pool })
    }
}

// impl std::future::IntoFuture for PostgresRepoConfig {
//     type Output = anyhow::Result<PostgresRepo>;
//     type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output>>>;

//     fn into_future(self) -> Self::IntoFuture {
//         PostgresRepo::new(self).boxed()
//     }
// }

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
    pub async fn new(postgres_repo_config: PostgresRepoConfig) -> anyhow::Result<PostgresRepo> {
        if postgres_repo_config.clear {
            for sql in db::DROP.iter() {
                postgres_repo_config.pool.get()
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
            postgres_repo_config.pool.get().await?.execute(*sql, &[]).await?;
        };

        Ok(
            Self { pool: postgres_repo_config.pool, connection_string: postgres_repo_config.connection_string }
        )
    }

    query! (
        upsert_lyric,
        execute,
        u64,
        crud::UPSERT_LYRIC,
        crud::UPSERT_LYRIC_TYPES,
        to_ok,
        id: uuid::Uuid,
        title: String,
        text: String,
    );

    query! (
        upsert_playlist,
        query,
        Vec<Row>,
        crud::UPSERT_PLAYLIST,
        crud::UPSERT_PLAYLIST_TYPES,
        to_ok,
        id: uuid::Uuid,
        title: String,
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

fn get_id(row: &Row) -> PostgresResult<Uuid> {
    row.try_get::<&str, uuid::Uuid>("id")
    .map_err(PostgresRepoError::from)
    .map(Uuid::from)
}

#[allow(clippy::map_identity)]
fn get_title(row: &Row) -> PostgresResult<String> {
    row.try_get::<&str, String>("title")
    .map_err(PostgresRepoError::from)
    .map(std::convert::identity)
}

fn get_parts(row: &Row) -> PostgresResult<Vec<Vec<String>>> {
    row.try_get::<&str, String>("parts")
    .map_err(PostgresRepoError::from)
    .map(to_parts)
}

fn get_members(row: &Row) -> PostgresResult<Vec<Uuid>> {
    row.try_get::<&str, Vec<uuid::Uuid>>("members")
    .map_err(PostgresRepoError::from)
    .map(convert_vec(Uuid::from))
}

fn convert_vec<F, T, U>(f: F) -> impl Fn(Vec<T>) -> Vec<U>
where F: Fn(T) -> U + Copy
{
    move |v| v.into_iter().map(f).collect()
}

fn try_convert_vec<F, T, U>(f: F) -> impl Fn(Vec<T>) -> PostgresResult<Vec<U>>
where F: Fn(T) -> PostgresResult<U> + Copy
{
    move |v| v.into_iter().map(f).collect()
}

fn to_lyric(row: Row) -> PostgresResult<Lyric> {
    Ok(
        Lyric {
            id: get_id(&row)?,
            title: get_title(&row)?,
            parts: get_parts(&row)?,
        }
    )    
}

fn to_playlist(row: Row) -> PostgresResult<Playlist> {
    Ok(
        Playlist {
            id: get_id(&row)?,
            title: get_title(&row)?,
            members: get_members(&row)?,
        }
    )
}

fn to_summary(row: Row) -> PostgresResult<Summary> {
    Ok(
        Summary {
            id: get_id(&row)?,
            title: get_title(&row)?,
        }
    )
}

fn to_ok<T>(t: T) -> PostgresResult<T> {
    Ok(t)
}



#[async_trait]
impl LiplRepo for PostgresRepo {
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>>
    {
        self.lyrics().await.map_err(into_anyhow_error)
    }

    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>>
    {
        self.lyric_summaries().await.map_err(into_anyhow_error)
    }

    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric>
    {
        self.lyric_detail(id.inner()).await.map_err(into_anyhow_error)
    }

    async fn post_lyric(&self, lyric: Lyric) -> anyhow::Result<Lyric>
    {
        self.upsert_lyric(
            lyric.id.inner(),
            lyric.title,
            to_text(&lyric.parts[..])
        )
        .and_then(
            move |_| self.lyric_detail(lyric.id.inner())
        )
        .await
        .map_err(into_anyhow_error)
    }

    async fn delete_lyric(&self, id: Uuid) -> anyhow::Result<()>
    {
        self.lyric_delete(id.inner())
        .map_ok(to_unit)
        .await
        .map_err(into_anyhow_error)
    }

    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>>
    {
        self.playlists().await.map_err(into_anyhow_error)
    }

    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>>
    {
        self.playlist_summaries().await.map_err(into_anyhow_error)
    }

    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist>
    {
        self.playlist_detail(id.inner()).await.map_err(into_anyhow_error)
    }

    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist>
    {
        let title = playlist.title.clone();
        self.upsert_playlist(
            playlist.id.inner(),
            title,
            playlist.members.iter().map(|uuid| uuid.inner()).collect()
        )
        .map_ok(|_| {})
        .map_err(into_anyhow_error)
        .and_then(move |_| self.get_playlist(playlist.id))
        .await
    }

    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()>
    {
        self.playlist_delete(id.inner())
        .map_ok(to_unit)
        .await
        .map_err(into_anyhow_error)
    }

    async fn stop(&self) -> anyhow::Result<()>
    {
        ready(Ok::<(), PostgresRepoError>(())).await.map_err(into_anyhow_error)
    }
}

fn to_unit<T>(_: T) { }


#[cfg(test)]
mod test {
    use std::mem::size_of;


    #[test]
    fn postgres_repo_is_sized() {
        assert_eq!(1, 1);
        assert_eq!(size_of::<super::PostgresRepo>(), 32);
    }
}