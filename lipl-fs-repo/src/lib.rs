use std::path::{PathBuf};

use async_trait::{async_trait};

use fs::{IO};
use lipl_types::{LiplRepo, Lyric, Playlist, PlaylistPost, RepoResult, Summary, Without, Uuid, RepoError};

// mod disk_format;
pub mod elapsed;
// pub mod error;
mod fs;
mod io;
// pub mod model;


// #[async_trait]
// pub trait LiplRepo {
//     async fn get_lyrics(&self) -> RepoResult<Vec<Lyric>>;
//     async fn get_lyric_summaries(&self) -> RepoResult<Vec<Summary>>;
//     async fn get_lyric(&self, id: String) -> RepoResult<Lyric>;
//     async fn post_lyric(&self, lyric: Lyric) -> RepoResult<()>;
//     async fn delete_lyric(&self, id: String) -> RepoResult<()>;
//     async fn get_playlists(&self) -> RepoResult<Vec<Playlist>>;
//     async fn get_playlist_summaries(&self) -> RepoResult<Vec<Summary>>;
//     async fn get_playlist(&self, id: String) -> RepoResult<Playlist>;
//     async fn post_playlist(&self, playlist: Playlist) -> RepoResult<()>;
//     async fn delete_playlist(&self, id: String) -> RepoResult<()>;
// }

pub struct FileSystem<'a> {
    source_dir: &'a str,
    playlist_extension: &'a str,
    lyric_extension: &'a str,
    lock: tokio::sync::RwLock<()>
}

impl<'a> FileSystem<'a> {
    pub fn new(s: &'a str, playlist_extension: &'a str, lyric_extension: &'a str) -> RepoResult<Self> {
        s.is_dir()?;

        Ok(
            FileSystem {
                source_dir: s,
                playlist_extension,
                lyric_extension,
                lock: tokio::sync::RwLock::new(())
            }    
        )
    }

    fn playlist_path(&self, id: &Uuid) -> PathBuf {
        self.source_dir.full_path(&id.to_string(), self.playlist_extension)
    }

    fn lyric_path(&self, id: &Uuid) -> PathBuf {
        self.source_dir.full_path(&id.to_string(), self.lyric_extension)
    }
}

#[async_trait]
impl<'a> LiplRepo for FileSystem<'a> {
    async fn get_lyrics(&self) -> RepoResult<Vec<Lyric>> {
        self.lock.read().await;
        io::get_list(
            self.source_dir,
            self.lyric_extension,io::get_lyric,
        )
        .await
    }

    async fn get_lyric_summaries(&self) -> RepoResult<Vec<Summary>> {
        self.lock.read().await;
        io::get_list(
            self.source_dir,
            self.lyric_extension,
            io::get_lyric_summary,
        )
        .await
    }

    async fn get_lyric(&self, id: Uuid) -> RepoResult<Lyric> {
        self.lock.read().await;
        io::get_lyric(
            self.lyric_path(&id)
        )
        .await
    }

    async fn post_lyric(&self, lyric: Lyric) -> RepoResult<()> {
        self.lock.write().await;
        io::post_item(
            self.lyric_path(&lyric.id),
            lyric,
        )
        .await
    }

    async fn delete_lyric(&self, id: Uuid) -> RepoResult<()> {
        self.lock.write().await;
        self.lyric_path(&id).remove().await?;
        for mut playlist in self.get_playlists().await? {
            if playlist.members.contains(&id) {
                playlist.members = playlist.members.without(&id);
                self.post_playlist(playlist).await?;
            }
        }
        Ok(())
    }

    async fn get_playlists(&self) -> RepoResult<Vec<Playlist>> {
        self.lock.read().await;
        io::get_list(self.source_dir, self.playlist_extension, io::get_playlist).await
    }

    async fn get_playlist_summaries(&self) -> RepoResult<Vec<Summary>> {
        self.lock.read().await;
        self.get_playlists()
        .await
        .map(lipl_types::summaries)
    }

    async fn get_playlist(&self, id: Uuid) -> RepoResult<Playlist> {
        self.lock.read().await;
        io::get_playlist(
            self.playlist_path(&id),
        )
        .await
    }

    async fn post_playlist(&self, playlist: Playlist) -> RepoResult<()> {
        self.lock.write().await;
        let lyric_ids: Vec<Uuid> = lipl_types::ids(self.get_lyric_summaries().await?.into_iter());
        for member in playlist.members.iter() {
            if !lyric_ids.contains(member) {
                return Err(RepoError::PlaylistInvalidMember(playlist.id.to_string(), member.to_string()));
            }
        }
        io::post_item(
            self.playlist_path(&playlist.id),
            PlaylistPost::from(playlist),
        ).await
    }

    async fn delete_playlist(&self, id: Uuid) -> RepoResult<()> {
        self.lock.write().await;
        self.playlist_path(&id).remove().await
    }
}
