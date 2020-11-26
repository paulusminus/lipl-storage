use std::fmt;
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::PathBufExt;
use crate::UuidExt;

mod serde_uuid;
mod serde_vec_uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Lyric {
    #[serde(with = "serde_uuid")]
    pub id: Uuid,
    pub title: Option<String>,
    pub parts: Vec<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct LyricPost {
    pub title: Option<String>,
    pub parts: Vec<Vec<String>>,
}

impl From<LyricPost> for Lyric {
    fn from(lp: LyricPost) -> Lyric {
        Lyric {
            id: Uuid::new_v4(),
            title: lp.title,
            parts: lp.parts,
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct Summary {
    #[serde(with = "serde_uuid")]
    pub id: Uuid,
    pub title: Option<String>,
}

impl Lyric {
    pub fn to_summary(&self) -> Summary {
        Summary {
            id: self.id,
            title: self.title.clone(),
        }
    }
}

impl fmt::Display for Lyric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lyric: {}, {} parts, id = {}",
            self.title.as_ref().unwrap_or(&"<< onbekend >>".to_owned()),
            self.parts.len(),
            self.id.to_base58()
        )
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DiskPlaylist {
    pub title: String,
    pub members: Vec<String>
}

impl From<(String, Vec<Uuid>)> for DiskPlaylist {
    fn from(s: (String, Vec<Uuid>)) -> DiskPlaylist {
        DiskPlaylist {
            title: s.0,
            members: s.1.iter().map(|uuid| uuid.to_base58()).collect()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    #[serde(with = "serde_uuid")]
    pub id: Uuid,
    pub title: String,
    #[serde(with = "serde_vec_uuid")]
    pub members: Vec<Uuid>
}

impl fmt::Display for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Playlist: {}, id = {}\n{}",
            self.title,
            self.id.to_base58(),
            self.members.iter().map(|m| format!("  - {}", m.to_base58())).collect::<Vec<String>>().join("\n")
        )
    }
}

pub trait HasId {
    fn id(&self) -> Uuid;
}

impl HasId for Lyric {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasId for Playlist {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl From<(Uuid, DiskPlaylist)> for Playlist {
    fn from(data: (Uuid, DiskPlaylist)) -> Playlist {
        Playlist {
            id: data.0,
            title: data.1.title,
            members: data.1.members.iter().map(|m| PathBuf::from(m).to_uuid())
            .collect()
        }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: Option<String>,
}
