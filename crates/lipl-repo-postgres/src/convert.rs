use lipl_core::{Uuid, Lyric, Playlist, Summary};
use parts::to_parts;
use bb8_postgres::tokio_postgres::Row;

use crate::{postgres_error, Result};

pub fn get_id(row: &Row) -> Result<Uuid> {
    row.try_get::<&str, uuid::Uuid>("id")
    .map_err(postgres_error)
    .map(Uuid::from)
}

#[allow(clippy::map_identity)]
pub fn get_title(row: &Row) -> Result<String> {
    row.try_get::<&str, String>("title")
    .map_err(postgres_error)
    .map(std::convert::identity)
}

pub fn get_parts(row: &Row) -> Result<Vec<Vec<String>>> {
    row.try_get::<&str, String>("parts")
    .map_err(postgres_error)
    .map(to_parts)
}

pub fn get_members(row: &Row) -> Result<Vec<Uuid>> {
    row.try_get::<&str, Vec<uuid::Uuid>>("members")
    .map_err(postgres_error)
    .map(convert_vec(Uuid::from))
}

pub fn convert_vec<F, T, U>(f: F) -> impl Fn(Vec<T>) -> Vec<U>
where F: Fn(T) -> U + Copy
{
    move |v| v.into_iter().map(f).collect()
}

pub fn try_convert_vec<F, T, U>(f: F) -> impl Fn(Vec<T>) -> Result<Vec<U>>
where F: Fn(T) -> Result<U> + Copy
{
    move |v| v.into_iter().map(f).collect()
}

pub fn to_lyric(row: Row) -> Result<Lyric> {
    Ok(
        Lyric {
            id: get_id(&row)?,
            title: get_title(&row)?,
            parts: get_parts(&row)?,
        }
    )    
}

pub fn to_playlist(row: Row) -> Result<Playlist> {
    Ok(
        Playlist {
            id: get_id(&row)?,
            title: get_title(&row)?,
            members: get_members(&row)?,
        }
    )
}

pub fn to_summary(row: Row) -> Result<Summary> {
    Ok(
        Summary {
            id: get_id(&row)?,
            title: get_title(&row)?,
        }
    )
}

pub fn to_ok<T>(t: T) -> Result<T> {
    Ok(t)
}

