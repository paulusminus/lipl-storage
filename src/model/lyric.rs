use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::uuid_ext::{UuidExt};
use super::traits::{HasId, HasSummary};
use super::summary::Summary;
use super::serde_uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Lyric {
    #[serde(with = "serde_uuid")]
    pub id: Uuid,
    pub title: Option<String>,
    pub parts: Vec<Vec<String>>,
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

impl HasId for Lyric {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasSummary for Lyric {
    fn to_summary(&self) -> Summary {
        Summary {
            id: self.id,
            title: self.title.clone(),
        }
    }
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
