use std::path::PathBuf;
use std::ffi::OsStr;

pub fn get_fs_files(rd: std::fs::ReadDir, filter: &'static str) -> impl Iterator<Item=PathBuf> {
    rd
    .filter_map(|entry| entry.ok().map(|e| e.path()))
    .filter(|entry| entry.is_file())
    .filter(move |path_buffer| path_buffer.extension() == Some(OsStr::new(filter)))
}
