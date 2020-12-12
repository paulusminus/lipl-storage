use std::fs::{read_dir, File};
use std::path::Path;
use std::io::{Read};

use super::get_fs_files;
use crate::model;
use model::{LiplResult, PathBufExt, Playlist, PlaylistPost};

pub fn get_playlist<R: Read>(reader: R) -> LiplResult<PlaylistPost> {
    Ok(serde_yaml::from_reader::<R, PlaylistPost>(reader)?)
}

pub fn get_playlists<P: AsRef<Path>>(path: P) -> LiplResult<impl Iterator<Item=Playlist>> {
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
            .map(Playlist::from)
        )?
    )
}
