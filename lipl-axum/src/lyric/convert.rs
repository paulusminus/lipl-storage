use lipl_types::{Lyric, Summary};
use tokio_postgres::Row;

use super::sql;

pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Vec<T>
where
    F: Fn(Row) -> T + Copy,
{
    move |rows| rows.into_iter().map(f).collect::<Vec<_>>()
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
