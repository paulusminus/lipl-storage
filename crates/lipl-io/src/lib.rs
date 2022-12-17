pub mod io;
pub mod model;
mod error;

use std::{path::{Path, PathBuf}, sync::Arc};
use tokio::sync::RwLock;

use lipl_core::{LiplRepo, Lyric, Summary, Uuid, Playlist, into_anyhow_error};
use model::{Db, HasSummaries};
pub use serde::{Deserialize, Serialize};

pub type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type Result<T> = std::result::Result<T, error::Error>;

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
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>> {
        Ok(
            self.inner.read().await.get_lyric_list()
        )
    }

    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>>  {
        Ok(
            self.inner.read().await.get_lyric_list().to_summaries()
        )
    }

    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric> {
        self.inner
            .read()
            .await
            .get_lyric(&id)
            .ok_or_else(|| crate::error::Error::NoKey(id.to_string()))
            .map_err(into_anyhow_error)
    }

    async fn delete_lyric(&self, id: Uuid) -> anyhow::Result<()> {
        self.inner.write().await.delete_lyric(&id).map_err(into_anyhow_error)
    }

    async fn post_lyric(&self, lyric: Lyric) -> anyhow::Result<Lyric> {
        self.inner.write().await.update_lyric(&lyric).map_err(into_anyhow_error)
    }

    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>> {
        Ok(
            self.inner.read().await.get_playlist_list()
        )
    }

    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        Ok(
            self.inner.read().await.get_playlist_list().to_summaries()
        )
    }

    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist> {
        self.inner
            .read()
            .await
            .get_playlist(&id)
            .ok_or_else(|| crate::error::Error::NoKey(id.to_string()))
            .map_err(into_anyhow_error)
    }

    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()> {
        self.inner.write().await.delete_playlist(&id).map_err(into_anyhow_error)
    }

    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist> {
        self.inner.write().await.update_playlist(&playlist).map_err(into_anyhow_error)
    }

    async fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }
}