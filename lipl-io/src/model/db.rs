use std::collections::{HashMap};
use std::ffi::{OsStr};
use std::fs::{metadata, remove_file};
use std::path::{PathBuf};
use crate::model::{LiplResult, LiplError, Lyric, LyricPost, Playlist, PlaylistPost, Uuid, ZIP};
use crate::io::{fs_read, fs_write, zip_read, zip_write};

type Collection<T> = HashMap<Uuid, T>;

pub struct Db {
    lyrics: Collection<Lyric>,
    playlists: Collection<Playlist>,
    path: PathBuf,
    files: Vec<PathBuf>,
}

impl Db {
    pub fn new(path: PathBuf) -> Self {
        Db {
            lyrics: HashMap::new(),
            playlists: HashMap::new(),
            path,
            files: vec![],
        }
    }

    pub fn get_lyric_list(&self) -> Vec<&Lyric> {
        self.lyrics.values().collect()
    }

    pub fn get_lyric(&self, id: &Uuid) -> Option<&Lyric> {
        self.lyrics.get(id)
    }

    pub fn add_lyric(&mut self, lyric: &Lyric) {
        self.lyrics.insert(lyric.id, lyric.clone());
    }

    pub fn add_lyric_post(&mut self, lyric_post: LyricPost) -> Lyric {
        let lyric: Lyric = lyric_post.into();
        self.add_lyric(&lyric);
        lyric
    }

    pub fn _remove_lyric_from_playlists(&mut self, lyric_id: &Uuid) {
        for playlist in self.playlists.iter_mut() {
            playlist.1.members = playlist.1.members.iter().cloned().filter(|l| l != lyric_id).collect();
        };
    }

    pub fn delete_lyric(&mut self, id: &Uuid) -> LiplResult<()> {
        self._remove_lyric_from_playlists(&id);
        self.lyrics.remove(id)
        .ok_or_else(|| LiplError::NoKey("Lyric".to_owned()))
        .map(|_| {})
    }

    pub fn update_lyric(&mut self, lyric: &Lyric) -> LiplResult<()> {
        let e = self.lyrics.get_mut(&lyric.id).ok_or_else(|| LiplError::NoKey("".to_owned()))?;
        *e = lyric.clone();
        Ok(())
    }

    pub fn get_playlist_list(&self) -> Vec<&Playlist> {
        self.playlists.values().collect()
    }

    pub fn get_playlist(&self, uuid: &Uuid) -> Option<&Playlist> {
        self.playlists.get(uuid)
    }

    pub fn _valid_members(&self, members: &[Uuid]) -> Vec<Uuid> {
        members.iter().cloned().filter(|id| self.lyrics.contains_key(id)).collect()
    }

    pub fn add_playlist(&mut self, playlist: &mut Playlist) -> Playlist {
        playlist.members = self._valid_members(&playlist.members);
        self.playlists.insert(playlist.id, playlist.clone());
        playlist.clone()
    }

    pub fn add_playlist_post(&mut self, playlist_post: &PlaylistPost) -> Playlist {
        let mut playlist: Playlist = playlist_post.clone().into();
        playlist.members = self._valid_members(&playlist.members);
        self.playlists.insert(playlist.id, playlist.clone());
        playlist
    }

    pub fn delete_playlist(&mut self, id: &Uuid) -> Option<Playlist> {
        self.playlists.remove(id)
    }

    pub fn update_playlist(&mut self, playlist_update: &Playlist) -> LiplResult<Playlist> {
        let mut playlist = playlist_update.clone();
        playlist.members = self._valid_members(&playlist.members);
        let e = self.playlists.get_mut(&playlist_update.id).ok_or_else(|| LiplError::NoKey("".to_owned()))?;
        *e = playlist.clone();
        Ok(playlist)
    }
}

pub enum DataType {
    Lyric(Lyric),
    Playlist(Playlist),
}

pub trait Persist {
    fn load(&mut self) -> LiplResult<()>;
    fn save(&self) -> LiplResult<()>;
    fn save_to(&self, path: PathBuf) -> LiplResult<()>;
    fn clear(&mut self);
}

impl Persist for Db {
    fn load(&mut self) -> LiplResult<()> {
        self.clear();
        if metadata(self.path.clone())?.is_file() && self.path.extension() == Some(OsStr::new(ZIP)) { 
            zip_read(self.path.clone(), self)
        }
        else if metadata(self.path.clone())?.is_dir() {
            fs_read(
                self.path.clone(), 
                |pathbuf, item| {
                    match item {
                        DataType::Lyric(lyric) => {
                            self.add_lyric(lyric);
                        },
                        DataType::Playlist(playlist) => {
                            self.add_playlist(playlist);
                        }
                    } 
                    self.files.push(pathbuf.clone());
                },
            )
        }
        else {
            Err(LiplError::NoPath(self.path.clone().to_string_lossy().to_owned().to_string()))
        }
    }

    fn clear(&mut self) {
        self.lyrics.clear();
        self.playlists.clear();
    }

    fn save(&self) -> LiplResult<()> {
        self.save_to(self.path.clone())
    }

    fn save_to(&self, path: PathBuf) -> LiplResult<()> {
        if metadata(&path)?.is_file() && (&path).extension() == Some(OsStr::new(ZIP)) { 
            zip_write(&path, self)
        }
        else if metadata(&path)?.is_dir() {
            // TODO: remove files first
            for file in self.files.iter() {
                remove_file(file)?;
            }
            fs_write(&path, self)
        }
        else {
            Err(LiplError::NoPath((&path).to_string_lossy().to_owned().to_string()))
        }
    }
}