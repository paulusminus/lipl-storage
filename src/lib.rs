use tokio::fs::{read_dir, File};
use futures::stream::{Stream, StreamExt};
use futures::future::ready;
use tokio::io::BufReader;
use std::path::PathBuf;
use std::io::Error;

mod parts;
pub use parts::to_parts_async;

pub async fn get_file(pb: &PathBuf) -> Result<(Option<String>, Vec<Vec<String>>), Error> {
    let file = File::open(pb).await?;
    let reader = BufReader::new(file);
    to_parts_async(reader).await
}

pub async fn get_lyrics(path: &str) -> Result<impl Stream<Item=(PathBuf, (Option<String>, Vec<Vec<String>>))>, Error> {
    read_dir(path)
    .await
    .map(|rd|
        rd
        .filter(|e| ready(e.is_ok()))
        .map(|e| e.unwrap().path())
        .then(|p| async move {
            (p.to_path_buf(), get_file(&p).await)
        })
        .filter(|(_, s)| ready(s.is_ok()))
        .map(|(p, s)| (p, s.unwrap()))
    )
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
