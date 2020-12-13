use std::fmt;
use serde::{Deserialize, Serialize};
use crate::model::{serde_uuid, HasId, HasSummary, Summary, Uuid, UuidExt};

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
            self.title.as_ref().unwrap_or(&"<< !! >>".to_owned()),
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
        (None, lp).into()
    }
}

impl From<(Option<Uuid>, LyricPost)> for Lyric {
    fn from(data: (Option<Uuid>, LyricPost)) -> Lyric {
        Lyric {
            id: data.0.unwrap_or(Uuid::new_v4()),
            title: data.1.title,
            parts: data.1.parts,
        }
    }
}

pub fn parts_to_string(parts: &[Vec<String>]) -> String {
    parts
    .iter()
    .map(|part| part.join("\n"))
    .collect::<Vec<String>>()
    .join("\n\n")
}

