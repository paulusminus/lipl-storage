pub const PREFIX: &str = "/api/v1";
pub const DEFAULT_LOG_FILTER: &str = "info,tower_http=debug,tokio_postgres=warn";
pub const PG_CONNECTION: &str = "host=/run/postgresql dbname=test user=paul";
pub const RUST_LOG: &str = "RUST_LOG";
pub const LOCALHOST: [u8; 4] = [127, 0, 0, 1];
pub const PORT: u16 = 3000;

pub mod sql {
    pub mod lyric {
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
    }

    pub mod playlist {
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
    }
}
