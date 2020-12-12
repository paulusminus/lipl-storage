use serde::{Deserialize, Serialize};

mod db;
mod err;
mod lyric;
mod pathbuf_ext;
mod playlist;
mod serde_uuid;
mod serde_vec_uuid;
mod summary;
mod traits;
mod uuid_ext;

pub use db::{create_db};
pub use err::{LiplError, LiplResult};
pub use lyric::{Lyric, LyricPost};
pub use pathbuf_ext::PathBufExt;
pub use playlist::{Playlist, PlaylistPost};
pub use summary::{Summary};
pub use traits::{HasId, HasSummary};
pub use uuid_ext::UuidExt;

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: Option<String>,
}
