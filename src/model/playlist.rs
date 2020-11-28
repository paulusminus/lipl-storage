use std::fmt;
use std::path::PathBuf;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::{HasId, HasSummary, Summary, UuidExt, PathBufExt};
use super::serde_uuid;
use super::serde_vec_uuid;

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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PlaylistPost {
    pub title: String,
    pub members: Vec<String>
}

impl From<(String, Vec<Uuid>)> for PlaylistPost {
    fn from(s: (String, Vec<Uuid>)) -> PlaylistPost {
        PlaylistPost {
            title: s.0,
            members: s.1.iter().map(|uuid| uuid.to_base58()).collect()
        }
    }
}

impl From<PlaylistPost> for Playlist {
    fn from(pp: PlaylistPost) -> Playlist {
        Playlist {
            id: Uuid::new_v4(),
            title: pp.title,
            members: pp.members.iter().map(|s| s.as_str().to_uuid()).collect::<Vec<Uuid>>(),
        }
    }
}

impl HasId for Playlist {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasSummary for Playlist {
    fn to_summary(&self) -> Summary {
        Summary {
            id: self.id,
            title: Some(self.title.clone()),
        }
    }
}

impl From<(Uuid, PlaylistPost)> for Playlist {
    fn from(data: (Uuid, PlaylistPost)) -> Playlist {
        Playlist {
            id: data.0,
            title: data.1.title,
            members: data.1.members.iter().map(|m| PathBuf::from(m).to_uuid())
            .collect()
        }
    }
}
