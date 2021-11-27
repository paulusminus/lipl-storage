use std::path::{Path, PathBuf};
use futures::{TryFuture, TryStreamExt};

use crate::model::{Error, Lyric, LyricPost, Playlist, PlaylistPost, Result, Summary, LyricMeta};
use crate::fs::{Reader};

pub trait Id {
    fn id(&self) -> Result<String>;
}

impl<P> Id for P where P: AsRef<Path> {
    fn id(&self) -> Result<String> {
        self
        .as_ref()
        .file_stem()
        .ok_or(anyhow::anyhow!("No valid filestem"))
        .map(|fs| fs.to_string_lossy().to_string())
    }
}

pub async fn get_lyric_summary<P>(path: P) -> Result<Summary> 
where P: AsRef<Path>
{
    let s = path.as_ref().read_frontmatter().await?;
    get_item::<LyricMeta, Summary>(s, path.id()?)
}

pub fn get_item<F, G>(s: String, id: String) -> Result<G>
where
    F: std::str::FromStr<Err=Error>,
    G: From<(F, String)>,
{
    let f: F = s.parse()?;
    let g = G::from((f, id));

    Ok(g)
}

pub async fn get_playlist<P>(path: P) -> Result<Playlist>
where
    P: AsRef<Path>,
{
    let s = path.as_ref().read_string().await?;
    get_item::<PlaylistPost, Playlist>(s, path.id()?)
}

pub async fn get_list<P, T, F, Fut>(path: P, ext: &str, f: F) -> Result<Vec<T>> 
where 
    P: AsRef<Path>,
    F: FnMut(PathBuf) -> Fut,
    Fut: TryFuture<Ok=T, Error=Error>,
{
    crate::fs::get_files(path, crate::fs::extension_filter(ext))
    .await?
    .and_then(f)
    .try_collect::<Vec<T>>()
    .await
}

pub async fn post_item<D, P>(path: P, d: D) -> Result<()>
where
    D: std::fmt::Display,
    P: AsRef<Path>,
{
    tokio::fs::write(path, d.to_string()).await?;
    Ok(())
}

pub async fn delete_file<P>(path: P) -> Result<()>
where
    P: AsRef<Path>
{
    tokio::fs::remove_file(path).await?;
    Ok(())
}

pub async fn get_lyric<P>(path: P) -> Result<Lyric>
where P: AsRef<Path>
{
    let s = path.as_ref().read_string().await?;
    get_item::<LyricPost, Lyric>(s, path.id()?)
}
