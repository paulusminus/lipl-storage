use std::ffi::OsString;
use std::collections::HashMap;
use uuid::Uuid;
use std::io::Error;
use std::path::{PathBuf};

use crate::model;

type IOResult<T> = Result<T, Error>;
type Collections = (HashMap<Uuid, model::Lyric>, HashMap<Uuid, model::Playlist>);

pub trait Db<T>
{
    fn load(&self) -> IOResult<T>;
    fn dump(&self, data: T) -> IOResult<()>;
    fn name(&self) -> OsString;
}

pub struct ZipDb {
    path: PathBuf,
}

impl ZipDb {
    pub fn new(path: PathBuf) -> Self {
        ZipDb { path }
    }
}

pub struct FsDb {
    path: PathBuf,
}

impl Db<Collections> for ZipDb {
    
    fn load(&self) -> IOResult<Collections> {
        crate::io::zip_read(self.path.to_owned())
    }

    fn name(&self) -> OsString {
        self.path.to_owned().into_os_string()
    }

    fn dump(&self, cols: Collections) -> IOResult<()> {
        crate::io::zip_write(self.path.to_owned(), cols.0, cols.1)
    }
}