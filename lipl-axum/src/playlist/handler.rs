use crate::{error::Error, to_json_response, to_status_ok, DatabaseConnection};
use axum::{extract::Path, http::StatusCode, Json};
use futures_util::TryFutureExt;
use lipl_types::{Playlist, PlaylistPost, Summary};
use super::db;

/// Handler for getting all playlists
pub async fn list(DatabaseConnection(connection): DatabaseConnection) -> Result<(StatusCode, Json<Vec<Summary>>), Error> {
    db::list(connection)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for getting a specific playlist
pub async fn item(
    DatabaseConnection(connection): DatabaseConnection,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<(StatusCode, Json<Playlist>), Error> {
    db::item(connection, id)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new playlist
pub async fn post(
    DatabaseConnection(connection): DatabaseConnection,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>), Error> {
    db::post(connection, playlist_post)
        .map_ok(to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific playlist
pub async fn delete(
    DatabaseConnection(connection): DatabaseConnection,
    Path(id): Path<lipl_types::Uuid>
) -> Result<StatusCode, Error> {
    db::delete(connection, id.inner())
        .map_ok(to_status_ok)
        .await
}

/// Handler for changing a specific playlist
pub async fn put(
    DatabaseConnection(connection): DatabaseConnection,
    Path(id): Path<lipl_types::Uuid>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>), Error> {
    db::put(connection, id, playlist_post)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}
