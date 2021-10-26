use std::fmt;
use serde::{Deserialize, Serialize};
use crate::model::{HasId, HasSummary, Summary, Uuid};

#[derive(Clone, Deserialize, Serialize)]
pub struct Lyric {
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
            self.id,
        )
    }
}

impl HasId for Lyric {
    fn id(&self) -> Uuid {
        self.id.clone()
    }
}

impl HasSummary for Lyric {
    fn to_summary(&self) -> Summary {
        Summary {
            id: self.id.clone(),
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
            id: data.0.unwrap_or_default(),
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_lyric_clone() {
        use super::Uuid;
        use super::Lyric;
        
        const CHRISTMAS: &'static str = "And so this is Christmas";
        const TITLE1: &'static str = "Whatever";
        const TITLE2: &'static str = "JaJa";
        let uuid1 = Uuid::default();
        let title1 = Some(TITLE1.to_owned());
        let parts1: Vec<Vec<String>> = vec![vec![CHRISTMAS.to_owned()]]; 
        let lyric1 = Lyric {
            id: uuid1.clone(),
            title: title1.clone(),
            parts: parts1.clone(),
        };
        assert_eq!(lyric1.id, uuid1);
        assert_eq!(lyric1.title, title1);
        assert_eq!(lyric1.parts, parts1);

        let mut lyric2 = (&lyric1).clone();
        let uuid2 = Uuid::default();
        let title2 = Some(TITLE2.to_owned());
        lyric2.title = title2;
        lyric2.id = uuid2.clone();
        assert_eq!(lyric1.title, title1);
        assert_eq!(lyric2.title, Some(TITLE2.to_owned()));
        assert_eq!(lyric2.id, uuid2);
        assert_eq!(lyric1.id, uuid1);
        assert_eq!(lyric1.parts[0][0], CHRISTMAS.to_owned());
        assert_eq!(lyric2.parts[0][0], CHRISTMAS.to_owned());
    }
}