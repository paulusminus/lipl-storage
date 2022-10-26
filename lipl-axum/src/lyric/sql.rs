pub const LIST: &str = "SELECT * FROM lyric ORDER BY title;";
pub const ITEM: &str = "SELECT * FROM lyric WHERE id = $1;";
pub const DELETE: &str = "DELETE FROM lyric WHERE id = $1;";
pub const INSERT: &str = "INSERT INTO lyric(id, title, parts) VALUES($1, $2, $3);";
pub const UPDATE: &str = "UPDATE lyric SET title = $1, parts = $2 WHERE id = $3;";

pub mod column {
    pub const ID: &str = "id";
    pub const PARTS: &str = "parts";
    pub const TITLE: &str = "title";
}
