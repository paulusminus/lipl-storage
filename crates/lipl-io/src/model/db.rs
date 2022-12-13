use std::collections::{HashMap};
use std::fs::{metadata, remove_file};
use std::path::{PathBuf, Path};
use tracing::info;
use lipl_core::{Lyric, LyricPost, Playlist, PlaylistPost, Uuid};
use crate::io::{fs_read, fs_write};

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

    pub fn get_lyric_list(&self) -> Vec<Lyric> {
        self.lyrics.values().cloned().collect()
    }

    pub fn get_lyric(&self, id: &Uuid) -> Option<Lyric> {
        self.lyrics.get(id).cloned()
    }

    pub fn add_lyric(&mut self, lyric: &Lyric) {
        self.lyrics.insert(lyric.id.clone(), lyric.clone());
    }

    pub fn add_lyric_post(&mut self, lyric_post: &LyricPost) -> Lyric {
        let lyric: Lyric = Lyric {
            id: Uuid::default(),
            title: lyric_post.title.clone(),
            parts: lyric_post.parts.clone(),
        };
        self.add_lyric(&lyric);
        lyric
    }

    pub fn _remove_lyric_from_playlists(&mut self, lyric_id: &Uuid) {
        for playlist in self.playlists.iter_mut() {
            playlist.1.members = playlist.1.members.iter().cloned().filter(|l| l != lyric_id).collect();
        };
    }

    pub fn delete_lyric(&mut self, id: &Uuid) -> crate::Result<()> {
        self._remove_lyric_from_playlists(id);
        self.lyrics.remove(id)
        .ok_or_else(|| crate::error::Error::NoKey("Lyric".to_owned()))
        .map(|_| {})
    }

    pub fn update_lyric(&mut self, lyric: &Lyric) -> crate::Result<Lyric> {
        let e = self.lyrics.get_mut(&lyric.id).ok_or_else(|| crate::error::Error::NoKey("".to_owned()))?;
        *e = lyric.clone();
        Ok(lyric.clone())
    }

    pub fn get_playlist_list(&self) -> Vec<Playlist> {
        self.playlists.values().cloned().collect()
    }

    pub fn get_playlist(&self, uuid: &Uuid) -> Option<Playlist> {
        self.playlists.get(uuid).cloned()
    }

    pub fn _valid_members(&self, members: &[Uuid]) -> Vec<Uuid> {
        members.iter().cloned().filter(|id| self.lyrics.contains_key(id)).collect()
    }

    pub fn add_playlist(&mut self, playlist: &mut Playlist) -> Playlist {
        playlist.members = self._valid_members(&playlist.members);
        self.playlists.insert(playlist.id.clone(), playlist.clone());
        playlist.clone()
    }

    pub fn add_playlist_post(&mut self, playlist_post: &PlaylistPost) -> Playlist {
        let mut playlist: Playlist = playlist_post.clone().into();
        playlist.members = self._valid_members(&playlist.members);
        self.playlists.insert(playlist.id.clone(), playlist.clone());
        playlist
    }

    pub fn delete_playlist(&mut self, id: &Uuid) -> crate::Result<()> {
        self.playlists.remove(id)
        .ok_or_else(|| crate::error::Error::NoKey("Playlist".to_owned()))
        .map(|_| {})
    }

    pub fn update_playlist(&mut self, playlist_update: &Playlist) -> crate::Result<Playlist> {
        let mut playlist = playlist_update.clone();
        playlist.members = self._valid_members(&playlist.members);
        let e = self.playlists.get_mut(&playlist_update.id).ok_or_else(|| crate::error::Error::NoKey("".to_owned()))?;
        *e = playlist.clone();
        Ok(playlist)
    }
}

pub enum DataType {
    Lyric(Lyric),
    Playlist(Playlist),
}

pub trait Persist {
    fn load(&mut self) -> crate::Result<()>;
    fn save(&self) -> crate::Result<()>;
    fn save_to(&self, path: &Path) -> crate::Result<()>;
    fn clear(&mut self);
}

impl Persist for Db {
    fn load(&mut self) -> crate::Result<()> {
        self.clear();
        /* if self.path.has_extension(ZIP) { 
            zip_read(self.path.clone(), self)
        }
        else */ if metadata(self.path.clone())?.is_dir() {
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
            Err(crate::error::Error::NoPath(self.path.to_path_buf()))
        }
    }

    fn clear(&mut self) {
        self.lyrics.clear();
        self.playlists.clear();
    }

    fn save(&self) -> crate::Result<()> {
        self.save_to(&self.path)
    }

    fn save_to(&self, path: &Path) -> crate::Result<()> {
        info!("{:?}", path.extension());
        /* if path.has_extension(ZIP) { 
            info!("Target is a zipfile");
            zip_write(path, self)
        }
        else */ if metadata(path)?.is_dir() {
            for file in self.files.iter() {
                remove_file(file)?;
            }
            fs_write(path, self)
        }
        else {
            Err(crate::error::Error::NoPath(path.to_path_buf()))
        }
    }
}