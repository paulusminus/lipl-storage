use std::collections::HashMap;
use std::ffi::{OsStr};
use std::fs::{read_dir, read_to_string, File};
use std::io::{Error};
use std::path::{PathBuf};

pub use uuid::Uuid;
use futures::future::ready;
use futures::io::{AllowStdIo, BufReader};
use futures::stream::{Stream, StreamExt, iter};

mod args;
mod model;
mod parts;
mod pathbuf_ext;
mod uuid_ext;
pub use pathbuf_ext::PathBufExt;
pub use uuid_ext::UuidExt;
use model::{Frontmatter, HasId, Playlist};
pub use args::{get_path};
pub use model::{DiskPlaylist, Lyric, Summary};
pub use parts::to_parts_async;

pub type Db<T> = HashMap<Uuid, T>;

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

fn get_fs_files(rd: std::fs::ReadDir, filter: &'static str) -> impl Iterator<Item=PathBuf> {
    rd
    .filter_map(|entry| entry.ok().map(|e| e.path()))
    .filter(|entry| entry.is_file())
    .filter(move |path_buffer| path_buffer.extension() == Some(OsStr::new(filter)))
}

pub async fn get_lyrics(path: &str) -> Result<impl Stream<Item=Lyric>, Error> {
    read_dir(path)
    .map(|list|
        iter(get_fs_files(list, "txt"))
        .then(|path_buffer| async move {
            get_lyric(&path_buffer).await
        })
        .filter_map(|lyric_file| ready(lyric_file.ok()))
    )
}

pub async fn get_playlists(path: &str) -> Result<impl Stream<Item=Playlist>, Error> {
    read_dir(path)
    .map(|list|
        iter(get_fs_files(list, "yaml"))
        .filter_map(|path_buffer| ready(get_playlist(&path_buffer)))
        .map(Playlist::from)
    )
}

pub async fn create_hashmap<T: HasId>(s: impl Stream<Item=T>) -> HashMap<Uuid, T> {
    s
    .collect::<Vec<T>>()
    .await
    .into_iter()
    .map(|e| (e.id(), e))
    .collect()
}

pub async fn create_db(path: &str) -> Result<(Db<Lyric>, Db<Playlist>), Error> {
    let dm_lyrics = create_hashmap(get_lyrics(path).await?).await;
    let dm_playlists = create_hashmap(get_playlists(path).await?).await;
    Ok(
        (
            dm_lyrics,
            dm_playlists,
        )
    )
}
