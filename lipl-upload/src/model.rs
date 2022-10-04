use lipl_types::LyricPost;
use futures::TryStream;
use futures::stream::iter;
use crate::{fs, UploadResult, error::UploadError};
use parts::{to_parts};

impl From<fs::Entry> for LyricPost {
    fn from(entry: fs::Entry) -> Self {
        Self {
            title: entry.title(),
            parts: to_parts(entry.contents),
        }
    }
}

pub fn try_iter<T>(v: Vec<T>) -> impl TryStream<Ok=T, Error=UploadError> {
    iter(
        v
        .into_iter()
        .map(UploadResult::Ok)
    )
}
