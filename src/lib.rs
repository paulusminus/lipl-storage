use tokio::fs::{read_dir, File};
use futures::stream::{Stream, StreamExt};
use futures::future::ready;
use tokio::io::BufReader;
use std::path::PathBuf;
use std::io::Error;

mod parts;
pub use parts::to_parts_async;

pub struct Lyric {
    pub yaml: Option<String>,
    pub parts: Vec<Vec<String>>,
}

pub async fn get_file(pb: &PathBuf) -> Result<Lyric, Error> {
    let file = File::open(pb).await?;
    let reader = BufReader::new(file);
    to_parts_async(reader).await
}

pub async fn get_lyrics(path: &str) -> Result<impl Stream<Item=(PathBuf, Lyric)>, Error> {
    read_dir(path)
    .await
    .map(|rd|
        rd
        .filter(|entry| ready(entry.is_ok()))
        .map(|entry| entry.unwrap().path())
        .then(|path_buffer| async move {
            (path_buffer.to_path_buf(), get_file(&path_buffer).await)
        })
        .filter(|(_, lyric_file)| ready(lyric_file.is_ok()))
        .map(|(path_buffer, lyric_file)| (path_buffer, lyric_file.unwrap()))
    )
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
