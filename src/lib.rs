// use std::collections::HashMap;
use std::ffi::{OsStr};
use std::fs::{read_dir, read_to_string, File};
use std::io::{Error};
use std::path::{PathBuf};

use dashmap::DashMap;
use futures::future::ready;
use futures::io::{AllowStdIo, BufReader};
use futures::stream::{Stream, StreamExt, iter};
use uuid::Uuid;

mod args;
mod model;
mod parts;
mod pathbuf_ext;
mod uuid_ext;
use pathbuf_ext::PathBufExt;
use uuid_ext::UuidExt;
use model::{DiskPlaylist, Frontmatter, HasId, Playlist};
pub use crate::args::{get_path};
pub use crate::model::Lyric;
pub use crate::parts::to_parts_async;

pub type Db<T> = DashMap<Uuid, T>;

pub async fn get_lyric(pb: &PathBuf) -> Result<Lyric, Error> {
    let file = File::open(pb)?;
    let reader = AllowStdIo::new(file);
    let async_reader = BufReader::new(reader);
    let (yaml, parts) = to_parts_async(async_reader).await?;

    let parsed = yaml.and_then(|text| serde_yaml::from_str::<Frontmatter>(&text).ok());
    let frontmatter = parsed.unwrap_or_default();
    let id = pb.to_uuid();

    Ok(
        Lyric {
            id,
            title: frontmatter.title,
            parts,
        }
    )
}

pub fn get_playlist(pb: &PathBuf) -> Option<(Uuid, DiskPlaylist)> {
    read_to_string(pb)
    .ok()
    .and_then(|s| serde_yaml::from_str::<DiskPlaylist>(&s).ok())
    .map(|d| (pb.to_uuid(), d))
}

fn get_files(rd: std::fs::ReadDir, filter: &'static str) -> impl Stream<Item=PathBuf> {
    iter(rd)
    .filter_map(|entry| ready(entry.map(|e| e.path()).ok()))
    .filter(|entry| ready(entry.is_file()))
    .filter(move |path_buffer| ready(path_buffer.extension() == Some(OsStr::new(filter))))    
}

pub async fn get_lyrics(path: &str) -> Result<impl Stream<Item=Lyric>, Error> {
    read_dir(path)
    .map(|list|
        get_files(list, "txt")
        .then(|path_buffer| async move {
            get_lyric(&path_buffer).await
        })
        .filter_map(|lyric_file| ready(lyric_file.ok()))
    )
}

pub async fn get_playlists(path: &str) -> Result<impl Stream<Item=Playlist>, Error> {
    read_dir(path)
    .map(|list|
        get_files(list, "yaml")
        .filter_map(|path_buffer| ready(get_playlist(&path_buffer)))
        .map(Playlist::from)
    )
}

pub async fn create_dashmap<T: HasId>(s: impl Stream<Item=T>) -> DashMap<Uuid, T> {
    s
    .collect::<Vec<T>>()
    .await
    .into_iter()
    .map(|e| (e.id(), e))
    .collect()
}

pub async fn create_db(path: &String) -> Result<(Db<Lyric>, Db<Playlist>), Error> {
    Ok(
        (
            create_dashmap(get_lyrics(path).await?).await,
            create_dashmap(get_playlists(path).await?).await,
        )
    )
}