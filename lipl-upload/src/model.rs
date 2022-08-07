use serde::{Deserialize, Serialize};
use crate::text::StringExt;
use crate::error::UploadError;
use futures::TryStream;
use futures::stream::iter;
use crate::{fs, UploadResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct Summary {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Lyric {
    pub id: String,
    pub title: String,
    pub parts: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LyricPost {
    pub title: String,
    pub parts: Vec<Vec<String>>,
}

impl From<fs::Entry> for LyricPost {
    fn from(entry: fs::Entry) -> Self {
        let title = entry
            .path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let parts = entry.contents.to_parts();
        LyricPost {
            title,
            parts,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaylistPost {
    pub title: String,
    pub members: Vec<String>,
}


pub fn try_iter<T>(v: Vec<T>) -> impl TryStream<Ok=T, Error=UploadError> {
    iter(
        v
        .into_iter()
        .map(UploadResult::Ok)
    )
}
