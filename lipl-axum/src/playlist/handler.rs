use crate::{error::Error, to_json_response, to_status_ok, ConnectionPool};
use axum::{extract::Path, http::StatusCode, Json};
use futures_util::TryFutureExt;
use lipl_types::{Playlist, PlaylistPost, Summary};
use super::db;

/// Handler for getting all playlists
pub async fn list(pool: ConnectionPool) -> Result<(StatusCode, Json<Vec<Summary>>), Error> {
    pool.get()
        .map_err(Error::from)
        .and_then(db::list)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for getting a specific playlist
pub async fn item(
    pool: ConnectionPool,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<(StatusCode, Json<Playlist>), Error> {
    pool.get()
        .map_err(Error::from)
        .and_then(|connection| async move { db::item(connection, id).await })
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new playlist
pub async fn post(
    pool: ConnectionPool,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>), Error> {
    pool.get()
        .map_err(Error::from)
        .and_then(|connection| async move { db::post(connection, playlist_post).await })
        .map_ok(to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific playlist
pub async fn delete(pool: ConnectionPool, Path(id): Path<lipl_types::Uuid>) -> Result<StatusCode, Error> {
    pool.get()
        .map_err(Error::from)
        .and_then(|connection| async move { db::delete(connection, id.inner()).await })
        .map_ok(to_status_ok)
        .await
}

/// Handler for changing a specific playlist
pub async fn put(
    pool: ConnectionPool,
    Path(id): Path<lipl_types::Uuid>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>), Error> {
    pool.get()
        .map_err(Error::from)
        .and_then(|connection| async move { db::put(connection, id, playlist_post).await })
        .map_ok(to_json_response(StatusCode::OK))
        .await
}
