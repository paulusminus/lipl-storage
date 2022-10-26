use super::sql;
use lipl_types::{Summary, Uuid};
use tokio_postgres::Row;

pub trait VecExt<T> {
    fn map<F, R>(self, f: F) -> Vec<R> where F: Fn(T) -> R;
}

impl<T> VecExt<T> for Vec<T> {
    fn map<F, R>(self, f: F) -> Vec<R> where F: Fn(T) -> R {
        self.into_iter().map(f).collect::<Vec<_>>()
    }
}

pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Vec<T>
where
    F: Fn(Row) -> T + Copy,
{
    move |rows| rows.map(f)
}

pub fn to_summary(row: Row) -> Summary {
    Summary {
        id: row.get::<&str, uuid::Uuid>(sql::column::ID).into(),
        title: row.get::<&str, String>(sql::column::TITLE),
    }
}

pub fn to_title(row: Row) -> String {
    row.get::<&str, String>(sql::column::TITLE)
}

pub fn to_uuid(row: Row) -> Uuid {
    row.get::<&str, uuid::Uuid>(sql::column::LYRIC_ID).into()
}

pub fn to_inner(uuid: Uuid) -> uuid::Uuid {
    uuid.inner()
}
