use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::io::Error as IOError;
use std::result::Result;
use futures::{future::{ready, Ready}, Future, Stream, TryStream, TryStreamExt};
use tokio_stream::wrappers::ReadDirStream;
use tokio::fs::{read_dir, read_to_string, DirEntry};
use crate::api::Api;
use crate::client::UploadClient;
use crate::UploadResult;
use crate::model::{LyricPost};
use crate::error::UploadError;

pub struct Entry {
    pub path: PathBuf,
    pub contents: String,
}

async fn get_entry<P>(path: P) -> UploadResult<Entry> 
where P: AsRef<Path>
{
    let contents = read_to_string(path.as_ref()).await?;
    Ok(Entry { path: path.as_ref().to_path_buf(), contents })
}

pub fn extension_filter(extension: &str) -> impl Fn(&PathBuf) -> Ready<bool> + '_
{
    |p| ready(p.extension() == Some(OsStr::new(extension)))
}

async fn get_files_stream<P: AsRef<Path>>(path: P) -> UploadResult<impl Stream<Item=Result<DirEntry, IOError>>> {
    let entries = read_dir(path.as_ref()).await?;
    Ok(ReadDirStream::new(entries))
}

pub async fn post_lyrics<'a, P, F, Fut>(path: P, filter: F, client: &'a UploadClient) -> UploadResult<impl TryStream<Ok=String, Error=UploadError> + 'a>
where 
    P: AsRef<Path> + 'a,
    F: Fn(&PathBuf) -> Fut + 'a,
    Fut: Future<Output = bool> + 'a
{
    get_files_stream(path)
    .await
    .map(|s|
        s.map_ok(|de| de.path())
        .map_err(UploadError::from)
        .try_filter(filter)
        .and_then(get_entry)
        .map_ok(LyricPost::from)
        .and_then(|lp| client.lyric_insert(lp))
        .map_ok(|lyric| lyric.id)
    )
}
