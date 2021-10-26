use std::ffi::OsStr;
use std::path::{Path};

pub trait PathBufExt {
    fn has_extension(&self, ext: &'static str) -> bool;
}

impl<T> PathBufExt for T where T: AsRef<Path> {
    fn has_extension(&self, ext: &'static str) -> bool {
        self.as_ref().extension() == Some(OsStr::new(ext))
    }
}
