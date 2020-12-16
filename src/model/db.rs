use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{PathBuf};
use crate::model::{LiplResult, Lyric, LyricPost, Playlist, PlaylistPost, Uuid, ZIP};
use crate::io::{fs_read, fs_write, zip_read, zip_write};

type Collection<T> = HashMap<Uuid, T>;

pub struct Db {
    lyrics: Collection<Lyric>,
    playlists: Collection<Playlist>,
    path: PathBuf,
}

impl Db {
    pub fn new(path: PathBuf) -> Self {
        Db {
            lyrics: HashMap::new(),
            playlists: HashMap::new(),
            path,
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

    pub fn delete_lyric(&mut self, id: &Uuid) {
        self._remove_lyric_from_playlists(&id);
        self.lyrics.remove(id);
    }

    pub fn update_lyric(&mut self, lyric: &Lyric) {
        self.lyrics.entry(lyric.id).and_modify(|e| *e = lyric.clone());
    }

    pub fn get_playlist_list(&self) -> Vec<&Playlist> {
        self.playlists.values().collect()
    }

    pub fn get_playlist(&self, uuid: &Uuid) -> Option<&Playlist> {
        self.playlists.get(uuid)
    }

    pub fn _valid_members(&self, members: &Vec<Uuid>) -> Vec<Uuid> {
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

    pub fn update_playlist(&mut self, playlist_update: &Playlist) -> Playlist {
        let mut playlist = playlist_update.clone();
        playlist.members = self._valid_members(&playlist.members);
        self.playlists.entry(playlist.id).and_modify(|e| *e = playlist.clone());
        playlist
    }
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
        if self.path.extension() == Some(OsStr::new(ZIP)) { 
            zip_read(self.path.clone(), self)?
        }
        else {
            fs_read(self.path.clone(), self)?
        };
        Ok(())
    }

    fn clear(&mut self) {
        self.lyrics.clear();
        self.playlists.clear();
    }

    fn save(&self) -> LiplResult<()> {
        self.save_to(self.path.clone())
    }

    fn save_to(&self, path: PathBuf) -> LiplResult<()> {
        if path.extension() == Some(OsStr::new(ZIP)) { 
            zip_write(path, self)?
        }
        else {
            fs_write(path, self)?
        };
        Ok(())
    }
}