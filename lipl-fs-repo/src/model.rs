use std::collections::hash_map::{DefaultHasher};
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Deserialize, Serialize};
pub use anyhow::{Error, Result};

pub trait HasSummary {
    fn summary(&self) -> Summary;
}

pub trait Without<T> where T: PartialEq {
    fn without(self, t: &T) -> Self;
}

#[derive(Clone, Debug, Hash)]
pub struct Lyric {
    pub id: String,
    pub title: String,
    pub parts: Vec<Vec<String>>,
}

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

impl From<(LyricPost, String)> for Lyric {
    fn from(tuple: (LyricPost, String)) -> Self {
        Lyric {
            id: tuple.1,
            title: tuple.0.title,
            parts: tuple.0.parts,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub members: Vec<String>,
}

impl Display for Playlist {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}, [{}]", self.id, self.title, self.members.join(", "))
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

impl From<(PlaylistPost, String)> for Playlist {
    fn from(tuple: (PlaylistPost, String)) -> Self {
        Playlist {
            id: tuple.1,
            title: tuple.0.title,
            members: tuple.0.members,
        }
    } 
}


#[derive(Deserialize, Serialize)]
pub struct PlaylistPost {
    pub title: String,
    pub members: Vec<String>,
}

impl From<Playlist> for PlaylistPost {
    fn from(p: Playlist) -> Self {
        PlaylistPost {
            title: p.title,
            members: p.members,
        }
    }
}

pub struct Summary {
    pub id: String,
    pub title: String,
}

impl From<(LyricMeta, String)> for Summary {
    fn from(tuple: (LyricMeta, String)) -> Self {
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

pub fn summary<T>(t: &T) -> Summary
where
    T: HasSummary
{
    t.summary()
}

pub fn summaries<T>(list: Vec<T>) -> Vec<Summary>
where
    T: HasSummary
{
    list.iter().map(summary).collect()
}

pub fn ids(list: Vec<Summary>) -> Vec<String>
{
    list.iter().map(|s| s.id.clone()).collect()
}

#[derive(Deserialize, Serialize)]
pub struct LyricMeta {
    pub title: String,
    pub hash: Option<u64>,
}

impl From<&Lyric> for LyricMeta {
    fn from(l: &Lyric) -> Self {
        LyricMeta {
            title: l.title.clone(),
            hash: Some(calculate_hash(l))
        }
    }
}

impl<T> Without<T> for Vec<T> where T: PartialEq {
    fn without(self, t: &T) -> Self {
        self.into_iter().filter(|s| s != t).collect()
    }
}

pub fn calculate_hash<T>(obj: T) -> u64 where T: Hash {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test {

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