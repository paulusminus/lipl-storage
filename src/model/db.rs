use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

use crate::io;
use crate::model::{Lyric, Playlist, LiplResult, HasId};

pub type Db<T> = HashMap<Uuid, T>;

fn create_hashmap<T: HasId>(s: impl Iterator<Item=T>) -> HashMap<Uuid, T> {
    s
    .collect::<Vec<T>>()
    .into_iter()
    .map(|e| (e.id(), e))
    .collect()
}

pub fn create_db<P>(path: P) -> LiplResult<(Db<Lyric>, Db<Playlist>)>
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
