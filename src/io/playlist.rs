use std::fs::{read_dir, File};
use std::path::Path;
use std::io::{Read, Error as IOError};

use serde_yaml::Error as YamlError;

use super::get_fs_files;
use crate::model;
use model::PathBufExt;

pub fn get_playlist<R: Read>(reader: R) -> Result<model::PlaylistPost, YamlError> {
    serde_yaml::from_reader::<R, model::PlaylistPost>(reader)
}

pub fn get_playlists<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item=model::Playlist>, IOError> {
    Ok(
        read_dir(path)
        .map(|list|
            get_fs_files(list, "yaml")
            .filter_map(
                |path_buffer|
                    File::open(&path_buffer).ok()
                    .and_then(|f| get_playlist(f).ok())
                    .map(|p| (path_buffer.to_uuid(), p))
            )
            .map(model::Playlist::from)
        )?
    )
}
