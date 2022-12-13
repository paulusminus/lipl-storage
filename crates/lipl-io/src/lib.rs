pub mod io;
pub mod model;
mod error;

use std::{path::{Path, PathBuf}, sync::Arc};
use tokio::sync::RwLock;

use lipl_core::{LiplRepo, Lyric, Summary, Uuid, Playlist};
use model::{Db, HasSummaries};
pub use serde::{Deserialize, Serialize};

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
    type Error = error::Error;

    async fn get_lyrics(&self) -> Result<Vec<Lyric>> {
        Ok(
            self.inner.read().await.get_lyric_list()
        )
    }

    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        Ok(
            self.inner.read().await.get_lyric_list().to_summaries()
        )
    }

    async fn get_lyric(&self, id: Uuid) -> Result<Lyric> {
        self.inner.read().await.get_lyric(&id).ok_or_else(|| crate::error::Error::NoKey(id.to_string()))
    }

    async fn delete_lyric(&self, id: Uuid) -> Result<()> {
        self.inner.write().await.delete_lyric(&id)
    }

    async fn post_lyric(&self, lyric: Lyric) -> Result<Lyric> {
        self.inner.write().await.update_lyric(&lyric)
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        Ok(
            self.inner.read().await.get_playlist_list()
        )
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        Ok(
            self.inner.read().await.get_playlist_list().to_summaries()
        )
    }

    async fn get_playlist(&self, id: Uuid) -> Result<Playlist> {
        self.inner.read().await.get_playlist(&id).ok_or_else(|| crate::error::Error::NoKey(id.to_string()))
    }

    async fn delete_playlist(&self, id: Uuid) -> Result<()> {
        self.inner.write().await.delete_playlist(&id)
    }

    async fn post_playlist(&self, playlist: Playlist) -> Result<Playlist> {
        self.inner.write().await.update_playlist(&playlist)
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}