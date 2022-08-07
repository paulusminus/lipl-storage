use async_trait::async_trait;
use crate::model::{Lyric, LyricPost, Playlist, PlaylistPost, Summary};
use crate::UploadResult;
use crate::client::UploadClient;


#[async_trait]
pub trait Api {
    async fn lyric_summaries(&self) -> UploadResult<Vec<Summary>>;
    async fn lyric_delete(&self, id: String) -> UploadResult<()>;
    async fn lyric_insert(&self, lyric_post: LyricPost) -> UploadResult<Lyric>;
    async fn playlist_summaries(&self) -> UploadResult<Vec<Summary>>;
    async fn playlist_delete(&self, id: String) -> UploadResult<()>;
    async fn playlist_insert(&self, playlist_post: PlaylistPost) -> UploadResult<Playlist>;
}

#[async_trait]
impl Api for UploadClient {
    async fn lyric_summaries(&self) -> UploadResult<Vec<Summary>> {
        self.get_object("lyric").await
    }

    async fn lyric_delete(&self, id: String) -> UploadResult<()> {
        self.delete_object(&format!("lyric/{}", id)).await
    }

    async fn lyric_insert(&self, lyric_post: LyricPost) -> UploadResult<Lyric> {
        self.insert_object("lyric", lyric_post).await
    }

    async fn playlist_summaries(&self) -> UploadResult<Vec<Summary>> {
        self.get_object("playlist").await
    }

    async fn playlist_delete(&self, id: String) -> UploadResult<()> {
        self.delete_object(&format!("playlist/{}", id)).await
    }

    async fn playlist_insert(&self, playlist_post: PlaylistPost) -> UploadResult<Playlist> {
        self.insert_object("playlist", playlist_post).await
    }
}
