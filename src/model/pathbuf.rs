use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use crate::model::{LiplResult, Uuid, UuidExt};

pub trait PathBufExt {
    fn to_uuid(&self) -> Uuid;
    fn try_to_uuid(&self) -> LiplResult<Uuid>;
    fn has_extension(&self, ext: &str) -> bool;
    fn is_file_type(&self, ext: &str) -> bool;
}

impl<T> PathBufExt for T where T: AsRef<Path> {
    fn to_uuid(&self) -> Uuid {
        self.try_to_uuid().unwrap()
    }

    fn try_to_uuid(&self) -> LiplResult<Uuid> {
        Uuid::try_from_base58(&self)
    }

    fn has_extension(&self, ext: &str) -> bool {
        let pb: PathBuf = self.as_ref().into();
        pb.extension() == Some(OsStr::new(ext))
    }

    fn is_file_type(&self, ext: &str) -> bool {
        let pb: PathBuf = self.as_ref().into();
        pb.is_file() && self.has_extension(ext)
    }
}
