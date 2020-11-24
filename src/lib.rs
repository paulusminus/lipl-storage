use std::ffi::{OsStr};
use std::fs::{read_dir, File};
use std::io::{Error};
use std::path::{PathBuf};

use futures::future::ready;
use futures::io::{AllowStdIo, BufReader};
use futures::stream::{Stream, StreamExt, iter};
use bs58::decode;
use serde::{Deserialize, Serialize};

mod parts;
pub use parts::to_parts_async;

pub struct Lyric {
    pub id: String,
    pub title: Option<String>,
    pub member_of: Option<Vec<String>>,
    pub parts: Vec<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Frontmatter {
    title: Option<String>,
    member_of: Option<Vec<String>>
}

pub async fn get_file(pb: &PathBuf) -> Result<Lyric, Error> {
    let file = File::open(pb)?;
    let test = AllowStdIo::new(file);
    let reader = BufReader::new(test);
    let (yaml, parts) = to_parts_async(reader).await?;

    let parsed = yaml.and_then(|text| serde_yaml::from_str::<Frontmatter>(&text).ok());
    let frontmatter = parsed.unwrap_or_else(|| Frontmatter { title: None, member_of: None });

    let mut decoded = [0xFF; 16];
    decode(pb.file_stem().unwrap().to_string_lossy().to_string().as_str()).into(&mut decoded).unwrap();
    let id = uuid::Uuid::from_slice(&decoded).unwrap().to_hyphenated().to_string();
    // let id = std::str::from_utf8(&decoded).unwrap().to_owned();

    Ok(
        Lyric {
            id,
            title: frontmatter.title,
            member_of: frontmatter.member_of,
            parts,
        }
    )
}

pub async fn get_lyrics(path: &str) -> Result<impl Stream<Item=Lyric>, Error> {
    read_dir(path)
    .map(|list|
        iter(list)
        .filter(|entry| ready(entry.is_ok()))
        .map(|entry| entry.unwrap().path())
        .filter(|path_buffer| ready(path_buffer.extension() == Some(OsStr::new("txt"))))
        .then(|path_buffer| async move {
            get_file(&path_buffer).await
        })
        .filter(|lyric_file| ready(lyric_file.is_ok()))
        .map(|lyric_file| lyric_file.unwrap())
    )
}
