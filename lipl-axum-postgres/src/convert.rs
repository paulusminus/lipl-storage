use lipl_core::{ext::VecExt, Lyric, Summary, Uuid, Playlist};
use tokio_postgres::Row;

use crate::Result;

pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Result<Vec<T>>
where
    F: Fn(Row) -> Result<T> + Copy,
{
    move |rows| rows.into_iter().map(f).collect::<Result<Vec<T>>>()
}

pub fn to_lyric(row: Row) -> Result<Lyric> {
    Ok(Lyric {
        id: row.try_get::<&str, uuid::Uuid>(sql::column::ID)?.into(),
        title: row.try_get::<&str, String>(sql::column::TITLE)?,
        parts: parts::to_parts(row.try_get::<&str, String>(sql::column::PARTS)?),
    })
}

pub fn to_playlist(row: Row) -> Result<Playlist> {
    Ok(Playlist {
        id: row.try_get::<&str, uuid::Uuid>(sql::column::ID)?.into(),
        title: row.try_get::<&str, String>(sql::column::TITLE)?,
        members: row.try_get::<&str, Option<Vec<uuid::Uuid>>>(sql::column::MEMBERS)?.unwrap_or_default().map(Uuid::from),
    })
}

pub fn to_summary(row: Row) -> Result<Summary> {
    Ok(Summary {
        id: row.try_get::<&str, uuid::Uuid>(sql::column::ID)?.into(),
        title: row.try_get::<&str, String>(sql::column::TITLE)?,
    })
}

// pub fn to_unit<T>(_: T) -> Result<()> {
//     Ok(())
// }

pub fn to_inner(uuid: Uuid) -> uuid::Uuid {
    uuid.inner()
}

mod sql {
    pub mod column {
        pub const ID: &str = "id";
        pub const PARTS: &str = "parts";
        pub const TITLE: &str = "title";
        pub const MEMBERS: &str = "members";
    }
}
