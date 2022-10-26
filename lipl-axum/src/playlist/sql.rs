pub const LIST: &str = "SELECT * FROM playlist ORDER BY title;";
pub const ITEM_TITLE: &str = "SELECT title FROM playlist WHERE id = $1;";
pub const ITEM_MEMBERS: &str =
    "SELECT lyric_id FROM membership WHERE playlist_id = $1 ORDER BY ordering;";
pub const DELETE: &str = "DELETE FROM playlist WHERE id = $1;";
pub const UPSERT: &str = "SELECT fn_upsert_playlist($1, $2, $3);";

pub mod column {
    pub const ID: &str = "id";
    pub const LYRIC_ID: &str = "lyric_id";
    pub const TITLE: &str = "title";
}
