use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use async_trait::{async_trait};
use serde::{Deserialize, Serialize};
pub use crate::uuid::Uuid;
pub use path_ext::{PathExt};
pub use error::Error;

#[cfg(feature = "postgres")]
pub use postgres_error::PostgresRepoError;

#[cfg(feature = "file")]
pub use file_error::FileRepoError;

mod disk_format;
pub mod error;
pub mod ext;
#[cfg(feature = "file")]
mod file_error;
mod path_ext;
#[cfg(feature = "postgres")]
mod postgres_error;
pub mod reexport;
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

pub trait HasSummary {
    fn summary(&self) -> Summary;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Lyric {
    pub id: Uuid,
    pub title: String,
    pub parts: Vec<Vec<String>>,
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



#[cfg(test)]
mod tests {

    #[test]
    fn without() {
        use super::ext::VecExt;
        let v = vec!["1", "2", "5"];
        let out = v.without(&"2");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0], "1");
        assert_eq!(out[1], "5");
    }
}
