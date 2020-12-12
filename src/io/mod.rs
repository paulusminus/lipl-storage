mod fs;
// mod lyric;
// mod lyric_parts;
mod playlist;
mod zip;
mod lyric_sync;
pub mod traits;

pub use fs::get_fs_files;
pub use lyric_sync::{get_lyric, get_lyrics, parts_from_reader};
pub use playlist::{get_playlist, get_playlists};
pub use self::zip::{zip_read, zip_write};
