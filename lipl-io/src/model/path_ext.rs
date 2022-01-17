use std::ffi::OsStr;
use std::path::{Path};
use crate::model::{RepoError, RepoResult, Uuid};

pub trait PathExt {
    fn has_extension(&self, ext: &str) -> bool;
    fn extract_uuid(&self) -> RepoResult<Uuid>;
}

impl<P> PathExt for P
where
    P: AsRef<Path>,
{
    fn has_extension(&self, ext: &str) -> bool {
        self.as_ref().extension() == Some(OsStr::new(ext))
    }

    fn extract_uuid(&self) -> RepoResult<Uuid> {
        self
        .as_ref()
        .file_stem()
        .ok_or_else(|| RepoError::NoPath(self.as_ref().to_path_buf()))
        .map(|file_stem| file_stem.to_string_lossy().to_string())
        .and_then(|s| s.parse::<Uuid>())
    }
}
