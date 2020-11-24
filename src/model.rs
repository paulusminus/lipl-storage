use std::path::{PathBuf};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::PathBufExt;

pub struct Lyric {
    pub id: Uuid,
    pub title: Option<String>,
    pub parts: Vec<Vec<String>>,
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: Option<String>,
}
