use lipl_core::{Lyric, Playlist, Uuid, parts::to_parts};
pub use redis_repo::RedisRepoConfig;

pub mod redis_repo;

pub fn new_lyric(title: &str, text: &str) -> Lyric {
    Lyric {
        id: Uuid::default(),
        title: title.to_owned(),
        parts: to_parts(text),
    }
}

pub fn new_playlist(title: &str, members: Vec<Uuid>) -> Playlist {
    Playlist {
        id: Uuid::default(),
        title: title.to_owned(),
        members,
    }
}
