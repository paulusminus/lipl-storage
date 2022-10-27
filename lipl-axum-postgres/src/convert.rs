use lipl_types::{Lyric, Summary, Uuid};
use tokio_postgres::Row;

use crate::ext::VecExt;

pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Vec<T>
where
    F: Fn(Row) -> T + Copy,
{
    move |rows| rows.map(f)
}

pub fn to_lyric(row: Row) -> Lyric {
    Lyric {
        id: row.get::<&str, uuid::Uuid>(sql::column::ID).into(),
        title: row.get::<&str, String>(sql::column::TITLE),
        parts: parts::to_parts(row.get::<&str, String>(sql::column::PARTS)),
    }
}

pub fn to_summary(row: Row) -> Summary {
    Summary {
        id: row.get::<&str, uuid::Uuid>(sql::column::ID).into(),
        title: row.get::<&str, String>(sql::column::TITLE),
    }
}

pub fn to_unit<T>(_: T) {}

pub fn to_title(row: Row) -> String {
    row.get::<&str, String>(sql::column::TITLE)
}

pub fn to_uuid(row: Row) -> Uuid {
    row.get::<&str, uuid::Uuid>(sql::column::LYRIC_ID).into()
}

pub fn to_inner(uuid: Uuid) -> uuid::Uuid {
    uuid.inner()
}

mod sql {
    pub mod column {
        pub const ID: &str = "id";
        pub const LYRIC_ID: &str = "lyric_id";
        pub const PARTS: &str = "parts";
        pub const TITLE: &str = "title";
    }
}