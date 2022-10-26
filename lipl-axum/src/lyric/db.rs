use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use futures_util::TryFutureExt;
use lipl_types::{Lyric, LyricPost, Summary, Uuid};
use parts::to_text;
use tokio_postgres::NoTls;

use crate::error::Error;
use super::convert;
use super::sql;

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
    id: uuid::Uuid,
) -> Result<Lyric, Error> {
    connection
        .query_one(sql::ITEM, &[&id])
        .map_err(Error::from)
        .map_ok(convert::to_lyric)
        .await
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
    lyric_post: LyricPost,
) -> Result<Lyric, Error> {
    let id = lipl_types::Uuid::default();

    connection
        .execute(
            sql::INSERT,
            &[&id.inner(), &lyric_post.title, &to_text(&lyric_post.parts)],
        )
        .map_err(Error::from)
        .await?;

    item(connection, id.inner()).await
}

pub async fn put(
    connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    id: Uuid,
    lyric_post: LyricPost,
) -> Result<Lyric, Error> {
    connection
        .execute(
            sql::UPDATE,
            &[&lyric_post.title, &to_text(&lyric_post.parts), &id.inner()],
        )
        .map_err(Error::from)
        .await?;

    item(connection, id.inner()).await
}
