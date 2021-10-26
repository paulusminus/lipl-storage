use std::fmt;
// use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::model::{HasId, HasSummary, Summary, Uuid};

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub members: Vec<Uuid>
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: Option<String>,
}

impl fmt::Display for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Playlist: {}, id = {}\n{}",
            self.title,
            self.id,
            self.members.iter().map(|m| format!("  - {}", m.to_string())).collect::<Vec<String>>().join("\n")
        )
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PlaylistPost {
    pub title: String,
    pub members: Vec<String>
}

impl From<(String, Vec<Uuid>)> for PlaylistPost {
    fn from(s: (String, Vec<Uuid>)) -> PlaylistPost {
        PlaylistPost {
            title: s.0,
            members: s.1.iter().map(|uuid| uuid.to_string()).collect()
        }
    }
}


impl From<PlaylistPost> for Playlist {
    fn from(pp: PlaylistPost) -> Playlist {
        (None, pp).into()
    }
}

impl HasId for Playlist {
    fn id(&self) -> Uuid {
        self.id.clone()
    }
}

impl HasSummary for Playlist {
    fn to_summary(&self) -> Summary {
        Summary {
            id: self.id.clone(),
            title: Some(self.title.clone()),
        }
    }
}

impl From<(Option<Uuid>, PlaylistPost)> for Playlist {
    fn from(data: (Option<Uuid>, PlaylistPost)) -> Playlist {
        Playlist {
            id: data.0.unwrap_or_default(),
            title: data.1.title,
            members: data.1.members.iter().map(|m| m.parse::<Uuid>().unwrap()).collect()
        }
    }
}
