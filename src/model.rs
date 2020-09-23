use serde::{Serialize, Deserialize};
use parking_lot::{RwLock};
use std::collections::{BTreeMap};
use std::sync::{Arc};

type Lyrics = BTreeMap<i32, lipl_data_disk::Lyric>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LyricSummary {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lyric {
    pub id: i32,
    pub title: String,
    pub parts: Vec<Vec<String>>
}

#[derive(Clone)]
pub struct Store {
    lyric_list: Arc<RwLock<Lyrics>>,
}

impl Store {
    pub fn from(list: impl std::iter::Iterator<Item = lipl_data_disk::Lyric>) -> Self {
        Store {
            lyric_list: Arc::new(
                RwLock::new(
                    (1..)
                    .zip(list)
                    .collect()
                )
            ),
        }
    }

    pub fn get_summaries(&self) -> Vec<LyricSummary> {
        self
        .lyric_list
        .read()
        .iter()
        .map(|(id, lyric)| LyricSummary { id: id.clone(), title: format!("{}", lyric.title.to_string_lossy()) })
        .collect()
    }

    pub fn get_lyric(&self, id: i32) -> Option<Lyric> {
        self
        .lyric_list
        .read()
        .get(&id)
        .map(|l| Lyric {
            title: l.title.to_string_lossy().to_string(), 
            parts: l.parts.clone(), 
            id,
        })
    }

    pub fn add_lyric(&self, parts: Vec<Vec<String>>, title: String) -> Option<Lyric> {
        let mut lock = self.lyric_list.write();
        let key = lock.keys().cloned().last().unwrap_or_default() + 1;
        lock.insert(key, lipl_data_disk::Lyric { parts: parts, title: std::ffi::OsString::from(title)})
        .map_or_else(
            |     | None,
            |lyric| Some(Lyric { id: key, parts: lyric.parts, title: lyric.title.to_string_lossy().to_string() })
        )
    }
}
