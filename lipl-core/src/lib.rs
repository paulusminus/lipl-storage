use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use anyhow::Result;
use async_trait::{async_trait};
use serde::{Deserialize, Serialize};
pub use crate::uuid::Uuid;
pub use path_ext::{PathExt};
pub use error::ModelError;

mod disk_format;
pub mod error;
pub mod ext;
mod path_ext;
mod uuid;

#[async_trait]
pub trait LyricDb {
    type Error: std::error::Error;
    async fn lyric_list(&self) -> Result<Vec<Summary>, Self::Error>;
    async fn lyric_list_full(&self) -> Result<Vec<Lyric>, Self::Error>;
    async fn lyric_item(&self, uuid: Uuid) -> Result<Lyric, Self::Error>;
    async fn lyric_post(&self, lyric_post: LyricPost) -> Result<Lyric, Self::Error>;
    async fn lyric_delete(&self, uuid: Uuid) -> Result<(), Self::Error>;
    async fn lyric_put(&self, uuid: Uuid, lyric_post: LyricPost) -> Result<Lyric, Self::Error>;
}

#[async_trait]
pub trait PlaylistDb {
    type Error: std::error::Error;
    async fn playlist_list(&self) -> Result<Vec<Summary>, Self::Error>;
    async fn playlist_list_full(&self) -> Result<Vec<Playlist>, Self::Error>;
    async fn playlist_item(&self, uuid: Uuid) -> Result<Playlist, Self::Error>;
    async fn playlist_post(&self, playlist_post: PlaylistPost) -> Result<Playlist, Self::Error>;
    async fn playlist_delete(&self, uuid: Uuid) -> Result<(), Self::Error>;
    async fn playlist_put(&self, uuid: Uuid, playlist_post: PlaylistPost) -> Result<Playlist, Self::Error>;
}

#[async_trait]
pub trait LiplRepo: Clone + Send + Sync {
    type Error: std::error::Error;
    async fn get_lyrics(&self) -> Result<Vec<Lyric>, Self::Error>;
    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>, Self::Error>;
    async fn get_lyric(&self, id: Uuid) -> Result<Lyric, Self::Error>;
    async fn post_lyric(&self, lyric: Lyric) -> Result<Lyric, Self::Error>;
    async fn delete_lyric(&self, id: Uuid) -> Result<(), Self::Error>;
    async fn get_playlists(&self) -> Result<Vec<Playlist>, Self::Error>;
    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>, Self::Error>;
    async fn get_playlist(&self, id: Uuid) -> Result<Playlist, Self::Error>;
    async fn post_playlist(&self, playlist: Playlist) -> Result<Playlist, Self::Error>;
    async fn delete_playlist(&self, id: Uuid) -> Result<(), Self::Error>;
    async fn stop(&self) -> Result<(), Self::Error>;
}

pub trait HasSummary {
    fn summary(&self) -> Summary;
}

pub trait Without<T>
where
    T: PartialEq,
{
    fn without(self, t: &T) -> Self;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Lyric {
    pub id: Uuid,
    pub title: String,
    pub parts: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LyricPost {
    pub title: String,
    pub parts: Vec<Vec<String>>,
}

impl Default for LyricPost {
    fn default() -> Self {
        LyricPost {
            title: "".to_owned(),
            parts: vec![],
        }
    }
}

impl From<(LyricPost, Uuid)> for Lyric {
    fn from(tuple: (LyricPost, Uuid)) -> Self {
        Lyric {
            id: tuple.1,
            title: tuple.0.title,
            parts: tuple.0.parts,
        }
    }
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

impl From<Lyric> for LyricPost {
    fn from(lyric: Lyric) -> Self {
        Self { title: lyric.title, parts: lyric.parts }
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

impl From<(PlaylistPost, Uuid)> for Playlist {
    fn from(tuple: (PlaylistPost, Uuid)) -> Self {
        Playlist {
            id: tuple.1,
            title: tuple.0.title,
            members: tuple.0.members,
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

impl From<(LyricMeta, Uuid)> for Summary {
    fn from(tuple: (LyricMeta, Uuid)) -> Self {
        Summary {
            id: tuple.1,
            title: tuple.0.title
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

pub fn summary<T>(t: &T) -> Summary
where
    T: HasSummary,
{
    t.summary()
}

pub fn summaries<T>(list: Vec<T>) -> Vec<Summary>
where
    T: HasSummary,
{
    list.iter().map(summary).collect()
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

impl<T> Without<T> for Vec<T>
where
    T: PartialEq,
{
    fn without(self, t: &T) -> Self {
        self.into_iter().filter(|s| s != t).collect()
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

#[derive(Debug, Serialize)]
pub struct RepoDb {
    pub lyrics: Vec<Lyric>,
    pub playlists: Vec<Playlist>,
}

impl RepoDb {
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

#[cfg(test)]
mod tests {

    #[test]
    fn without() {
        use super::Without;
        let v = vec!["1", "2", "5"];
        let out = v.without(&"2");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0], "1");
        assert_eq!(out[1], "5");
    }
}

pub mod reexport {
    pub use uuid::Uuid;
}