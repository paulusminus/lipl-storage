use lipl_core::{RedisRepoError, Lyric, Playlist, Uuid};
use parts::to_parts;
pub use redis_repo::{RedisRepoConfig};

mod redis_repo;

type Result<T> = std::result::Result<T, RedisRepoError>;

pub fn new_lyric(title: &str, text: &str) -> Lyric {
    Lyric {
        id: Uuid::default(),
        title: title.to_owned(),
        parts: to_parts(text.to_owned()),
    }
}

pub fn new_playlist(title: &str, members: Vec<Uuid>) -> Playlist {
    Playlist {
        id: Uuid::default(),
        title: title.to_owned(),
        members,
    }
}
