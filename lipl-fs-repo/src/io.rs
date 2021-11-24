use std::collections::hash_map::{DefaultHasher};
use std::hash::{Hash, Hasher};
use std::path::{Path};
use futures::prelude::*;
use futures::future::ready;
use tokio::fs::{read_to_string, File};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio_stream::wrappers::{LinesStream};
use crate::{Error, Lyric, Playlist, PlaylistPost, Result, Summary, YamlMeta};

const YAML_PREFIX: &str = "---";

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

fn calculate_hash<T>(obj: T) -> u64 where T: Hash {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

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

pub async fn get_lyric_summary<P>(path: P) -> Result<Summary> where P: AsRef<Path> {
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

pub async fn get_playlist<P>(path: P) -> Result<Playlist> where P: AsRef<Path> {
    let s = read_to_string(path.as_ref()).await?;
    let playlist_post: PlaylistPost = serde_yaml::from_str(&s)?;

    let playlist = Playlist {
        id: path.id()?,
        title: playlist_post.title,
        members: playlist_post.members,
    };

    Ok(playlist)
}

pub async fn get_list<P, T, F, Fut>(path: P, ext: &str, f: F) -> Result<Vec<T>> 
where 
    P: AsRef<Path>,
    F: FnMut(std::path::PathBuf) -> Fut,
    Fut: TryFuture<Ok=T, Error=Error>,
{
    crate::fs::get_files(path, crate::fs::extension_filter(ext))
    .await?
    .and_then(f)
    .try_collect::<Vec<T>>()
    .await
}

pub async fn post_lyric<P>(path: P, lyric: Lyric) -> Result<()> where P: AsRef<Path> {
    let file = File::create(path).await?;
    let mut writer = BufWriter::new(file);

    let hash = calculate_hash(&lyric);
    let meta = YamlMeta {
        title: lyric.title,
        hash: Some(hash),
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

pub async fn post_playlist<P>(path: P, playlist: Playlist) -> Result<()> where P: AsRef<Path> {
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

pub async fn delete_file<P>(path: P) -> Result<()> where P: AsRef<Path> {
    tokio::fs::remove_file(path).await.map_err(Error::from)
}

pub async fn get_lyric<P>(path: P) -> Result<Lyric>
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
        .ok_or(anyhow::anyhow!("Cannot deserialize frontmatter"))?;

    Ok(
        Lyric {
            id: path.id()?,
            title: frontmatter.title,
            parts,
        }
    )
}
