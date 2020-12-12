use std::ffi::OsString;
use std::collections::HashMap;
use uuid::Uuid;
use std::path::{PathBuf};
use crate::model::{LiplResult, Lyric, Playlist};

type Collections = (HashMap<Uuid, Lyric>, HashMap<Uuid, Playlist>);

pub trait Db<T>
{
    fn load(&self) -> LiplResult<T>;
    fn dump(&self, data: T) -> LiplResult<()>;
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

impl Db<Collections> for ZipDb {
    
    fn load(&self) -> LiplResult<Collections> {
        crate::io::zip_read(self.path.to_owned())
    }

    fn name(&self) -> OsString {
        self.path.to_owned().into_os_string()
    }

    fn dump(&self, cols: Collections) -> LiplResult<()> {
        crate::io::zip_write(self.path.to_owned(), cols.0, cols.1)
    }
}
