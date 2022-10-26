use crate::{error, ConnectionPool};
use axum::{extract::Path, http::StatusCode, Json};
use futures_util::TryFutureExt;
use lipl_types::{Lyric, LyricPost, Summary};
use super::db;

/// Handler for getting all lyrics
pub async fn list(pool: ConnectionPool) -> Result<(StatusCode, Json<Vec<Summary>>), error::Error> {
    pool.get()
        .map_err(error::Error::from)
        .and_then(super::db::list)
        .map_ok(crate::to_json_response(StatusCode::OK))
        .await
}

/// Handler for getting a specific lyric
pub async fn item(
    pool: ConnectionPool,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    pool.get()
        .map_err(error::Error::from)
        .and_then(|connection| async move { db::item(connection, id.inner()).await })
        .map_ok(crate::to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new lyric
pub async fn post(
    pool: ConnectionPool,
    Json(lyric_post): Json<LyricPost>,
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    pool.get()
        .map_err(error::Error::from)
        .and_then(|connection| async move { db::post(connection, lyric_post).await })
        .map_ok(crate::to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific lyric
pub async fn delete(
    pool: ConnectionPool,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<StatusCode, error::Error> {
    pool.get()
        .map_err(error::Error::from)
        .and_then(|connection| async move { db::delete(connection, id.inner()).await })
        .map_ok(crate::to_status_ok)
        .await
}

/// Handler for changing a specific lyric
pub async fn put(
    pool: ConnectionPool,
    Path(id): Path<lipl_types::Uuid>,
    Json(lyric_post): Json<LyricPost>,
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    pool.get()
        .map_err(error::Error::from)
        .and_then(|connection| async move { db::put(connection, id, lyric_post).await })
        .map_ok(crate::to_json_response(StatusCode::OK))
        .await
}
