use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};
use log::{info};

use crate::model::{parts_to_string, Db, PathBufExt, Lyric, LiplError, LiplResult, Playlist, PlaylistPost, UuidExt, YAML, TXT};
use crate::io::{get_lyric, get_playlist};

pub fn fs_read<P>(dir_path: P, db: &mut Db) -> LiplResult<()>
where P: AsRef<Path> {
    info!("Starting to read from directory {}", dir_path.as_ref().to_string_lossy());

    for entry in read_dir(&dir_path)? {
        let file_path = entry?.path();
        if file_path.is_file() && file_path.has_extension(TXT) {
            let uuid = file_path.to_uuid();
            db.add_lyric(
                &get_lyric(File::open(file_path)?).map(|lp| Lyric::from((Some(uuid), lp)))?
            );
        }
    }

    for entry in read_dir(&dir_path)? {
        let file_path = entry?.path();
        if file_path.is_file() && file_path.has_extension(YAML) {
            let uuid = file_path.to_uuid();
            db.add_playlist(
                &mut get_playlist(File::open(file_path)?).map(|pp| Playlist::from((Some(uuid), pp)))?
            );
        }
    }

    Ok(())
}

pub fn fs_write<P: AsRef<Path>>(path: P, db: &Db) -> LiplResult<()> {
    info!("Starting to write to directory {}", path.as_ref().to_string_lossy());
    let dir: PathBuf = path.as_ref().into();
    if !dir.exists() {
        return Err(LiplError::NonExistingDirectory(dir));
    }

    for lyric in db.get_lyric_list() {
        let filename: PathBuf = format!("{}.{}", lyric.id.to_base58(), TXT).into();
        let full_path: PathBuf = dir.join(filename);
        info!("Writing: {}", &full_path.to_string_lossy());

        let title_content = lyric.title.as_ref().map(|s| format!("---\ntitle: {}\n---\n\n", s)).unwrap_or_default();
        let content = format!("{}{}", title_content, parts_to_string(&lyric.parts));
        let bytes = content.as_str().as_bytes();
        std::fs::write(full_path, bytes)?;
    };

    for playlist in db.get_playlist_list() {
        let filename = format!("{}.{}", playlist.id.to_base58(), YAML);
        let full_path = dir.join(filename);
        info!("Writing: {}", &full_path.to_string_lossy());
        let disk_playlist = PlaylistPost::from((playlist.title.clone(), playlist.members.clone()));
        let content = serde_yaml::to_string(&disk_playlist)?;
        let bytes = content.as_str().as_bytes();
        std::fs::write(full_path, bytes)?;
    }
    
    Ok(())
}
