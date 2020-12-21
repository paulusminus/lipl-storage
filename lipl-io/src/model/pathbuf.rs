use std::ffi::OsStr;
use std::path::{Path};
use crate::model::{LiplResult, Uuid, UuidExt};

pub trait PathBufExt {
    fn to_uuid(&self) -> Uuid;
    fn try_to_uuid(&self) -> LiplResult<Uuid>;
    fn has_extension(&self, ext: &'static str) -> bool;
}

impl<T> PathBufExt for T where T: AsRef<Path> {
    fn to_uuid(&self) -> Uuid {
        self.try_to_uuid().unwrap()
    }

    fn try_to_uuid(&self) -> LiplResult<Uuid> {
        Uuid::try_from_base58(&self)
    }

    fn has_extension(&self, ext: &'static str) -> bool {
        self.as_ref().extension() == Some(OsStr::new(ext))
    }
}
