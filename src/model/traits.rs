use std::fs::{DirEntry, File};
use std::path::PathBuf;
use crate::model::{LiplResult, Summary, Uuid, PathBufExt};
use zip::read::{ZipFile};

pub trait HasId {
    fn id(&self) -> Uuid;
}

pub trait HasSummary {
    fn to_summary(&self) -> Summary;
}

pub trait VFile<'a> {
    fn uuid(&self) -> Uuid;
    fn reader(&'a mut self) -> LiplResult<Box<dyn std::io::Read + 'a>>;
    fn path(&self) -> PathBuf;
    fn is_file(&self) -> bool;
}

impl<'a> VFile<'a> for DirEntry {
    fn uuid(&self) -> Uuid {
        self.path().to_uuid()
    }

    fn reader(&'a mut self) -> LiplResult<Box<dyn std::io::Read + 'a>> {
        Ok(
            Box::new(File::open(self.path())?)
        )
    }

    fn path(&self) -> PathBuf {
        self.path().into()
    }

    fn is_file(&self) -> bool {
        self.path().is_file()
    }
}

impl<'a> VFile<'a> for ZipFile<'a> {
    fn uuid(&self) -> Uuid {
        self.name().to_uuid()
    }

    fn reader(&'a mut self) -> LiplResult<Box<dyn std::io::Read + 'a>> {
        Ok(
            Box::new(self)
        )
    }

    fn path(&self) -> PathBuf {
        self.name().into()
    }

    fn is_file(&self) -> bool {
        self.is_file()
    }
}