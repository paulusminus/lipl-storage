use std::ffi::OsStr;
use std::path::{Path};
use crate::model::{HasExtension, ExtractUuid, LiplError, LiplResult, Uuid};

impl<T> HasExtension for T where T: AsRef<Path> {
    fn has_extension(&self, ext: &str) -> bool {
        self.as_ref().extension() == Some(OsStr::new(ext))
    }
}

impl<T> ExtractUuid for T where T: AsRef<Path> {
    fn extract_uuid(&self) -> LiplResult<Uuid> {
        let file_stem = self.as_ref().file_stem().ok_or(LiplError::NoPath(self.as_ref().to_path_buf()))?;
        let s = file_stem.to_string_lossy().to_string();
        s.parse::<Uuid>()
    }
}
