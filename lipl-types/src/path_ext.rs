use std::ffi::OsStr;
use std::path::{Path};
use crate::{error::{ModelError, ModelResult}, Uuid};

pub trait PathExt {
    fn has_extension(&self, ext: &str) -> bool;
    fn extract_uuid(&self) -> ModelResult<Uuid>;
}

impl<P> PathExt for P
where
    P: AsRef<Path>,
{
    fn has_extension(&self, ext: &str) -> bool {
        self.as_ref().extension() == Some(OsStr::new(ext))
    }

    fn extract_uuid(&self) -> ModelResult<Uuid> {
        self
        .as_ref()
        .file_stem()
        .ok_or_else(|| ModelError::NoPath(self.as_ref().to_path_buf()))
        .map(|file_stem| file_stem.to_string_lossy().to_string())
        .and_then(|s| s.parse::<Uuid>())
    }
}
