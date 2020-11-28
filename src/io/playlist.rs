use std::fs::{read_dir, File};
use std::path::Path;
use std::io::Error;
use futures::future::ready;
use futures::stream::{Stream, StreamExt, iter};

use super::get_fs_files;
use crate::model;
use model::PathBufExt;

pub fn get_playlist<T: std::io::Read>(reader: Result<T, std::io::Error>) -> Option<model::PlaylistPost> {
    reader
    .ok()
    .and_then(|r| serde_yaml::from_reader::<T, model::PlaylistPost>(r).ok())
}

pub async fn get_playlists<P: AsRef<Path>>(path: P) -> Result<impl Stream<Item=model::Playlist>, Error> {
    read_dir(path)
    .map(|list|
        iter(get_fs_files(list, "yaml"))
        .filter_map(
            |path_buffer| ready(
                get_playlist(
                    File::open(&path_buffer)
                )
                .map(|p| (path_buffer.to_uuid(), p))
            )
        )
        .map(model::Playlist::from)
    )
}
