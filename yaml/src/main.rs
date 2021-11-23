use std::path::{Path, PathBuf};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ffi::OsStr;
use anyhow::{anyhow, Error, Result};
use serde::{Deserialize};
use tokio::fs::{read_dir, read_to_string, File};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::{LinesStream, ReadDirStream};
use futures::prelude::*;
use futures::future::ready;

const SOURCE_DIR: &str = "../data/";
const TXT_EXTENSION: &str = "txt";
const YAML_EXTENSION: &str = "yaml";
const YAML_PREFIX: &str = "---";

pub trait HasSummary {
    fn summary(&self) -> Summary;
}

pub trait Id {
    fn id(&self) -> Result<String>;
}

impl<P> Id for P where P: AsRef<Path> {
    fn id(&self) -> Result<String> {
        self
        .as_ref()
        .file_stem()
        .ok_or(anyhow!("No valid filestem"))
        .map(|fs| fs.to_string_lossy().to_string())
    }
}

#[derive(Deserialize)]
pub struct YamlMeta {
    pub title: String,
}

pub struct Summary {
    pub id: String,
    pub title: String,
}

impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}", self.id, self.title)
    }
}

#[derive(Deserialize)]
pub struct PlaylistPost {
    pub title: String,
    pub members: Vec<String>,
}

#[derive(Deserialize)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub members: Vec<String>,
}

impl HasSummary for Playlist {
    fn summary(&self) -> Summary {
        Summary {
            id: self.id.clone(),
            title: self.title.clone(),
        }
    }
}

impl Display for Playlist {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {}, [{}]", self.id, self.title, self.members.join(", "))
    }
}

pub struct Timer(pub std::time::Instant);

impl Timer {
    fn new() -> Self {
        Timer(std::time::Instant::now())
    }

    fn report_elapased(&self) {
        println!("Elapsed: {} milliseconds", self.0.elapsed().as_millis());
    }
}

fn files_filter(path_buf: &PathBuf, s: &str) -> impl Future<Output=bool> {
    ready(path_buf.extension() == Some(OsStr::new(s)))
}

fn to_std_output(s: impl std::fmt::Display) {
    println!("{}", s);
}

fn is_yaml_prefix(s: &String) -> impl TryFuture<Ok=bool, Error=Error> {
    ready(
        Ok(s.trim_end() == YAML_PREFIX)
    )
}

fn is_not_yaml_prefix(s: &String) -> impl TryFuture<Ok=bool, Error=Error> {
    ready(
        Ok(s.trim_end() != YAML_PREFIX)
    )
}

async fn get_lyric_summary<P>(path: P) -> Result<Summary> where P: AsRef<Path> {
    let file = File::open(path.as_ref()).await?;
    let lines = BufReader::new(file).lines();

    let yaml =
        LinesStream::new(lines)
        .map_err(Error::from)
        .try_skip_while(is_not_yaml_prefix)
        .try_skip_while(is_yaml_prefix)
        .try_take_while(is_not_yaml_prefix)
        .try_collect::<Vec<String>>()
        .await?
        .join("\n");

    let lyric: YamlMeta = serde_yaml::from_str(&yaml)?;

    let summary = Summary {
        id: path.id()?,
        title: lyric.title,
    };

    Ok(summary)
}

async fn get_lyric_summaries<P>(path: P) -> Result<Vec<Summary>> where P: AsRef<Path> {
    get_files(path, TXT_EXTENSION)
    .await?
    .and_then(get_lyric_summary)
    .try_collect::<Vec<Summary>>()
    .await
}

async fn get_playlist<P>(path: P) -> Result<Playlist> where P: AsRef<Path> {
    let s = read_to_string(path.as_ref()).await?;
    let playlist_post: PlaylistPost = serde_yaml::from_str(&s)?;

    let playlist = Playlist {
        id: path.id()?,
        title: playlist_post.title,
        members: playlist_post.members,
    };

    Ok(playlist)
}

async fn get_files<P>(path: P, filter: &str) -> Result<impl TryStream<Ok=PathBuf, Error=Error> + '_> where P: AsRef<Path> {
    read_dir(path)
    .await
    .map_err(Error::from)
    .map(
        |de| 
            ReadDirStream::new(de)
            .map_err(Error::from)
            .map_ok(|de| de.path())
            .try_filter(|pb| files_filter(pb, filter))
    )
}

async fn get_playlists<P>(path: P) -> Result<Vec<Playlist>> where P: AsRef<Path> {
    get_files(path, YAML_EXTENSION)
    .await?
    .and_then(get_playlist)
    .try_collect::<Vec<Playlist>>()
    .await
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()>{
    let timer = Timer::new();
    println!("Lyrics");
    get_lyric_summaries(SOURCE_DIR).await?.iter().for_each(to_std_output);
    println!();
    println!("Playlists");
    get_playlists(SOURCE_DIR).await?.iter().map(|p| p.summary()).for_each(to_std_output);
    timer.report_elapased();
    Ok(())
}
