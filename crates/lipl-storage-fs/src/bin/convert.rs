use std::{
    env::var,
    ffi::OsStr,
    fs::{read_dir, read_to_string, rename, DirEntry, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use lipl_core::{
    disk_format_yaml::{LyricPostWrapper, PlaylistPostWrapper},
    Error, Lyric, Playlist, Result, Uuid,
};
use lipl_storage_fs::constant::{LYRIC_EXTENSION, TOML_EXTENSION};

const YAML_EXTENSION: &str = "yaml";
const TO_TOML_OK_FILENAME: &str = "TO_TOML_OK";

fn to_object<T, U>(de: &DirEntry) -> Result<U>
where
    T: FromStr<Err = Error>,
    U: From<(Uuid, T)>,
{
    let s = read_to_string(de.path())?;
    let post = s.parse::<T>()?;
    let uuid = de
        .path()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .parse::<Uuid>()?;
    Ok((uuid, post).into())
}

fn write_object<D: std::fmt::Display, P: AsRef<Path>>(out_path: P, d: D) -> Result<()> {
    let mut out = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(out_path.as_ref())?;
    write!(&mut out, "{}", d)?;
    Ok(())
}

fn is_file(de: &DirEntry) -> bool {
    de.file_type().ok().map(|f| f.is_file()) == Some(true)   
}

fn backup(de: &DirEntry) -> Result<()> {
    let mut backup = de.path().as_os_str().to_owned();
    backup.push(".bak");
    rename(de.path(), backup)?;
    Ok(())
}

fn main() -> Result<()> {
    let in_dir = var("LIPL_STORAGE_FS_DIR").map(PathBuf::from)?;

    if in_dir.exists() && in_dir.is_dir() {
        println!("Using {} as input directory", in_dir.to_string_lossy());
    } else {
        return Err(Error::NonExistingDirectory(in_dir));
    }

    if in_dir.join(TO_TOML_OK_FILENAME).exists() && in_dir.join(TO_TOML_OK_FILENAME).is_file() {
        return Err(Error::Argument("Already converted"));
    }

    let dir = read_dir(&in_dir)?;
    let files = dir.map_while(|r| r.ok());

    for file in files.filter(is_file) {
        println!("Processing file {}", file.file_name().to_string_lossy());
        if file.path().extension() == Some(OsStr::new(YAML_EXTENSION)) {
            let playlist = to_object::<PlaylistPostWrapper, Playlist>(&file)?;
            println!("Found playlist with title {}", playlist.title);
            let out_path = in_dir.join(format!("{}.{}", playlist.id, TOML_EXTENSION));

            backup(&file)?;
            println!("original file renamed with bak extension");

            write_object(&out_path, playlist)?;
        } else if file.path().extension() == Some(OsStr::new(LYRIC_EXTENSION)) {
            let lyric = to_object::<LyricPostWrapper, Lyric>(&file)?;
            println!("Found lyric with title {}", lyric.title);
            let out_path = in_dir.join(format!("{}.{}", lyric.id, LYRIC_EXTENSION));

            backup(&file)?;
            println!("original file renamed with bak extension");
            write_object(out_path, lyric)?;
        }
    }
    OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(in_dir.join(TO_TOML_OK_FILENAME))?;

    Ok(())
}
