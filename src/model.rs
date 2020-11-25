use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::PathBufExt;
use crate::UuidExt;

pub struct Lyric {
    pub id: Uuid,
    pub title: Option<String>,
    pub parts: Vec<Vec<String>>,
}

impl Display for Lyric {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub members: Vec<Uuid>
}

impl Display for Playlist {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

pub struct Db {
    pub lyrics: HashMap<Uuid, Lyric>,
    pub playlists: HashMap<Uuid, Playlist>,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: Option<String>,
}
