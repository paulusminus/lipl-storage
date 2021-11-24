use std::path::{Path, PathBuf};
use std::ffi::{OsStr};
use tokio_stream::wrappers::{ReadDirStream};
use crate::{Error, Result};
use futures::prelude::*;
use futures::future::{ready, Ready};
use tokio::fs::{read_dir};

pub fn extension_filter(s: &str) -> impl Fn(&PathBuf) -> Ready<bool> + '_ {
    |path_buf| ready(path_buf.extension() == Some(OsStr::new(s)))
}

pub fn full_path(base: &str, filename: &str, extension: &str) -> PathBuf {
    Path::new(base).join(format!("{}.{}", filename, extension))
}

pub fn is_dir<P>(path: P) -> bool where P: AsRef<Path> {
    Path::new(path.as_ref()).is_dir()
}

pub async fn get_files<'a, P, F>(path: P, filter: F) -> Result<impl TryStream<Ok=PathBuf, Error=Error> + 'a> 
where P: AsRef<Path>, F: Fn(&PathBuf) -> Ready<bool> + 'a
{
    read_dir(path)
    .await
    .map_err(Error::from)
    .map(
        |de| 
            ReadDirStream::new(de)
            .map_err(Error::from)
            .map_ok(|de| de.path())
            .try_filter(filter)
    )
}
