use std::fmt::{Display, Formatter, Result as FmtResult};
use async_trait::{async_trait};
use serde::{Deserialize, Serialize};
pub use uuid_wrapper::Uuid;
pub use path_ext::{PathExt};
pub use error::RepoError;

mod disk_format;
mod error;
mod path_ext;
pub mod request;
mod uuid_wrapper;

pub type RepoResult<T> = Result<T, error::RepoError>;

#[async_trait]
pub trait LiplRepo {
    async fn get_lyrics(&self) -> RepoResult<Vec<Lyric>>;
    async fn get_lyric_summaries(&self) -> RepoResult<Vec<Summary>>;
    async fn get_lyric(&self, id: Uuid) -> RepoResult<Lyric>;
    async fn post_lyric(&self, lyric: Lyric) -> RepoResult<Lyric>;
    async fn delete_lyric(&self, id: Uuid) -> RepoResult<()>;
    async fn get_playlists(&self) -> RepoResult<Vec<Playlist>>;
    async fn get_playlist_summaries(&self) -> RepoResult<Vec<Summary>>;
    async fn get_playlist(&self, id: Uuid) -> RepoResult<Playlist>;
    async fn post_playlist(&self, playlist: Playlist) -> RepoResult<Playlist>;
    async fn delete_playlist(&self, id: Uuid) -> RepoResult<()>;
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

#[derive(Clone, Debug, Serialize)]
pub struct Lyric {
    pub id: Uuid,
    pub title: String,
    pub parts: Vec<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub members: Vec<Uuid>,
}

impl Display for Playlist {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}, [{}]", self.id, self.title, self.members.iter().map(|uuid| uuid.to_string()).collect::<Vec<_>>().join(", "))
    }
}

impl HasSummary for Playlist {
    fn summary(&self) -> Summary {
        Summary {
            id: self.id.clone(),
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

#[derive(Clone, Deserialize, Serialize)]
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
