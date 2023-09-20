/*!
 This crate is a depency library for other crates who wish to work with lyrics and playlists.

 The main trait is LiplRepo. Other crates implement this trait to hide implementation details for creating, reading, updating and deleting
 lyrics ands playlist from a store. All implementation except MemoryRepo implement a persistent store.

 MemoryRepo is usefull for testing perposes.
 
 */

use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::sync::Arc;
use std::cmp::Ordering;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
pub use crate::uuid::Uuid;
pub use error::{postgres_error, redis_error, Error};

mod disk_format;
pub mod error;
pub mod reexport;
#[cfg(feature = "transaction")]
pub mod transaction;
mod uuid;

pub type Result<T> = core::result::Result<T, Error>;

#[async_trait]
pub trait LiplRepo: Send + Sync {
    async fn get_lyrics(&self) -> Result<Vec<Lyric>>;
    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>>;
    async fn get_lyric(&self, id: Uuid) -> Result<Lyric>;
    async fn upsert_lyric(&self, lyric: Lyric) -> Result<Lyric>;
    async fn delete_lyric(&self, id: Uuid) -> Result<()>;
    async fn get_playlists(&self) -> Result<Vec<Playlist>>;
    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>>;
    async fn get_playlist(&self, id: Uuid) -> Result<Playlist>;
    async fn upsert_playlist(&self, playlist: Playlist) -> Result<Playlist>;
    async fn delete_playlist(&self, id: Uuid) -> Result<()>;
    async fn stop(&self) -> Result<()>;
}

#[async_trait]
pub trait ToRepo {
    async fn to_repo(self) -> Result<Arc<dyn LiplRepo>>;
}

pub trait HasSummary {
    fn summary(&self) -> Summary;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Lyric {
    pub id: Uuid,
    pub title: String,
    pub parts: Vec<Vec<String>>,
}

impl HasSummary for Lyric {
    fn summary(&self) -> Summary {
        Summary {
            id: self.id,
            title: self.title.clone(),
        }
    }
}

pub fn by_title<T>(a: &T, b: &T) -> Ordering 
where
    T: HasSummary,
{
    a.summary().title.cmp(&b.summary().title)
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LyricPost {
    pub title: String,
    pub parts: Vec<Vec<String>>,
}

impl From<(Option<Uuid>, LyricPost)> for Lyric {
    fn from(data: (Option<Uuid>, LyricPost)) -> Lyric {
        Lyric {
            id: data.0.unwrap_or_default(),
            title: data.1.title,
            parts: data.1.parts,
        }
    }
}

impl From<LyricPost> for Lyric {
    fn from(lyric_post: LyricPost) -> Self {
        Self {
            id: Default::default(),
            title: lyric_post.title,
            parts: lyric_post.parts,
        }
    }
}

impl From<Lyric> for LyricPost {
    fn from(lyric: Lyric) -> Self {
        Self { title: lyric.title, parts: lyric.parts }
    }
}

impl From<(&str, &str)> for LyricPost {
    fn from(value: (&str, &str)) -> Self {
        Self {
            title: value.0.to_owned(),
            parts: parts::to_parts(value.1.to_owned()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub members: Vec<Uuid>,
}

impl HasSummary for Playlist {
    fn summary(&self) -> Summary {
        Summary {
            id: self.id,
            title: self.title.clone(),
        }
    }
}

impl From<PlaylistPost> for Playlist {
    fn from(pp: PlaylistPost) -> Playlist {
        (None, pp).into()
    }
}

impl From<(Option<Uuid>, PlaylistPost)> for Playlist {
    fn from(data: (Option<Uuid>, PlaylistPost)) -> Playlist {
        Playlist {
            id: data.0.unwrap_or_default(),
            title: data.1.title,
            members: data.1.members,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PlaylistPost {
    pub title: String,
    pub members: Vec<Uuid>,
}

impl From<Playlist> for PlaylistPost {
    fn from(p: Playlist) -> Self {
        PlaylistPost {
            title: p.title,
            members: p.members,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Summary {
    pub id: Uuid,
    pub title: String,
}

impl From<(Option<Uuid>, LyricMeta)> for Summary {
    fn from(tuple: (Option<Uuid>, LyricMeta)) -> Self {
        Summary {
            id: tuple.0.unwrap_or_default(),
            title: tuple.1.title
        }
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}", self.id, self.title)
    }
}

impl HasSummary for Summary {
    fn summary(&self) -> Summary {
        self.clone()
    }
}

pub fn to_summary<T>(t: &T) -> Summary
where
    T: HasSummary,
{
    t.summary()
}

pub fn to_summaries<T>(list: Vec<T>) -> Vec<Summary>
where
    T: HasSummary,
{
    list.iter().map(to_summary).collect()
}

pub fn ids<T>(list: impl Iterator<Item=T>) -> Vec<Uuid>
where
    T: HasSummary,
{
    list.map(|s| s.summary().id).collect()
}

#[derive(Deserialize, Serialize)]
pub struct LyricMeta {
    pub title: String,
    pub hash: Option<String>,
}

impl From<&Lyric> for LyricMeta {
    fn from(l: &Lyric) -> Self {
        LyricMeta {
            title: l.title.clone(),
            hash: l.etag()
        }
    }
}

pub trait Etag {
    fn etag(&self) -> Option<String>;
}

impl<T: Serialize> Etag for T {
    fn etag(&self) -> Option<String> {
        bincode::serialize(self)
        .map(|bytes| etag::EntityTag::const_from_data(&bytes))
        .map(|etag| etag.to_string())
        .ok()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepoDb {
    pub lyrics: Vec<Lyric>,
    pub playlists: Vec<Playlist>,
}

impl From<(Vec<Lyric>, Vec<Playlist>)> for RepoDb {
    fn from(tuple: (Vec<Lyric>, Vec<Playlist>)) -> Self {
        Self {
            lyrics: tuple.0,
            playlists: tuple.1
        }
    }
}

impl RepoDb {
    pub fn find_lyric_by_title(&self, title: &str) -> Option<Lyric> {
        self.lyrics.iter().find(|lyric| lyric.title == *title).cloned()
    }

    pub fn to_yaml(&self) -> Result<String> {
        let s = serde_yaml::to_string(self)?;
        Ok(s)
    }
}

impl std::fmt::Display for RepoDb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let lyrics = 
            core::iter::once("Lyrics:".to_owned())
            .chain(
                self
                .lyrics
                .iter()
                .map(|lyric| 
                    format!(" - {}, {} parts", lyric.title, lyric.parts.len()
                )
            )
            .chain(
                core::iter::once("".to_owned()),
            )
            .chain(
                core::iter::once(
                    "Playlists:".to_owned(),
                )
            )
            .chain(
                self
                .playlists
                .iter()
                .map(|playlist| 
                    format!(" - {}", playlist.title)
                )
            )
        );
        write!(f, "{}", lyrics.collect::<Vec<_>>().join("\n"))
    }
}

pub trait Yaml {
    fn load<R>(r: R) -> Result<Self> where R: std::io::Read, Self: Sized;
    fn save<W>(&self, w: W) -> Result<()> where W: std::io::Write;
}
