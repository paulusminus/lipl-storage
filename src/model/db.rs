use std::collections::HashMap;
use std::path::Path;
use std::io::Error;
use uuid::Uuid;

use crate::io;
use crate::model;

pub type Db<T> = HashMap<Uuid, T>;

fn create_hashmap<T: model::HasId>(s: impl Iterator<Item=T>) -> HashMap<Uuid, T> {
    s
    .collect::<Vec<T>>()
    .into_iter()
    .map(|e| (e.id(), e))
    .collect()
}

pub fn create_db<P>(path: P) -> Result<(Db<model::Lyric>, Db<model::Playlist>), Error>
where P: AsRef<Path> 
{
    let hm_lyrics = create_hashmap(io::get_lyrics(&path)?);
    let hm_playlists = create_hashmap(io::get_playlists(&path)?);
    Ok(
        (
            hm_lyrics,
            hm_playlists,
        )
    )
}
