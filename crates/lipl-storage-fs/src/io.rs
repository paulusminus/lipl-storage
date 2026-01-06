use futures_util::{Stream, StreamExt, TryFuture, TryStreamExt};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::str::FromStr;

use crate::fs::IO;
use lipl_core::{Error, Lyric, LyricMeta, LyricPost, Playlist, PlaylistPost, Summary, Uuid};

type Result<T> = std::result::Result<T, Error>;

pub async fn get_lyric_summary<P>(path: P) -> Result<Summary>
where
    P: AsRef<Path> + Send + Sync,
{
    get_item::<LyricMeta, Summary>(path.read_frontmatter().await?, path.id()?)
}

pub fn get_item<F, G>(s: String, id: Uuid) -> Result<G>
where
    F: FromStr<Err = lipl_core::Error>,
    G: From<(Option<Uuid>, F)>,
{
    s.parse::<F>()
        .map_err(|_| Error::Parse(format!("{id}")))
        .map(|f| G::from((Some(id), f)))
}

pub async fn get_playlist<P>(path: P) -> Result<Playlist>
where
    P: AsRef<Path> + Send + Sync,
{
    get_item::<PlaylistPost, Playlist>(path.read_string().await?, path.id()?)
}

pub async fn get_list<P, T, F, Fut>(path: P, ext: &str, f: F) -> Result<Vec<T>>
where
    P: AsRef<Path> + Send + Sync,
    F: FnMut(PathBuf) -> Fut,
    Fut: TryFuture<Ok = T, Error = Error>,
{
    path.get_files(crate::fs::extension_filter(ext))
        .await?
        .and_then(f)
        .try_collect::<Vec<T>>()
        .await
}

#[allow(dead_code)]
pub async fn get_stream<'a, P, T, F, Fut>(
    path: P,
    ext: &'a str,
    f: F,
) -> Result<Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>>>
where
    P: AsRef<Path> + Send + Sync,
    F: FnMut(PathBuf) -> Fut + Send + Sync + 'a,
    Fut: TryFuture<Ok = T, Error = Error> + Send + Sync + 'a,
{
    Ok(path
        .get_files(crate::fs::extension_filter(ext))
        .await?
        .and_then(f)
        .boxed())
}

pub async fn post_item<D, P>(path: P, d: D) -> Result<()>
where
    D: std::fmt::Display,
    P: AsRef<Path> + Send + Sync,
{
    path.write_string(d.to_string()).await
}

pub async fn get_lyric<P>(path: P) -> Result<Lyric>
where
    P: AsRef<Path> + Send + Sync,
{
    get_item::<LyricPost, Lyric>(path.read_string().await?, path.id()?)
}

#[cfg(test)]
mod test {
    use futures_util::TryStreamExt;
    use lipl_core::to_summary;
    use std::path::PathBuf;

    use crate::{
        constant::{LYRIC_EXTENSION, TOML_EXTENSION},
        io::{get_lyric, get_lyric_summary, get_playlist},
    };

    use super::get_stream;

    fn data_dir() -> PathBuf {
        std::env::var("DATA_DIR").unwrap().into()
    }

    #[tokio::test]
    async fn test_get_lyric_stream() {
        let mut stream = get_stream(data_dir(), LYRIC_EXTENSION, get_lyric)
            .await
            .unwrap();
        while let Some(item) = stream.try_next().await.unwrap() {
            dbg!(item);
        }
    }

    #[tokio::test]
    async fn test_get_lyric_summary_stream() {
        let mut stream = get_stream(data_dir(), LYRIC_EXTENSION, get_lyric_summary)
            .await
            .unwrap();
        while let Some(item) = stream.try_next().await.unwrap() {
            dbg!(item);
        }
    }

    #[tokio::test]
    async fn test_get_playlist_stream() {
        let mut stream = get_stream(data_dir(), TOML_EXTENSION, get_playlist)
            .await
            .unwrap();
        while let Some(item) = stream.try_next().await.unwrap() {
            dbg!(item);
        }
    }

    #[tokio::test]
    async fn test_get_playlist_summary_stream() {
        let mut stream = get_stream(data_dir(), TOML_EXTENSION, get_playlist)
            .await
            .unwrap()
            .map_ok(|playlist| to_summary(&playlist));
        while let Some(item) = stream.try_next().await.unwrap() {
            dbg!(item);
        }
    }
}
