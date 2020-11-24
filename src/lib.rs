use std::ffi::{OsStr};
use std::fs::{read_dir, read_to_string, File};
use std::io::{Error};
use std::path::{PathBuf};

use futures::future::ready;
use futures::io::{AllowStdIo, BufReader};
use futures::stream::{Stream, StreamExt, iter};
use bs58::decode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod parts;
pub use parts::to_parts_async;

pub struct Lyric {
    pub id: Uuid,
    pub title: Option<String>,
    pub member_of: Option<Vec<String>>,
    pub parts: Vec<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DiskPlaylist {
    pub title: String,
    pub members: Vec<String>
}

pub struct Playlist {
    pub title: String,
    pub members: Vec<Uuid>
}

impl From<DiskPlaylist> for Playlist {
    fn from(disk_playlist: DiskPlaylist) -> Playlist {
        Playlist {
            title: disk_playlist.title,
            members: disk_playlist.members.iter().map(|m| {
                let mut decoded = [0xFF; 16];
                decode(m).into(&mut decoded).unwrap();
                Uuid::from_slice(&decoded).unwrap()
            })
            .collect()
        }
    }
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
    let id = uuid::Uuid::from_slice(&decoded).unwrap(); //.to_hyphenated().to_string();
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
        .filter_map(|entry| ready(entry.map(|e| e.path()).ok()))
        .filter(|path_buffer| ready(path_buffer.extension() == Some(OsStr::new("txt"))))
        .then(|path_buffer| async move {
            get_file(&path_buffer).await
        })
        .filter_map(|lyric_file| ready(lyric_file.ok()))
    )
}

pub async fn get_playlists(path: &str) -> Result<impl Stream<Item=Playlist>, Error> {
    read_dir(path)
    .map(|list|
        iter(list)
        .filter_map(|entry| ready(entry.map(|e| e.path()).ok()))
        .filter(|path_buffer| ready(path_buffer.extension() == Some(OsStr::new("yaml"))))
        .filter_map(|path_buffer| ready(read_to_string(path_buffer).ok()))
        .filter_map(|s| ready(serde_yaml::from_str::<DiskPlaylist>(&s).ok()))
        .map(Playlist::from)
    )
}
