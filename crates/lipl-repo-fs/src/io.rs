use std::str::FromStr;
use std::path::{Path, PathBuf};
use futures::{TryFuture, TryStreamExt};

use lipl_core::{Error, Lyric, LyricPost, Playlist, PlaylistPost, Summary, LyricMeta, Uuid};
use crate::fs::IO;

type Result<T> = std::result::Result<T, Error>;

pub async fn get_lyric_summary<P>(path: P) -> Result<Summary> 
where P: AsRef<Path> + Send + Sync
{
    get_item::<LyricMeta, Summary>(
        path.read_frontmatter().await?,
        path.id()?,
    )
}

pub fn get_item<F, G>(s: String, id: Uuid) -> Result<G>
where
    F: FromStr<Err=lipl_core::Error>,
    G: From<(Option<Uuid>, F)>,
{
    s.parse::<F>().map_err(|_| Error::Parse(format!("{id}"))).map(|f| G::from((Some(id), f)))
}

pub async fn get_playlist<P>(path: P) -> Result<Playlist>
where
    P: AsRef<Path> + Send + Sync,
{
    get_item::<PlaylistPost, Playlist>(
        path.read_string().await?,
        path.id()?,
    )
}

pub async fn get_list<P, T, F, Fut>(path: P, ext: &str, f: F) -> Result<Vec<T>> 
where 
    P: AsRef<Path> + Send + Sync,
    F: FnMut(PathBuf) -> Fut,
    Fut: TryFuture<Ok=T, Error=Error>,
{
    path.get_files(crate::fs::extension_filter(ext))
    .await?
    .and_then(f)
    .try_collect::<Vec<T>>()
    .await
}

pub async fn post_item<D, P>(path: P, d: D) -> Result<()>
where
    D: std::fmt::Display,
    P: AsRef<Path> + Send + Sync,
{
    path.write_string(d.to_string()).await
}

pub async fn get_lyric<P>(path: P) -> Result<Lyric>
where P: AsRef<Path> + Send + Sync,
{
    get_item::<LyricPost, Lyric>(
        path.read_string().await?,
        path.id()?
    )
}
