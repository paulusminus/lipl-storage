use std::{collections::HashMap, sync::{RwLock, Arc}, cmp::Ordering};

use async_trait::async_trait;
use lipl_core::{LyricDb, Lyric, LyricPost, Playlist, PlaylistPost, Summary, Uuid, PlaylistDb, Without};
use crate::error::Error;

mod error;

#[derive(Clone)]
enum Record {
    Lyric(LyricPost),
    Playlist(PlaylistPost),
}

#[derive(Clone)]
pub struct InMemoryDb {
    db: Arc<RwLock<HashMap<Uuid, Record>>>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryDb {
    fn default() -> Self {
        Self::new()
    }
}

fn compare_title(a: &Summary, b: &Summary) -> Ordering {
    a.title.cmp(&b.title)
}

fn lyric_compare_title(a: &Lyric, b: &Lyric) -> Ordering {
    a.title.cmp(&b.title)
}

fn playlist_compare_title(a: &Playlist, b: &Playlist) -> Ordering {
    a.title.cmp(&b.title)
}

#[async_trait]
impl LyricDb for InMemoryDb {
    type Error = Error;

    async fn lyric_list(&self) ->  Result<Vec<Summary>, Self::Error> {
        let mut summaries = self.db.read().unwrap().iter().filter_map(|(key, record)| {
                if let Record::Lyric(lyric_post) = record {
                    Some(Summary { id: *key, title: lyric_post.title.clone() })
                }
                else {
                    None
                }
            })
            .collect::<Vec<_>>();

        summaries.sort_by(compare_title);
        Ok(summaries)
    }

    async fn lyric_list_full(&self) ->  Result<Vec<Lyric>, Self::Error> {
        let mut lyrics = self.db.read().unwrap().iter().filter_map(|(key, record)| {
                if let Record::Lyric(lyric_post) = record {
                    Some(Lyric::from((Some(*key), lyric_post.clone())))
                }
                else {
                    None
                }
            })
            .collect::<Vec<_>>();

        lyrics.sort_by(lyric_compare_title);
        Ok(lyrics)
    }

    async fn lyric_item(&self, uuid: Uuid) -> Result<Lyric, Self::Error> {
        self.db.read().unwrap().get(&uuid)
        .and_then(|record| {
            match record {
                Record::Lyric(lyric_post) => Some(Lyric::from((Some(uuid), lyric_post.clone()))),
                _ => None
            }
        })
        .ok_or(crate::error::Error::NotFound)
    }

    async fn lyric_post(&self, lyric_post: LyricPost) ->  Result<Lyric, Self::Error> {
        let uuid = Uuid::default();
        match self.db.write().unwrap().insert(uuid, Record::Lyric(lyric_post.clone())) {
            Some(_) => Err(crate::error::Error::Occupied),
            None => Ok(
                Lyric::from((Some(uuid), lyric_post))
            )
        }
    }

    async fn lyric_delete(&self, uuid: Uuid) -> Result<(), Self::Error> {
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
            Err(crate::error::Error::NotFound)
        }
    }

    async fn lyric_put(&self, uuid: Uuid, lyric_post: LyricPost) -> Result<Lyric, Self::Error> {
        let mut db = self.db.write().unwrap();
        let lyric = Lyric::from((Some(uuid), lyric_post.clone()));
        let entry = db.get_mut(&uuid).ok_or(Error::NotFound)?;
        *entry = Record::Lyric(lyric_post);
        Ok(lyric)
    }
}

#[async_trait]
impl PlaylistDb for InMemoryDb {
    type Error = crate::error::Error;
    async fn playlist_list(&self) -> Result<Vec<Summary>, Self::Error> {
        let mut summaries = self.db.read().unwrap().iter().filter_map(|(key, record)| {
            match record {
                Record::Playlist(playlist_post) => Some(Summary { id: *key, title: playlist_post.title.clone() }),
                _ => None
            }
        })
        .collect::<Vec<_>>();

        summaries.sort_by(compare_title);
        Ok(summaries)
    }

    async fn playlist_list_full(&self) -> Result<Vec<Playlist>, Self::Error> {
        let mut playlists = self.db.read().unwrap().iter().filter_map(|(key, record)| {
            match record {
                Record::Playlist(playlist_post) => Some(Playlist::from((Some(*key), playlist_post.clone()))),
                _ => None
            }
        })
        .collect::<Vec<_>>();

        playlists.sort_by(playlist_compare_title);
        Ok(playlists)
    }

    async fn playlist_item(&self, uuid: Uuid) -> Result<Playlist, Self::Error> {
        self.db.read().unwrap().get(&uuid)
        .and_then(|record| {
            match record {
                Record::Playlist(playlist_post) => Some(Playlist::from((Some(uuid), playlist_post.clone()))),
                _ => None,
            }
        })
        .ok_or(crate::error::Error::NotFound)
    }

    async fn playlist_post(&self, playlist_post: PlaylistPost) -> Result<Playlist, Self::Error> {
        let uuid = Uuid::default();

        match self.db.write().unwrap().insert(uuid, Record::Playlist(playlist_post.clone())) {
            Some(_) => Err(crate::error::Error::Occupied),
            None => Ok(Playlist::from((Some(uuid), playlist_post)))
        }
    }

    async fn playlist_delete(&self, uuid: Uuid) -> Result<(), Self::Error> {
        self.db.write().unwrap().remove(&uuid).ok_or(crate::error::Error::NotFound).map(|_| ())
    }

    async fn playlist_put(&self, uuid: Uuid, playlist_post: PlaylistPost) -> Result<Playlist, Self::Error> {
        let playlist = Playlist::from((Some(uuid), playlist_post.clone()));
        self.db.write().unwrap().entry(uuid).and_modify(|v| *v = Record::Playlist(playlist_post));
        Ok(playlist)
    }
}


#[cfg(test)]
mod tests {
    use lipl_core::PlaylistDb;

    #[tokio::test]
    async fn post_playlist() {
        let db = super::InMemoryDb::new();

        let playlist_post = super::PlaylistPost {
            title: "Alle 13 goed".to_owned(),
            members: vec![],
        };

        let playlist = db.playlist_post(playlist_post).await.unwrap();

        let playlists = db.playlist_list().await.unwrap();
        assert_eq!(playlists[0].title, "Alle 13 goed".to_owned());
        assert_eq!(playlists[0].id, playlist.id);
    }
}
