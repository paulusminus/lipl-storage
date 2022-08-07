use std::fs::{read_dir, DirEntry, File};
use std::io::prelude::*;
use std::io::Error as IOError;
use std::io::Result as IOResult;
use std::path::PathBuf;

pub struct Lyric {
    pub title: String,
    pub text: String,
}

impl From<(&PathBuf, String)> for Lyric {
    fn from(pb: (&PathBuf, String)) -> Self {
        Lyric {
            title: pb
                .0
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default(),
            text: pb.1,
        }
    }
}

fn file_to_lyric(path: &PathBuf) -> IOResult<Lyric> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok((path, contents).into())
}

fn entry_to_lyric(r: Result<DirEntry, IOError>) -> Option<Lyric> {
    r.ok().and_then(|f| file_to_lyric(&f.path()).ok())
}

pub fn get_lyrics(source_dir: &str) -> Result<impl Iterator<Item = Lyric>, IOError> {
    read_dir(source_dir).map(|entries| entries.filter_map(entry_to_lyric))
}
