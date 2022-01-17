// use crate::model::{Summary, Uuid, RepoResult};
use lipl_types::{Etag, Lyric, LyricMeta, Playlist, PlaylistPost, Summary, Uuid, RepoResult};

pub trait HasId {
    fn id(&self) -> Uuid;
}

impl HasId for Lyric {
    fn id(&self) -> Uuid {
        self.id.clone()
    }
}

impl HasId for Playlist {
    fn id(&self) -> Uuid {
        self.id.clone()
    }
}

pub trait HasSummary {
    fn to_summary(&self) -> Summary;
}

impl HasSummary for Lyric {
    fn to_summary(&self) -> Summary {
        Summary {
            id: self.id.clone(),
            title: self.title.clone(),
        }
    }
}

impl HasSummary for Playlist {
    fn to_summary(&self) -> Summary {
        Summary {
            id: self.id.clone(),
            title: self.title.clone(),
        }
    }
}

pub trait HasSummaries {
    fn to_summaries(&self) -> Vec<Summary>;
}

impl<T: HasSummary> HasSummaries for Vec<T> {
    fn to_summaries(&self) -> Vec<Summary> {
        self.iter().map(|l| l.to_summary()).collect()
    }
}

pub trait ToDiskFormat {
    fn to_disk_format(&self) -> RepoResult<String>;
}

impl ToDiskFormat for Lyric {
    fn to_disk_format(&self) -> RepoResult<String> {
        let front_matter = LyricMeta {
            title: self.title.clone(),
            hash: self.etag(),
        };
        let yaml = serde_yaml::to_string(&front_matter)?;
        let title_content = format!("{}---\n\n", yaml);
        Ok(format!("{}{}", title_content, parts_to_string(&self.parts)))   
    }
}

pub fn parts_to_string(parts: &[Vec<String>]) -> String {
    parts
    .iter()
    .map(|part| part.join("\n"))
    .collect::<Vec<String>>()
    .join("\n\n")
}

impl ToDiskFormat for Playlist {
    fn to_disk_format(&self) -> RepoResult<String> {
        let disk_playlist = PlaylistPost {
            title: self.title.clone(),
            members: self.members.clone()
        };
        let content = serde_yaml::to_string(&disk_playlist)?;
        Ok(content)    
    }
}

pub trait TryFromDiskFormat<T>: Sized {
    fn from_disk_format(value: T) -> RepoResult<Self>;
}

impl TryFromDiskFormat<(String, Uuid)> for Playlist {
    fn from_disk_format(value: (String, Uuid)) -> RepoResult<Playlist> {
        let pp: PlaylistPost = serde_yaml::from_str(&value.0)?;
        Ok(
            (
                Some(value.1),
                pp,
            )
            .into()
        )
    }
}
