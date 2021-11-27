use std::path::{PathBuf};

use anyhow::{anyhow};
use model::{Error, Result, Lyric, Playlist, PlaylistPost, Summary, Without};
use async_trait::{async_trait};

mod disk_format;
pub mod elapsed;
mod fs;
mod io;
pub mod model;

#[async_trait]
pub trait LiplRepo {
    async fn get_lyrics(&self) -> Result<Vec<Lyric>>;
    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>>;
    async fn get_lyric(&self, id: String) -> Result<Lyric>;
    async fn post_lyric(&self, lyric: model::Lyric) -> Result<()>;
    async fn delete_lyric(&self, id: String) -> Result<()>;
    async fn get_playlists(&self) -> Result<Vec<Playlist>>;
    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>>;
    async fn get_playlist(&self, id: String) -> Result<Playlist>;
    async fn post_playlist(&self, playlist: model::Playlist) -> Result<()>;
    async fn delete_playlist(&self, id: String) -> Result<()>;
}

pub struct FileSystem<'a> {
    source_dir: &'a str,
    playlist_extension: &'a str,
    lyric_extension: &'a str,
    lock: tokio::sync::RwLock<()>
}

impl<'a> FileSystem<'a> {
    pub fn new(s: &'a str, playlist_extension: &'a str, lyric_extension: &'a str) -> Result<Self> {
        fs::is_dir(s)?;

        Ok(
            FileSystem {
                source_dir: s,
                playlist_extension,
                lyric_extension,
                lock: tokio::sync::RwLock::new(())
            }    
        )
    }

    fn playlist_path(&self, id: &String) -> PathBuf {
        fs::full_path(self.source_dir, id, self.playlist_extension)
    }

    fn lyric_path(&self, id: &String) -> PathBuf {
        fs::full_path(self.source_dir, id, self.lyric_extension)
    }
}

#[async_trait]
impl<'a> LiplRepo for FileSystem<'a> {
    async fn get_lyrics(&self) -> Result<Vec<Lyric>> {
        self.lock.read().await;
        io::get_list(self.source_dir, self.lyric_extension, io::get_lyric).await
    }

    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        self.lock.read().await;
        io::get_list(self.source_dir, self.lyric_extension, io::get_lyric_summary).await
    }

    async fn get_lyric(&self, id: String) -> Result<Lyric> {
        self.lock.read().await;
        io::get_lyric(
            self.lyric_path(&id)
        )
        .await
    }

    async fn post_lyric(&self, lyric: Lyric) -> Result<()> {
        self.lock.write().await;
        io::post_item(
            self.lyric_path(&lyric.id),
            lyric,
        )
        .await
    }

    async fn delete_lyric(&self, id: String) -> Result<()> {
        self.lock.write().await;
        io::delete_file(
            self.lyric_path(&id)
        )
        .await?;
        let playlists = self.get_playlists().await?;
        for mut playlist in playlists {
            if playlist.members.contains(&id) {
                playlist.members = playlist.members.without(&id);
                self.post_playlist(playlist).await?;
            }
        }
        Ok(())
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        self.lock.read().await;
        io::get_list(self.source_dir, self.playlist_extension, io::get_playlist).await
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.lock.read().await;
        self.get_playlists()
        .await
        .map(model::summaries)
    }

    async fn get_playlist(&self, id: String) -> Result<Playlist> {
        self.lock.read().await;
        io::get_playlist(
            self.playlist_path(&id),
        )
        .await
    }

    async fn post_playlist(&self, playlist: model::Playlist) -> Result<()> {
        self.lock.write().await;
        let lyric_ids: Vec<String> = model::ids(self.get_lyric_summaries().await?);
        for member in playlist.members.iter() {
            if !lyric_ids.contains(member) {
                return Err(anyhow!("Playlist {} contains invalid member", playlist.id));
            }
        }
        io::post_item(
            self.playlist_path(&playlist.id),
            PlaylistPost::from(playlist),
        ).await
    }

    async fn delete_playlist(&self, id: String) -> Result<()> {
        self.lock.write().await;
        io::delete_file(
            self.playlist_path(&id),
        )
        .await
    }
}
