use std::{collections::HashMap, sync::{RwLock, Arc}};

use async_trait::async_trait;
use lipl_core::{LyricDb, Lyric, LyricPost, Playlist, PlaylistPost, Summary, Uuid, PlaylistDb, Without};

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

#[async_trait]
impl LyricDb for InMemoryDb {
    type Error = crate::error::Error;

    async fn lyric_list(&self) ->  Result<Vec<Summary>, Self::Error> {
        let mut result = self.db.read().unwrap().iter().filter_map(|(key, record)| {
                if let Record::Lyric(lyric_post) = record {
                    Some(Summary { id: key.clone(), title: lyric_post.title.clone() })
                }
                else {
                    None
                }
            })
            .collect::<Vec<_>>();

        result.sort_by(|a, b| a.title.cmp(&b.title));
        Ok(result)
    }

    async fn lyric_item(&self, uuid: Uuid) -> Result<Lyric, Self::Error> {
        self.db.read().unwrap().get(&uuid)
        .and_then(|record| {
            if let Record::Lyric(lyric_post) = record { 
                Some(Lyric::from((Some(uuid.clone()), lyric_post.clone())))
            }
            else {
                None
            }    
        })
        .ok_or(crate::error::Error::NotFound)
    }

    async fn lyric_post(&self, lyric_post: LyricPost) ->  Result<Lyric, Self::Error> {
        let uuid = Uuid::default();
        match self.db.write().unwrap().insert(uuid.clone(), Record::Lyric(lyric_post.clone())) {
            Some(_) => Err(crate::error::Error::Occupied),
            None => Ok(
                Lyric::from((Some(uuid), lyric_post))
            )
        }
    }

    async fn lyric_delete(&self, uuid: Uuid) -> Result<(), Self::Error> {
        let mut db = self.db.write().unwrap();
        db.remove(&uuid).ok_or(crate::error::Error::NotFound).map(|_| ())?;
        db.iter_mut().for_each(|(uuid, record)| {
            match record {
                Record::Playlist(playlist_post) => {
                    *playlist_post = PlaylistPost {
                        title: playlist_post.title.clone(),
                        members: playlist_post.members.clone().without(&uuid)
                    }
                },
                _ => {},
            }
        });
        Ok(())
    }

    async fn lyric_put(&self, uuid: Uuid, lyric_post: LyricPost) -> Result<Lyric, Self::Error> {
        let lyric = Lyric::from((Some(uuid), lyric_post.clone()));
        self.db.write().unwrap().entry(uuid).and_modify(|v| *v = Record::Lyric(lyric_post));
        Ok(lyric)
    }
}

#[async_trait]
impl PlaylistDb for InMemoryDb {
    type Error = crate::error::Error;
    async fn playlist_list(&self) -> Result<Vec<Summary>, Self::Error> {
        let mut result = self.db.read().unwrap().iter().filter_map(|(key, record)| {
            if let Record::Playlist(playlist_post) = record {
                Some(Summary { id: key.clone(), title: playlist_post.title.clone() })
            }
            else {
                None
            }
        })
        .collect::<Vec<_>>();

        result.sort_by(|a, b| a.title.cmp(&b.title));
        Ok(result)
    }

    async fn playlist_item(&self, uuid: Uuid) -> Result<Playlist, Self::Error> {
        self.db.read().unwrap().get(&uuid)
        .and_then(|record| {
            if let Record::Playlist(playlist_post) = record { 
                Some(Playlist::from((Some(uuid.clone()), playlist_post.clone())))
            }
            else {
                None
            }    
        })
        .ok_or(crate::error::Error::NotFound)
    }

    async fn playlist_post(&self, playlist_post: PlaylistPost) -> Result<Playlist, Self::Error> {
        let uuid = Uuid::default();

        match self.db.write().unwrap().insert(uuid.clone(), Record::Playlist(playlist_post.clone())) {
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
