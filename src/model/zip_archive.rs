use std::fs::File;
use zip::read::{ZipArchive, ZipFile};

pub trait ZipArchiveExt<'a> {
    fn list(&'a mut self) -> &'a Vec<ZipFile<'a>>;
}

impl<'a> ZipArchiveExt<'a> for &'a mut ZipArchive<File> {

    fn list(&'a mut self) -> &'a Vec<ZipFile<'a>> {
        let mut result: &'a Vec<ZipFile<'a>> = vec![];
        for i in 0..self.len() {
            result.push(self.by_index(i).unwrap());
        }
        result
    }
}