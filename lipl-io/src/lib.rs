pub mod io;
pub mod model;

use std::{path::{Path, PathBuf}};
use tokio::sync::RwLock;

use lipl_types::{LiplRepo, Lyric, RepoResult, Summary, Uuid, RepoError, Playlist};
use model::{Db, HasSummaries};
pub use serde::{Deserialize, Serialize};

pub struct RepoWrapper {
    inner: RwLock<Db>,
}

impl RepoWrapper {
    pub fn new<P>(path: P) -> RepoWrapper where P: AsRef<Path> {
        let path_buf = PathBuf::from(path.as_ref());
        RepoWrapper { 
            inner:  RwLock::new(Db::new(path_buf))
        }
    }
}

#[async_trait::async_trait]
impl LiplRepo for RepoWrapper {
    async fn get_lyrics(&self) -> RepoResult<Vec<Lyric>> {
        Ok(
            self.inner.read().await.get_lyric_list()
        )
    }

    async fn get_lyric_summaries(&self) -> RepoResult<Vec<Summary>> {
        Ok(
            self.inner.read().await.get_lyric_list().to_summaries()
        )
    }

    async fn get_lyric(&self, id: Uuid) -> RepoResult<Lyric> {
        self.inner.read().await.get_lyric(&id).ok_or_else(|| RepoError::NoKey(id.to_string()))
    }

    async fn delete_lyric(&self, id: Uuid) -> RepoResult<()> {
        self.inner.write().await.delete_lyric(&id)
    }

    async fn post_lyric(&self, lyric: Lyric) -> RepoResult<Lyric> {
        self.inner.write().await.update_lyric(&lyric)
    }

    async fn get_playlists(&self) -> RepoResult<Vec<Playlist>> {
        Ok(
            self.inner.read().await.get_playlist_list()
        )
    }

    async fn get_playlist_summaries(&self) -> RepoResult<Vec<Summary>> {
        Ok(
            self.inner.read().await.get_playlist_list().to_summaries()
        )
    }

    async fn get_playlist(&self, id: Uuid) -> RepoResult<Playlist> {
        self.inner.read().await.get_playlist(&id).ok_or_else(|| RepoError::NoKey(id.to_string()))
    }

    async fn delete_playlist(&self, id: Uuid) -> RepoResult<()> {
        self.inner.write().await.delete_playlist(&id)
    }

    async fn post_playlist(&self, playlist: Playlist) -> RepoResult<Playlist> {
        self.inner.write().await.update_playlist(&playlist)
    }
}