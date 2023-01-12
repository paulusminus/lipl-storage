pub mod io;
pub mod model;

use std::{path::{Path, PathBuf}, sync::Arc};
use tokio::sync::RwLock;

use lipl_core::{LiplRepo, Lyric, Summary, Uuid, Playlist};
use model::{Db, HasSummaries};
pub use serde::{Deserialize, Serialize};

pub type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
pub struct RepoWrapper {
    inner: Arc<RwLock<Db>>,
}

impl RepoWrapper {
    pub fn new<P>(path: P) -> RepoWrapper where P: AsRef<Path> {
        let path_buf = PathBuf::from(path.as_ref());
        RepoWrapper { 
            inner: Arc::new(RwLock::new(Db::new(path_buf)))
        }
    }
}

#[async_trait::async_trait]
impl LiplRepo for RepoWrapper {
    async fn get_lyrics(&self) -> lipl_core::Result<Vec<Lyric>> {
        Ok(
            self.inner.read().await.get_lyric_list()
        )
    }

    async fn get_lyric_summaries(&self) -> lipl_core::Result<Vec<Summary>>  {
        Ok(
            self.inner.read().await.get_lyric_list().to_summaries()
        )
    }

    async fn get_lyric(&self, id: Uuid) -> lipl_core::Result<Lyric> {
        self.inner
            .read()
            .await
            .get_lyric(&id)
            .ok_or_else(|| lipl_core::Error::NoKey(id.to_string()))
            .map_err(Into::into)
    }

    async fn delete_lyric(&self, id: Uuid) -> lipl_core::Result<()> {
        self
            .inner
            .write()
            .await
            .delete_lyric(&id)
    }

    async fn upsert_lyric(&self, lyric: Lyric) -> lipl_core::Result<Lyric> {
        self
            .inner
            .write()
            .await
            .update_lyric(&lyric)
    }

    async fn get_playlists(&self) -> lipl_core::Result<Vec<Playlist>> {
        Ok(
            self.inner.read().await.get_playlist_list()
        )
    }

    async fn get_playlist_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        Ok(
            self.inner.read().await.get_playlist_list().to_summaries()
        )
    }

    async fn get_playlist(&self, id: Uuid) -> lipl_core::Result<Playlist> {
        self.inner
            .read()
            .await
            .get_playlist(&id)
            .ok_or_else(|| lipl_core::Error::NoKey(id.to_string()))
    }

    async fn delete_playlist(&self, id: Uuid) -> lipl_core::Result<()> {
        self.inner
            .write()
            .await
            .delete_playlist(&id)
    }

    async fn upsert_playlist(&self, playlist: Playlist) -> lipl_core::Result<Playlist> {
        self.inner.write()
        .await
        .update_playlist(&playlist)
        .map_err(Into::into)
    }

    async fn stop(&self) -> lipl_core::Result<()> {
        Ok(())
    }
}