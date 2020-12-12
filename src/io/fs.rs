// use std::collections::HashMap;
// use crate::model;
// use crate::io;
// use uuid::Uuid;
use std::path::Path;
use std::ffi::OsStr;
// use std::fs::{read_dir, File};
// use std::io::Error;
// use model::PathBufExt;
// use std::convert::TryFrom;

pub fn get_fs_files(rd: std::fs::ReadDir, filter: &'static str) -> impl Iterator<Item=impl AsRef<Path>> {
    rd
    .filter_map(|entry| entry.ok().map(|e| e.path()))
    .filter(|entry| entry.is_file())
    .filter(move |path_buffer| path_buffer.extension() == Some(OsStr::new(filter)))
}


/*
pub fn load<P>(dir_path: P) -> Result<(HashMap<Uuid, model::Lyric>, HashMap<Uuid, model::Playlist>), Error>
where P: AsRef<Path> {
    let mut lyric_hm: HashMap<Uuid, model::Lyric> = HashMap::new();
    let mut playlist_hm: HashMap<Uuid, model::Playlist> = HashMap::new();

    for entry in read_dir(&dir_path)? {
        let file_path = entry?.path();
        if file_path.is_file() && file_path.extension() == Some(OsStr::new(".txt")) {
            let uuid = file_path.to_uuid();
            lyric_hm.insert(
                uuid,
                io::get_lyric(File::open(file_path)?, uuid)?
            );
        }
        else if file_path.is_file() && file_path.extension() == Some(OsStr::new(".yaml")) {
            let uuid = file_path.to_uuid();
            playlist_hm.insert(
                uuid,
                io::get_playlist(File::open(file_path)?).unwrap().into()
            );
        }
    }

    Ok((lyric_hm, playlist_hm))
}

enum FileType {
    Yaml,
    Text,
}

impl TryFrom<std::path::PathBuf> for FileType {
    type Error = &'static str;
    fn try_from(p: std::path::PathBuf) -> Result<Self, Self::Error> {
        if p.extension() == Some(OsStr::new("yaml")) {
            return Ok(FileType::Yaml);
        }
        if p.extension() == Some(OsStr::new("txt")) {
            return Ok(FileType::Text);
        }
        Err("Wrong")
    }
}

pub async fn load2<P: AsRef<Path>>(dir_path: P) -> Result<(HashMap<Uuid, model::Lyric>, HashMap<Uuid, model::Playlist>), Error> {
    let mut db = (HashMap::<Uuid, model::Lyric>::new(), HashMap::<Uuid, model::Playlist>::new());

    let result = read_dir(dir_path)?
    .filter_map(|e| e.ok())
    .map(|e| e.path())
    .filter(|p| p.is_file());

    for p in result {
        if p.extension() == Some(OsStr::new("yaml")) {
            let uuid = p.to_uuid();
            db.1.insert(
                uuid,
                io::get_playlist(File::open(p)?).unwrap().into()
            );
        }
        else if p.extension() == Some(OsStr::new("txt")) {
            let uuid = p.to_uuid();
            db.0.insert(
                uuid,
                io::get_lyric(File::open(p)?, uuid)?
            );
        }
    } 
    Ok(db)
}
*/