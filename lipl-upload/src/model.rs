use lipl_types::LyricPost;
use crate::error::UploadError;
use futures::TryStream;
use futures::stream::iter;
use crate::{fs, UploadResult};
use parts::{to_parts};

pub fn lyric_post_from_entry(entry: fs::Entry) -> LyricPost {
    let title = 
        entry
        .path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let parts = to_parts(entry.contents);
    LyricPost {
        title,
        parts,
    }
}

pub fn try_iter<T>(v: Vec<T>) -> impl TryStream<Ok=T, Error=UploadError> {
    iter(
        v
        .into_iter()
        .map(UploadResult::Ok)
    )
}
