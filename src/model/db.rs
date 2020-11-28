use std::collections::HashMap;
use std::path::Path;
use std::io::Error;
use futures::stream::{Stream, StreamExt};
use uuid::Uuid;

use crate::io;
use crate::model;

pub type Db<T> = HashMap<Uuid, T>;

async fn create_hashmap<T: model::HasId>(s: impl Stream<Item=T>) -> HashMap<Uuid, T> {
    s
    .collect::<Vec<T>>()
    .await
    .into_iter()
    .map(|e| (e.id(), e))
    .collect()
}

pub async fn create_db<P: AsRef<Path>>(path: P) -> Result<(Db<model::Lyric>, Db<model::Playlist>), Error> {
    let hm_lyrics = create_hashmap(io::get_lyrics(&path).await?).await;
    let hm_playlists = create_hashmap(io::get_playlists(&path).await?).await;
    Ok(
        (
            hm_lyrics,
            hm_playlists,
        )
    )
}
