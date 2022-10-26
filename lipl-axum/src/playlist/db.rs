use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use futures_util::TryFutureExt;
use lipl_types::{Playlist, PlaylistPost, Summary, Uuid};
use tokio_postgres::NoTls;

use super::sql;
use crate::error::Error;
use super::convert;

pub async fn list(
    connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
) -> Result<Vec<Summary>, Error> {
    connection
        .query(sql::LIST, &[])
        .map_err(Error::from)
        .map_ok(convert::to_list(convert::to_summary))
        .await
}

pub async fn item(
    connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    id: Uuid,
) -> Result<Playlist, Error> {
    let title = connection
        .query_one(sql::ITEM_TITLE, &[&id.inner()])
        .map_err(Error::from)
        .await?
        .get::<&str, String>(sql::column::TITLE);

    let members = connection
        .query(sql::ITEM_MEMBERS, &[&id.inner()])
        .map_err(Error::from)
        .await?
        .into_iter()
        .map(|row| Uuid::from(row.get::<&str, uuid::Uuid>(sql::column::LYRIC_ID)))
        .collect::<Vec<_>>();

    Ok(Playlist { id, title, members })
}

pub async fn delete(
    connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    id: uuid::Uuid,
) -> Result<u64, Error> {
    connection
        .execute(sql::DELETE, &[&id])
        .map_err(Error::from)
        .await
}

pub async fn post(
    connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    playlist_post: PlaylistPost,
) -> Result<Playlist, Error> {
    let id = Uuid::default();
    let members = playlist_post
        .members
        .into_iter()
        .map(|uuid| uuid.inner())
        .collect::<Vec<_>>();

    connection
        .query_one(
            sql::UPSERT,
            &[&id.inner(), &playlist_post.title, &members.as_slice()],
        )
        .map_err(Error::from)
        // .inspect_ok(|row| { println!("Row: {:#?}", row.get::<&str, Option<Vec<uuid::Uuid>>>("fn_upsert_playlist")); })
        .await?;

    item(connection, id).await
}

pub async fn put(
    connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    id: lipl_types::Uuid,
    playlist_post: PlaylistPost,
) -> Result<Playlist, Error> {
    let members = playlist_post
        .members
        .into_iter()
        .map(|uuid| uuid.inner())
        .collect::<Vec<_>>();

    connection
        .query_one(
            sql::UPSERT,
            &[&id.inner(), &playlist_post.title, &members.as_slice()],
        )
        .map_err(Error::from)
        // .map_ok(|row| row.get::<usize, Vec<uuid::Uuid>>(0))
        .await?;

    item(connection, id).await
}
