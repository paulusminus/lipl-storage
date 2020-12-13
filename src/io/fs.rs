use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

use crate::model::{parts_to_string, PathBufExt, Lyric, LiplError, LiplResult, Playlist, PlaylistPost, Uuid, UuidExt};
use crate::io::{get_lyric, get_playlist};

pub fn fs_read<P>(dir_path: P) -> LiplResult<(HashMap<Uuid, Lyric>, HashMap<Uuid, Playlist>)>
where P: AsRef<Path> {
    let mut lyric_hm: HashMap<Uuid, Lyric> = HashMap::new();
    let mut playlist_hm: HashMap<Uuid, Playlist> = HashMap::new();

    for entry in read_dir(&dir_path)? {
        let file_path = entry?.path();
        if file_path.is_file() && file_path.extension() == Some(OsStr::new("txt")) {
            let uuid = file_path.to_uuid();
            lyric_hm.insert(
                uuid,
                get_lyric(File::open(file_path)?).map(|lp| Lyric::from((Some(uuid), lp)))?
            );
        }
        else if file_path.is_file() && file_path.extension() == Some(OsStr::new("yaml")) {
            let uuid = file_path.to_uuid();
            playlist_hm.insert(
                uuid,
                get_playlist(File::open(file_path)?).map(|pp| Playlist::from((Some(uuid), pp)))?
            );
        }
    }

    Ok((lyric_hm, playlist_hm))
}

pub fn fs_write<P: AsRef<Path>>(path: P, lyrics: HashMap<Uuid, Lyric>, playlists: HashMap<Uuid, Playlist>) -> LiplResult<()> {
    let dir: PathBuf = path.as_ref().into();
    if !dir.exists() {
        return Err(LiplError::NonExistingDirectory(dir));
    }

    for lyric in lyrics.values() {
        let filename: PathBuf = format!("{}.txt", lyric.id.to_base58()).into();
        let full_path: PathBuf = dir.join(filename);
        let title_content = lyric.title.as_ref().map(|s| format!("---\ntitle: {}\n---\n\n", s)).unwrap_or_default();
        let content = format!("{}{}", title_content, parts_to_string(&lyric.parts));
        let bytes = content.as_str().as_bytes();
        std::fs::write(full_path, bytes)?;
    };

    for playlist in playlists.values() {
        let filename = format!("{}.yaml", playlist.id.to_base58());
        let full_path = dir.join(filename);
        let disk_playlist = PlaylistPost::from((playlist.title.clone(), playlist.members.clone()));
        let content = serde_yaml::to_string(&disk_playlist)?;
        let bytes = content.as_str().as_bytes();
        std::fs::write(full_path, bytes)?;
    }
    
    Ok(())
}
