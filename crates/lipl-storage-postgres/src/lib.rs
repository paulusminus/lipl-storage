use std::fmt::Debug;
use std::future::ready;
use std::sync::Arc;

use async_trait::async_trait;
use bb8_postgres::bb8::Pool;
use bb8_postgres::tokio_postgres::{NoTls, Row};
use bb8_postgres::PostgresConnectionManager;
use futures_util::TryFutureExt;
use lipl_core::{
    parts::to_text, postgres_error, Error, LiplRepo, Lyric, Playlist, Result, Summary, ToRepo, Uuid,
};

use crate::db::crud;
use crate::macros::query;

mod constant;
mod convert;
mod db;
mod macros;
pub mod pool;

#[derive(Clone)]
pub struct PostgresRepoConfig {
    pub connection_string: String,
    pub clear: bool,
    pub manager: PostgresConnectionManager<NoTls>,
}

impl PostgresRepoConfig {
    pub fn clear(self, clear: bool) -> Self {
        Self {
            connection_string: self.connection_string,
            clear,
            manager: self.manager,
        }
    }
}

impl std::str::FromStr for PostgresRepoConfig {
    type Err = lipl_core::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        pool::get(s)
            .map_err(lipl_core::Error::from)
            .map(|manager| PostgresRepoConfig {
                connection_string: s.into(),
                clear: false,
                manager,
            })
    }
}

#[async_trait]
impl ToRepo for PostgresRepoConfig {
    async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        let repo = PostgresRepo::new(self).await?;
        Ok(Arc::new(repo))
    }
}

#[derive(Clone)]
pub struct PostgresRepo {
    pool: Pool<PostgresConnectionManager<NoTls>>,
    connection_string: String,
}

impl Debug for PostgresRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Postgres {}", self.connection_string)
    }
}

impl PostgresRepo {
    pub async fn new(postgres_repo_config: PostgresRepoConfig) -> lipl_core::Result<PostgresRepo> {
        let pool = Pool::builder()
            .max_size(constant::POOL_MAX_SIZE)
            .build(postgres_repo_config.manager)
            .map_err(|error| Error::Connection(Box::new(error)))
            .await?;
        if postgres_repo_config.clear {
            for sql in db::DROP.iter() {
                pool.get()
                    .map_err(|error| Error::Postgres(Box::new(error)))
                    .and_then(|connection| async move {
                        connection
                            .execute(*sql, &[])
                            .map_err(postgres_error)
                            .map_ok(to_unit)
                            .await
                    })
                    .await?;
            }
        }

        for sql in db::CREATE {
            pool.get()
                .map_err(postgres_error)
                .and_then(|connection| async move {
                    connection.execute(*sql, &[]).map_err(postgres_error).await
                })
                .await?;
        }

        Ok(Self {
            pool,
            connection_string: postgres_repo_config.connection_string,
        })
    }

    query! (
        upsert_lyric,
        execute,
        u64,
        crud::UPSERT_LYRIC,
        crud::UPSERT_LYRIC_TYPES,
        convert::to_ok,
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
        convert::to_ok,
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
        convert::to_ok,
        id: uuid::Uuid,
    );

    query! (
        playlist_delete,
        execute,
        u64,
        crud::DELETE_PLAYLIST,
        crud::DELETE_PLAYLIST_TYPES,
        convert::to_ok,
        id: uuid::Uuid,
    );

    query!(
        lyric_summaries,
        query,
        Vec<Summary>,
        crud::SELECT_LYRIC_SUMMARIES,
        crud::SELECT_LYRIC_SUMMARIES_TYPES,
        convert::try_convert_vec(convert::to_summary),
    );

    query!(
        lyrics,
        query,
        Vec<Lyric>,
        crud::SELECT_LYRICS,
        crud::SELECT_LYRICS_TYPES,
        convert::try_convert_vec(convert::to_lyric),
    );

    query! (
        lyric_detail,
        query_one,
        Lyric,
        crud::SELECT_LYRIC_DETAIL,
        crud::SELECT_LYRIC_DETAIL_TYPES,
        convert::to_lyric,
        id: uuid::Uuid,
    );

    query! {
        playlists,
        query,
        Vec<Playlist>,
        crud::SELECT_PLAYLISTS,
        crud::SELECT_PLAYLISTS_TYPES,
        convert::try_convert_vec(convert::to_playlist),
    }

    query! {
        playlist_detail,
        query_one,
        Playlist,
        crud::SELECT_PLAYLIST_DETAIL,
        crud::SELECT_PLAYLIST_DETAIL_TYPES,
        convert::to_playlist,
        id: uuid::Uuid,
    }

    query!(
        playlist_summaries,
        query,
        Vec<Summary>,
        crud::SELECT_PLAYLIST_SUMMARIES,
        crud::SELECT_PLAYLIST_SUMMARIES_TYPES,
        convert::try_convert_vec(convert::to_summary),
    );
}

#[async_trait]
impl LiplRepo for PostgresRepo {
    async fn get_lyrics(&self) -> lipl_core::Result<Vec<Lyric>> {
        self.lyrics().err_into().await
    }

    async fn get_lyric_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        self.lyric_summaries().err_into().await
    }

    async fn get_lyric(&self, id: Uuid) -> lipl_core::Result<Lyric> {
        self.lyric_detail(id.inner()).err_into().await
    }

    async fn upsert_lyric(&self, lyric: Lyric) -> lipl_core::Result<Lyric> {
        self.upsert_lyric(lyric.id.inner(), lyric.title, to_text(&lyric.parts[..]))
            .and_then(move |_| self.lyric_detail(lyric.id.inner()))
            .err_into()
            .await
    }

    async fn delete_lyric(&self, id: Uuid) -> lipl_core::Result<()> {
        self.lyric_delete(id.inner())
            .map_ok(to_unit)
            .err_into()
            .await
    }

    async fn get_playlists(&self) -> lipl_core::Result<Vec<Playlist>> {
        self.playlists().err_into().await
    }

    async fn get_playlist_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        self.playlist_summaries().err_into().await
    }

    async fn get_playlist(&self, id: Uuid) -> lipl_core::Result<Playlist> {
        self.playlist_detail(id.inner()).err_into().await
    }

    async fn upsert_playlist(&self, playlist: Playlist) -> lipl_core::Result<Playlist> {
        let title = playlist.title.clone();
        self.upsert_playlist(
            playlist.id.inner(),
            title,
            playlist.members.iter().map(|uuid| uuid.inner()).collect(),
        )
        .err_into()
        .and_then(move |_| self.get_playlist(playlist.id))
        .await
    }

    async fn delete_playlist(&self, id: Uuid) -> lipl_core::Result<()> {
        self.playlist_delete(id.inner())
            .map_ok(to_unit)
            .err_into()
            .await
    }

    async fn stop(&self) -> lipl_core::Result<()> {
        ready(Ok::<(), Error>(())).err_into().await
    }
}

fn to_unit<T>(_: T) {}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    #[test]
    fn postgres_repo_is_sized() {
        assert_eq!(1, 1);
        assert_eq!(size_of::<super::PostgresRepo>(), 32);
    }
}
