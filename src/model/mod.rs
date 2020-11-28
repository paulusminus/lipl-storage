use serde::{Deserialize, Serialize};

mod db;
mod lyric;
mod pathbuf_ext;
mod playlist;
mod serde_uuid;
mod serde_vec_uuid;
mod summary;
mod traits;
mod uuid;
mod uuid_ext;

pub use db::{create_db};
pub use lyric::{Lyric, LyricPost};
pub use pathbuf_ext::PathBufExt;
pub use playlist::{Playlist, PlaylistPost};
pub use summary::{Summary};
pub use uuid_ext::UuidExt;
pub use traits::{HasId, HasSummary};

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: Option<String>,
}
