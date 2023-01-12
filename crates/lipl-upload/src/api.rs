use async_trait::async_trait;
use futures::{TryFutureExt};
use lipl_core::{Lyric, LyricPost, Playlist, PlaylistPost, Summary, Uuid};
use rest_api_client::{ApiClient, ApiRequest};

use crate::{Result};

#[async_trait]
pub trait Api {
    async fn lyric_summaries(&self) -> Result<Vec<Summary>>;
    async fn lyric_delete(&self, id: Uuid) -> Result<()>;
    async fn lyric_insert(&self, lyric_post: LyricPost) -> Result<Lyric>;
    async fn playlist_summaries(&self) -> Result<Vec<Summary>>;
    async fn playlist_delete(&self, id: Uuid) -> Result<()>;
    async fn playlist_insert(&self, playlist_post: PlaylistPost) -> Result<Playlist>;
}

pub struct UploadClient {
    inner: ApiClient,
}

impl From<ApiClient> for UploadClient {
    fn from(api_client: ApiClient) -> Self {
        Self {
            inner: api_client
        }
    }
}

#[async_trait]
impl Api for UploadClient {
    async fn lyric_summaries(&self) -> Result<Vec<Summary>> {
        self.inner.get("lyric")
        .err_into()
        .await
    }

    async fn lyric_delete(&self, id: Uuid) -> Result<()> {
        self.inner.delete(&format!("lyric/{}", id))
        .err_into()
        .await
    }

    async fn lyric_insert(&self, lyric_post: LyricPost) -> Result<Lyric> {
        self.inner.post("lyric", lyric_post)
        .err_into()
        .await
    }

    async fn playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.inner.get("playlist")
        .err_into()
        .await
    }

    async fn playlist_delete(&self, id: Uuid) -> Result<()> {
        self.inner.delete(&format!("playlist/{}", id))
        .err_into()
        .await
    }

    async fn playlist_insert(&self, playlist_post: PlaylistPost) -> Result<Playlist> {
        self.inner.post("playlist", playlist_post)
        .err_into()
        .await
    }
}
