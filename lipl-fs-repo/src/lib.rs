use std::hash::{Hash};
use std::fmt::{Display, Formatter, Result as FmtResult};
use anyhow::{anyhow};
pub use anyhow::{Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use futures::{Future};

mod fs;
mod io;

pub trait HasSummary {
    fn summary(&self) -> Summary;
}

#[async_trait]
pub trait LiplRepo {
    async fn get_lyrics(&self) -> Result<Vec<Lyric>>;
    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>>;
    async fn get_lyric(&self, id: String) -> Result<Lyric>;
    async fn post_lyric(&self, lyric: Lyric) -> Result<()>;
    async fn delete_lyric(&self, id: String) -> Result<()>;
    async fn get_playlists(&self) -> Result<Vec<Playlist>>;
    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>>;
    async fn get_playlist(&self, id: String) -> Result<Playlist>;
    async fn post_playlist(&self, playlist: Playlist) -> Result<()>;
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
        if !fs::is_dir(s) { return Err(anyhow!("cannot find directory {}", s)) }

        Ok(
            FileSystem {
                source_dir: s,
                playlist_extension,
                lyric_extension,
                lock: tokio::sync::RwLock::new(())
            }    
        )
    }
}

#[async_trait]
impl<'a> LiplRepo for FileSystem<'a> {
    async fn get_lyrics(&self) -> Result<Vec<Lyric>> {
        self.lock.read().await;
        crate::io::get_list(self.source_dir, self.lyric_extension, crate::io::get_lyric).await
    }

    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        self.lock.read().await;
        crate::io::get_list(self.source_dir, self.lyric_extension, crate::io::get_lyric_summary).await
    }

    async fn get_lyric(&self, id: String) -> Result<Lyric> {
        self.lock.read().await;
        crate::io::get_lyric(
            fs::full_path(self.source_dir, &id, self.lyric_extension)
        )
        .await
    }

    async fn post_lyric(&self, lyric: Lyric) -> Result<()> {
        self.lock.write().await;
        crate::io::post_lyric(
            fs::full_path(self.source_dir, &lyric.id, self.lyric_extension),
            lyric,
        )
        .await
    }

    async fn delete_lyric(&self, id: String) -> Result<()> {
        self.lock.write().await;
        crate::io::delete_file(
            fs::full_path(self.source_dir, &id, self.lyric_extension)
        )
        .await?;
        let playlists = self.get_playlists().await?;
        for mut playlist in playlists {
            if playlist.members.contains(&id) {
                playlist.members = playlist.members.into_iter().filter(|m| *m != id).collect();
                self.post_playlist(playlist).await?;
            }
        }
        Ok(())
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        self.lock.read().await;
        crate::io::get_list(self.source_dir, self.playlist_extension, crate::io::get_playlist).await
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.lock.read().await;
        self.get_playlists()
        .await
        .map(
            |playlists| playlists.iter().map(to_summary).collect::<Vec<Summary>>()
        )
    }

    async fn get_playlist(&self, id: String) -> Result<Playlist> {
        self.lock.read().await;
        crate::io::get_playlist(
            fs::full_path(self.source_dir, &id, self.playlist_extension)
        )
        .await
    }

    async fn post_playlist(&self, playlist: Playlist) -> Result<()> {
        self.lock.write().await;
        let lyric_ids: Vec<String> = self.get_lyric_summaries().await?.into_iter().map(|s| s.id).collect();
        for member in playlist.members.iter() {
            if !lyric_ids.contains(member) {
                return Err(anyhow!("Playlist {} contains invalid member", playlist.id));
            }
        }
        crate::io::post_playlist(
            fs::full_path(self.source_dir, &playlist.id, self.playlist_extension),
            playlist,
        ).await
    }

    async fn delete_playlist(&self, id: String) -> Result<()> {
        self.lock.write().await;
        crate::io::delete_file(
            fs::full_path(self.source_dir, &id, self.playlist_extension)
        )
        .await
    }
}


#[derive(Deserialize, Serialize)]
pub struct YamlMeta {
    pub title: String,
    pub hash: Option<u64>,
}

pub struct Summary {
    pub id: String,
    pub title: String,
}

impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}", self.id, self.title)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlaylistPost {
    pub title: String,
    pub members: Vec<String>,
}

#[derive(Deserialize)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub members: Vec<String>,
}

impl HasSummary for Playlist {
    fn summary(&self) -> Summary {
        Summary {
            id: self.id.clone(),
            title: self.title.clone(),
        }
    }
}

impl Display for Playlist {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}, [{}]", self.id, self.title, self.members.join(", "))
    }
}

#[derive(Debug, Hash)]
pub struct Lyric {
    pub id: String,
    pub title: String,
    pub parts: Vec<Vec<String>>
}

pub struct Timer(pub std::time::Instant);

impl Timer {
    fn new() -> Self {
        Timer(std::time::Instant::now())
    }

    fn report_elapsed(&self) {
        println!("Elapsed: {} milliseconds", self.0.elapsed().as_millis());
    }
}


pub fn to_std_output(s: impl std::fmt::Display) {
    println!("{}", s);
}

fn to_summary<T>(t: &T) -> Summary where T: HasSummary {
    t.summary()
}

pub async fn time_it<T, F, O>(process: F) -> Result<T> 
where 
    F: Fn() -> O,
    O: Future<Output=Result<T>>
{
    let timer = Timer::new();
    let result = process().await?;
    timer.report_elapsed();
    Ok(result)
}

