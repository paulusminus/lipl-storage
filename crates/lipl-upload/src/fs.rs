use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::io::Error as IOError;
use std::result::Result;
use futures::{future::{ready, Ready}, Future, Stream, TryFutureExt, TryStream, TryStreamExt};
use tokio_stream::wrappers::ReadDirStream;
use tokio::fs::{read_dir, read_to_string, DirEntry};
use crate::UploadResult;
use crate::api::{UploadClient, Api};
use lipl_core::{Uuid, LyricPost};
use crate::error::UploadError;

pub struct Entry {
    pub path: PathBuf,
    pub contents: String,
}

impl Entry {
    pub fn title(&self) -> String {
        self
        .path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
    }
}

async fn entry_from_file<P>(path: P) -> UploadResult<Entry> 
where P: AsRef<Path>
{
    read_to_string(path.as_ref())
    .err_into()
    .map_ok(|contents| Entry { path: path.as_ref().to_path_buf(), contents })
    .await
}

pub fn extension_filter(extension: &str) -> impl Fn(&PathBuf) -> Ready<bool> + '_
{
    |p| ready(p.extension() == Some(OsStr::new(extension)))
}

async fn get_files_stream<P: AsRef<Path>>(path: P) -> UploadResult<impl Stream<Item=Result<DirEntry, IOError>>> {
    read_dir(path.as_ref())
    .err_into()
    .map_ok(ReadDirStream::new)
    .await
}

pub async fn post_lyrics<'a, P, F, Fut>(path: P, filter: F, client: &'a UploadClient) -> UploadResult<impl TryStream<Ok=Uuid, Error=UploadError> + 'a>
where 
    P: AsRef<Path> + 'a,
    F: Fn(&PathBuf) -> Fut + 'a,
    Fut: Future<Output = bool> + 'a
{
    get_files_stream(path)
    .map_ok(|s|
        s.map_ok(|de| de.path())
        .err_into()
        .try_filter(filter)
        .and_then(entry_from_file)
        .map_ok(LyricPost::from)
        .and_then(|lp| client.lyric_insert(lp))
        .map_ok(|lyric| lyric.id)
    )
    .await
}
