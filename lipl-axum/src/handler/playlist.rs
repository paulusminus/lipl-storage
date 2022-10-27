use super::{to_json_response, to_status_ok};
use lipl_axum_postgres::{PostgresConnection, Result};
use axum::{extract::Path, http::StatusCode, Json};
use futures_util::TryFutureExt;
use lipl_types::{Playlist, PlaylistDb, PlaylistPost, Summary};

/// Handler for getting all playlists
pub async fn list(
    connection: PostgresConnection<'_>,
) -> Result<(StatusCode, Json<Vec<Summary>>)> {
    connection
        .playlist_list()
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for getting a specific playlist
pub async fn item(
    connection: PostgresConnection<'_>,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<(StatusCode, Json<Playlist>)> {
    connection
        .playlist_item(id)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new playlist
pub async fn post(
    connection: PostgresConnection<'_>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>)> {
    connection
        .playlist_post(playlist_post)
        .map_ok(to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific playlist
pub async fn delete(
    connection: PostgresConnection<'_>,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<StatusCode> {
    connection.playlist_delete(id).map_ok(to_status_ok).await
}

/// Handler for changing a specific playlist
pub async fn put(
    connection: PostgresConnection<'_>,
    Path(id): Path<lipl_types::Uuid>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>)> {
    connection
        .playlist_put(id, playlist_post)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}
