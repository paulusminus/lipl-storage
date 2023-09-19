use lipl_core::{reexport, Lyric, Summary, Uuid, Playlist};
use lipl_util::VecExt;
use tokio_postgres::Row;
use crate::{postgres_error, Result};

pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Result<Vec<T>>
where
    F: Fn(Row) -> Result<T> + Copy,
{
    move |rows| rows.try_map(f)
}

pub fn to_lyric(row: Row) -> Result<Lyric> {
    Ok(Lyric {
        id: row.try_get::<&str, reexport::uuid::Uuid>(column::ID).map_err(postgres_error)?.into(),
        title: row.try_get::<&str, String>(column::TITLE).map_err(postgres_error)?,
        parts: parts::to_parts(row.try_get::<&str, String>(column::PARTS).map_err(postgres_error)?),
    })
}

pub fn to_playlist(row: Row) -> Result<Playlist> {
    Ok(Playlist {
        id: row.try_get::<&str, reexport::uuid::Uuid>(column::ID).map_err(postgres_error)?.into(),
        title: row.try_get::<&str, String>(column::TITLE).map_err(postgres_error)?,
        members: row.try_get::<&str, Option<Vec<reexport::uuid::Uuid>>>(column::MEMBERS).map_err(postgres_error)?.unwrap_or_default().map(Uuid::from),
    })
}

pub fn to_summary(row: Row) -> Result<Summary> {
    Ok(Summary {
        id: row.try_get::<&str, reexport::uuid::Uuid>(column::ID).map_err(postgres_error)?.into(),
        title: row.try_get::<&str, String>(column::TITLE).map_err(postgres_error)?,
    })
}

pub fn to_inner(uuid: Uuid) -> reexport::uuid::Uuid {
    uuid.inner()
}

mod column {
    pub const ID: &str = "id";
    pub const PARTS: &str = "parts";
    pub const TITLE: &str = "title";
    pub const MEMBERS: &str = "members";
}
