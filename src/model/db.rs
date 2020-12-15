use std::collections::HashMap;
use std::path::{PathBuf};
use crate::model::{Lyric, LyricPost, Playlist, PlaylistPost, Uuid};

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

    pub fn add_lyric(&mut self, lyric: Lyric) {
        self.lyrics.insert(lyric.id, lyric);
    }

    pub fn add_lyric_post(&mut self, lyric_post: LyricPost) {
        let lyric: Lyric = lyric_post.into();
        self.add_lyric(lyric);
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

    pub fn add_playlist(&mut self, mut playlist: Playlist) {
        playlist.members = self._valid_members(&playlist.members);
        self.playlists.insert(playlist.id, playlist);
    }

    pub fn add_playlist_post(&mut self, playlist_post: &PlaylistPost) {
        let mut playlist: Playlist = playlist_post.clone().into();
        playlist.members = self._valid_members(&playlist.members);
        self.playlists.insert(playlist.id, playlist);
    }

    pub fn delete_playlist(&mut self, id: Uuid) {
        self.playlists.remove(&id);
    }

    pub fn update_playlist(&mut self, playlist_update: &Playlist) {
        let mut playlist = playlist_update.clone();
        playlist.members = self._valid_members(&playlist.members);
        self.playlists.entry(playlist.id).and_modify(|e| *e = playlist);
    }
}
