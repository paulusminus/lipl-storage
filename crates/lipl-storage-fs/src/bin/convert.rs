use std::{env::{set_var, var}, ffi::OsStr, fs::{read_dir, read_to_string, DirEntry, OpenOptions}, io::Write, path::PathBuf, process::exit, str::FromStr};

use lipl_core::{disk_format_yaml::{LyricPostWrapper, PlaylistPostWrapper}, Error, Lyric, Playlist, Uuid};
use lipl_storage_fs::constant::{LYRIC_EXTENSION, TOML_EXTENSION};

const YAML_EXTENSION: &str = "yaml";

fn to_object<T, U>(de: DirEntry) -> U
where
    T: FromStr<Err = Error>,
    U: From<(Uuid, T)>,
{
    let s = read_to_string(de.path()).unwrap();
    let post = s.parse::<T>().unwrap();
    let uuid = de.path().file_stem().unwrap().to_string_lossy().to_string().parse::<Uuid>().unwrap();
    (uuid, post).into()
}

fn write_object<D: std::fmt::Display>(out_path: PathBuf, d: D) 
{
    let mut out = OpenOptions::new().create(true).write(true).open(out_path).unwrap();
    write!(&mut out, "{}", d).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_var("DATA_IN", "./data");
    set_var("DATA_OUT", "./out");
    let in_dir = var("DATA_IN").map(PathBuf::from).unwrap();
    let out_dir = var("DATA_OUT").map(PathBuf::from).unwrap();
    if in_dir.exists() && in_dir.is_dir() && out_dir.exists() && out_dir.is_dir() {
        let dir = read_dir("./data").unwrap().map_while(Result::ok);

        for file in dir.filter(|de| de.file_type().unwrap().is_file()) {
            if file.path().extension() == Some(&OsStr::new(YAML_EXTENSION)) {
                let playlist = to_object::<PlaylistPostWrapper, Playlist>(file);
                let out_path = out_dir.join(format!("{}.{}", playlist.id, TOML_EXTENSION));
                write_object(out_path, playlist);
            }
            else if file.path().extension() == Some(&OsStr::new(LYRIC_EXTENSION)) {
                let lyric = to_object::<LyricPostWrapper, Lyric>(file);
                let out_path = out_dir.join(format!("{}.{}", lyric.id, LYRIC_EXTENSION));
                write_object(out_path, lyric);
            }
        }
    }
    else {
        eprint!("Check DATA_IN and DATA_OUT. Are you sure you set the environment variables correct?");
        exit(1);
    }

    Ok(())
}