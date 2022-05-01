use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};
use log::{info};

use crate::model::{Db, YAML, TXT, DataType, HasId, ToDiskFormat};
use lipl_types::{PathExt, RepoError, RepoResult};
use crate::io::{lyricpost_from_reader, playlistpost_from_reader};

pub fn fs_read<P, F>(dir_path: P, mut adder: F) -> RepoResult<()>
where P: AsRef<Path>,
F: FnMut(&PathBuf, &mut DataType),
{
    info!("Starting to read from directory {}", dir_path.as_ref().to_string_lossy());

    for entry in read_dir(&dir_path)? {
        let file_path = entry?.path();
        if file_path.has_extension(YAML) {
            let uuid = file_path.extract_uuid()?;
            adder(
                &file_path,
                &mut DataType::Playlist(
                    playlistpost_from_reader(
                        File::open(&file_path)?
                    )
                    .map(|pp| (Some(uuid), pp).into())?
                )
            );
        }
        else if file_path.has_extension(TXT) {
            let uuid = file_path.extract_uuid()?;
            adder(
                &file_path,
                &mut DataType::Lyric(
                    lyricpost_from_reader(
                        File::open(&file_path)?
                    )
                    .map(|lp| (Some(uuid), lp).into())?,
                )
            );
        }
    }

    Ok(())
}

fn write_fs_item<T>(item: T, ext: &str, parent_dir: &Path) -> RepoResult<()> where T: HasId + ToDiskFormat {
    let filename: PathBuf = format!("{}.{}", item.id(), ext).into();
    let full_path: PathBuf = parent_dir.join(filename);
    info!("Writing: {}", &full_path.to_string_lossy());
    let content = item.to_disk_format()?;
    let bytes = content.as_str().as_bytes();
    std::fs::write(full_path, bytes)?;
    Ok(())
}


pub fn fs_write<P>(path: P, db: &Db) -> RepoResult<()> 
where P: AsRef<Path>
{
    info!("Starting to write to directory {}", path.as_ref().to_string_lossy());
    let dir: PathBuf = path.as_ref().into();
    if !dir.exists() {
        return Err(RepoError::NonExistingDirectory(dir));
    }

    for lyric in db.get_lyric_list() {
        write_fs_item(
            lyric.clone(),
            TXT, 
            &dir,
        )?;
    }

    for playlist in db.get_playlist_list() {
        write_fs_item(
            playlist.clone(),
            YAML,
            &dir,
        )?;
    }
    
    Ok(())
}
