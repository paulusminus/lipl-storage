use std::{collections::HashMap, sync::{RwLock, Arc}, iter::empty};
use async_trait::async_trait;
use lipl_core::{
    Error,
    LiplRepo,
    Lyric,
    LyricPost,
    Playlist,
    PlaylistPost,
    Result,
    Summary,
    Uuid,
    Yaml,
    RepoDb,
    reexport::serde_yaml, by_title, ToRepo, HasSummary,
};
use lipl_core::vec_ext::VecExt;

#[derive(Clone)]
enum Record {
    Lyric(LyricPost),
    Playlist(PlaylistPost),
}

#[derive(Clone, Default)]
pub struct MemoryRepoConfig {
    pub sample_data: bool,
    pub transaction_log: Option<Arc<dyn std::io::Read + Send + Sync>>
}

impl std::str::FromStr for MemoryRepoConfig {
    type Err = lipl_core::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Ok(Self { sample_data: false, transaction_log: None})
        }
        else {
            s.trim().parse::<bool>()
                .map(|sample_data| Self { sample_data, transaction_log: None })
                .map_err(|_| lipl_core::Error::Argument("must be false or true"))
        }
    }
}

#[async_trait]
impl ToRepo for MemoryRepoConfig {
    async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        if self.sample_data {
            let repo = lipl_sample_data::repo_db();
            Ok(
                Arc::new(MemoryRepo::from(repo))
            )
        }
        else {
            Ok(
                Arc::new(MemoryRepo::new(empty(), empty()))
            )
        }
    }
}

#[derive(Clone)]
pub struct MemoryRepo {
    db: Arc<RwLock<HashMap<Uuid, Record>>>,
}

impl From<RepoDb> for MemoryRepo {
    fn from(repo_db: RepoDb) -> Self {
        MemoryRepo::new(repo_db.lyrics.into_iter(), repo_db.playlists.into_iter())
    }
}

fn lyric_to_tuple(lyric: Lyric) -> (Uuid, Record) {
    (lyric.id, Record::Lyric(lyric.into()))
}

fn playlist_to_tuple(playlist: Playlist) -> (Uuid, Record) {
    (playlist.id, Record::Playlist(playlist.into()))
}

impl MemoryRepo {
    pub fn new(lyrics: impl Iterator<Item = Lyric>, playlists: impl Iterator<Item = Playlist>) -> Self {
        Self {
            db: Arc::new(
                RwLock::new(
                    HashMap::from_iter(
                        lyrics.map(lyric_to_tuple).chain(playlists.map(playlist_to_tuple)),
                    )
                )
            ),
        }
    }

    fn to_repo_db(&self) -> RepoDb {
        self.db.read().unwrap()
            .iter()
            .fold(
                (Vec::<Lyric>::new(), Vec::<Playlist>::new()),
                |acc, (uuid, record)| {
                    match record {
                        Record::Lyric(lyric_post) =>
                            (
                                acc.0.add_one((Some(*uuid), lyric_post.clone()).into()),
                                acc.1,
                            ),
                        Record::Playlist(playlist_post) =>
                            (
                                acc.0,
                                acc.1.add_one((Some(*uuid), playlist_post.clone()).into()),
                            )
                    }
                }
            )
            .into()
    }
}

impl Default for MemoryRepo {
    fn default() -> Self {
        Self::new(empty(), empty())
    }
}

impl Yaml for MemoryRepo {
    fn load<R>(r: R) -> Result<Self>
    where 
        R: std::io::Read,
        Self: Sized,
    {
        serde_yaml::from_reader::<_, RepoDb>(r)
            .map_err(Into::into)
            .map(MemoryRepo::from)
    }

    fn save<W>(&self, w: W) -> Result<()>
    where
        W: std::io::Write,
    {
        serde_yaml::to_writer(w, &self.to_repo_db())
            .map_err(Into::into)
    }
}

#[async_trait]
impl LiplRepo for MemoryRepo {
    async fn get_lyric_summaries(&self) ->  Result<Vec<Summary>> {
        self.get_lyrics()
            .await
            .map(|lyrics| lyrics.map(|lyric| lyric.summary())
        )
    }

    async fn get_lyrics(&self) ->  Result<Vec<Lyric>> {
        let mut lyrics = self.db.read().unwrap().iter().filter_map(|(key, record)| {
                if let Record::Lyric(lyric_post) = record {
                    Some(Lyric::from((Some(*key), lyric_post.clone())))
                }
                else {
                    None
                }
            })
            .collect::<Vec<_>>();

        lyrics.sort_by(by_title);
        Ok(lyrics)
    }

    async fn get_lyric(&self, uuid: Uuid) -> Result<Lyric> {
        self.db.read().unwrap()
        .get(&uuid)
        .and_then(|record| {
            match record {
                Record::Lyric(lyric_post) => Some(Lyric::from((Some(uuid), lyric_post.clone()))),
                _ => None
            }
        })
        .ok_or(Error::NotFound(uuid))
    }

    async fn upsert_lyric(&self, lyric: Lyric) ->  Result<Lyric> {
        self.db.write().unwrap()
            .entry(lyric.clone().id)
            .and_modify(|lyric_post| *lyric_post = Record::Lyric(lyric.clone().into()))
            .or_insert_with(|| Record::Lyric(lyric.clone().into()));
        Ok(lyric)
    }

    async fn delete_lyric(&self, uuid: Uuid) -> Result<()> {
        let mut db = self.db.write().unwrap();
        if db.remove(&uuid).is_some() {
            db.iter_mut().for_each(|(_, record)| {
                if let Record::Playlist(playlist_post) = record {
                    *playlist_post = PlaylistPost {
                        title: playlist_post.title.clone(),
                        members: playlist_post.members.clone().without(&uuid)
                    }
                }
            });
            Ok(())
        }
        else {
            Err(Error::NotFound(uuid))
        }
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.get_playlists()
            .await
            .map(|playlists| playlists.map(|p| p.summary()))
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        let mut playlists = self.db.read().unwrap().iter().filter_map(|(key, record)| {
            match record {
                Record::Playlist(playlist_post) => Some(Playlist::from((Some(*key), playlist_post.clone()))),
                _ => None
            }
        })
        .collect::<Vec<_>>();

        playlists.sort_by(by_title);
        Ok(playlists)
    }

    async fn get_playlist(&self, uuid: Uuid) -> Result<Playlist> {
        self.db.read().unwrap().get(&uuid)
        .and_then(|record| {
            match record {
                Record::Playlist(playlist_post) => Some(Playlist::from((Some(uuid), playlist_post.clone()))),
                _ => None,
            }
        })
        .ok_or(Error::NotFound(uuid))
    }

    async fn upsert_playlist(&self, playlist: Playlist) -> Result<Playlist> {
        self.db.write().unwrap()
            .entry(playlist.clone().id)
            .and_modify(|record| *record = Record::Playlist(playlist.clone().into()))
            .or_insert_with(|| Record::Playlist(playlist.clone().into()));
        Ok(playlist) 
    }

    async fn delete_playlist(&self, uuid: Uuid) -> Result<()> {
        self.db.write().unwrap().remove(&uuid).ok_or(Error::NotFound(uuid)).map(|_| ())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{MemoryRepo};
    use lipl_core::{LiplRepo, PlaylistPost, LyricPost};

    #[tokio::test]
    async fn post_lyric() {
        let db = MemoryRepo::default();

        let lyric_post = LyricPost {
            title: "Alle 13 goed".to_owned(),
            parts: vec![],
        };

        let lyric = db.upsert_lyric((None, lyric_post).into()).await.unwrap();

        let lyrics = db.get_lyrics().await.unwrap();
        assert_eq!(lyrics[0].title, "Alle 13 goed".to_owned());
        assert_eq!(lyrics[0].id, lyric.id);
    }

    #[tokio::test]
    async fn post_lyric_change() {
        let db = MemoryRepo::default();

        let lyric_post = LyricPost {
            title: "Alle 13 goed".to_owned(),
            parts: vec![],
        };

        let mut lyric = db.upsert_lyric((None, lyric_post).into()).await.unwrap();
        let mut lyrics = db.get_lyrics().await.unwrap();
        assert_eq!(lyrics[0].title, "Alle 13 goed".to_owned());
        assert_eq!(lyrics[0].id, lyric.id);

        lyric.title = "Alle 15 goed".to_owned();
        lyric = db.upsert_lyric(lyric).await.unwrap();
        lyrics = db.get_lyrics().await.unwrap();
        assert_eq!(lyrics[0].title, "Alle 15 goed".to_owned());
        assert_eq!(lyrics[0].id, lyric.id);
    }

    #[tokio::test]
    async fn post_playlist() {
        let db = MemoryRepo::default();

        let playlist_post = PlaylistPost {
            title: "Alle 13 goed".to_owned(),
            members: vec![],
        };

        let playlist = db.upsert_playlist((None, playlist_post).into()).await.unwrap();

        let playlists = db.get_playlists().await.unwrap();
        assert_eq!(playlists[0].title, "Alle 13 goed".to_owned());
        assert_eq!(playlists[0].id, playlist.id);
    }
}
