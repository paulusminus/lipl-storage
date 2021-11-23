use std::path::{Path, PathBuf};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ffi::OsStr;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::fs::{read_dir, read_to_string, File};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
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

#[async_trait]
pub trait LiplRepo {
    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>>;
    async fn get_lyric(&self, id: String) -> Result<Lyric>;
    async fn post_lyric(&self, lyric: Lyric) -> Result<()>;
    async fn delete_lyric(&self, id: String) -> Result<()>;
    async fn get_playlists(&self) -> Result<Vec<Playlist>>;
    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>>;
    async fn get_playlist(&self, id: String) -> Result<Playlist>;
    async fn post_playlist(&self, playlist: Playlist) -> Result<()>;
    async fn delete_playlist(&self, id: String) -> Result<()>;
}

pub struct FileSystem<'a> {
    source_dir: &'a str
}

impl<'a> FileSystem<'a> {
    pub fn new(s: &'a str) -> Self {
        FileSystem {
            source_dir: s
        }
    }
}

#[async_trait]
impl<'a> LiplRepo for FileSystem<'a> {
    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        get_lyric_summaries(self.source_dir).await
    }

    async fn get_lyric(&self, id: String) -> Result<Lyric> {
        get_lyric(
            self.join(&id, TXT_EXTENSION)
        )
        .await
    }

    async fn post_lyric(&self, lyric: Lyric) -> Result<()> {
        post_lyric(self.join(&lyric.id, TXT_EXTENSION), lyric).await
    }

    async fn delete_lyric(&self, id: String) -> Result<()> {
        delete_file(
            self.join(&id, TXT_EXTENSION)
        )
        .await?;
        let playlists = self.get_playlists().await?;
        for mut playlist in playlists {
            if playlist.members.contains(&id) {
                playlist.members = playlist.members.into_iter().filter(|m| *m != id).collect();
                self.post_playlist(playlist).await?;
            }
        }
        Ok(())
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        get_playlists(self.source_dir).await
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.get_playlists()
        .await
        .map(
            |playlists| playlists.iter().map(to_summary).collect::<Vec<Summary>>()
        )
    }

    async fn get_playlist(&self, id: String) -> Result<Playlist> {
        get_playlist(
            self.join(&id, YAML_EXTENSION)
        )
        .await
    }

    async fn post_playlist(&self, playlist: Playlist) -> Result<()> {
        let lyric_ids: Vec<String> = self.get_lyric_summaries().await?.into_iter().map(|s| s.id).collect();
        for member in playlist.members.iter() {
            if !lyric_ids.contains(member) {
                return Err(anyhow!("Playlist contains invalid member"));
            }
        }
        post_playlist(self.join(&playlist.id, YAML_EXTENSION), playlist).await
    }

    async fn delete_playlist(&self, id: String) -> Result<()> {
        delete_file(
            self.join(&id, YAML_EXTENSION)
        )
        .await
    }
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

pub trait JoinPath {
    fn join(&self, id: &str, ext: &str) -> PathBuf;
}

impl<'a> JoinPath for FileSystem<'a> {
    fn join(&self, id: &str, ext: &str) -> PathBuf {
        Path::new(self.source_dir).join(format!("{}.{}", id, ext))
    }
} 

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
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

#[derive(Debug)]
pub struct Lyric {
    pub id: String,
    pub title: String,
    pub parts: Vec<Vec<String>>
}

pub struct Timer(pub std::time::Instant);

impl Timer {
    fn new() -> Self {
        Timer(std::time::Instant::now())
    }

    fn report_elapsed(&self) {
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

fn to_summary<T>(t: &T) -> Summary where T: HasSummary {
    t.summary()
}

async fn get_lyric<P>(path: P) -> Result<Lyric>
where P: AsRef<Path>
{
    let file = File::open(path.as_ref()).await?;
    let lines = BufReader::new(file).lines();
    let mut lines_stream = LinesStream::new(lines).boxed();

    let mut new_part = true;
    let mut parts: Vec<Vec<String>> = vec![];
    let mut yaml: Option<String> = None;
    let mut yaml_start: bool = false;
    let mut line_no = 0;

    while let Some(line) = lines_stream.try_next().await? {
        line_no += 1;
        if line == *"---" {
            if line_no == 1 {
                yaml_start = true;
                yaml = Some("".to_owned());
                continue;
            }
            else if yaml_start {
                yaml_start = false;
                continue;
            }
        }

        if yaml_start {
            if let Some(v) = yaml.as_mut() {
                v.extend(vec![line, "\n".into()]);
            }
            continue;
        }
        
        if line.trim().is_empty() {
            new_part = true;
            continue;
        }
        
        if new_part {
            parts.push(vec![line.trim().into()]);
            new_part = false;
            continue;
        }

        parts.last_mut().unwrap().push(line.trim().into());
    }

    let frontmatter = 
        yaml
        .and_then(|text| 
            serde_yaml::from_str::<YamlMeta>(&text).ok()
        )
        .ok_or(anyhow!("Cannot deserialize frontmatter"))?;

    Ok(
        Lyric {
            id: path.id()?,
            title: frontmatter.title,
            parts,
        }
    )
}

async fn post_lyric<P>(path: P, lyric: Lyric) -> Result<()> where P: AsRef<Path> {
    let file = File::create(path).await?;
    let mut writer = BufWriter::new(file);

    let meta = YamlMeta {
        title: lyric.title
    };
    let yaml = serde_yaml::to_string(&meta)?;
    let mut _count = writer.write_all(yaml.as_bytes()).await?;

    _count = writer.write_all(YAML_PREFIX.as_bytes()).await?;

    for part in lyric.parts {
        _count = writer.write_all("\n\n".as_bytes()).await?;
        _count = writer.write_all(part.join("\n").as_bytes()).await?;
    }

    writer.flush().await?;

    Ok(())
}

async fn post_playlist<P>(path: P, playlist: Playlist) -> Result<()> where P: AsRef<Path> {
    let file = File::create(path).await?;
    let mut writer = BufWriter::new(file);

    let playlist_post = PlaylistPost {
        title: playlist.title,
        members: playlist.members,
    };

    let out = serde_yaml::to_string(&playlist_post)?;
    writer.write_all(out.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}

async fn delete_file<P>(path: P) -> Result<()> where P: AsRef<Path> {
    tokio::fs::remove_file(path).await.map_err(Error::from)
}

async fn time_it<T, F, O>(process: F) -> Result<T> 
where 
    F: Fn() -> O,
    O: Future<Output=Result<T>>
{
    let timer = Timer::new();
    let result = process().await?;
    timer.report_elapsed();
    Ok(result)
}

async fn process() -> Result<()> {
    if !Path::new(SOURCE_DIR).is_dir() { return Err(anyhow!("cannot find directory {}", SOURCE_DIR)) }
    let repo = FileSystem::new(SOURCE_DIR);

    println!("Lyrics");
    repo.get_lyric_summaries().await?.iter().for_each(to_std_output);
    println!();

    println!("Playlists");
    repo.get_playlist_summaries().await?.iter().for_each(to_std_output);

    repo.delete_lyric("KGxasqUC1Uojk1viLGbMZK".to_owned()).await?;

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()>{
    time_it(process).await
}
