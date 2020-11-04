use tokio::fs::{read_dir, File};
use futures::stream::{Stream, StreamExt};
use futures::future::ready;
use tokio::io::BufReader;
use std::path::PathBuf;
use std::io::Error;
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
    let file = File::open(pb).await?;
    let reader = BufReader::new(file);
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
    .await
    .map(|rd|
        rd
        .filter(|entry| ready(entry.is_ok()))
        .map(|entry| entry.unwrap().path())
        .then(|path_buffer| async move {
            get_file(&path_buffer).await
        })
        .filter(|lyric_file| ready(lyric_file.is_ok()))
        .map(|lyric_file| lyric_file.unwrap())
    )
}
