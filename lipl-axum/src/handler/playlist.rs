use std::sync::Arc;

use super::{to_error_response, to_json_response, to_status_ok};
use axum::{extract::{Path, State}, http::StatusCode, Json, response::Response};
use futures_util::TryFutureExt;
use lipl_axum_postgres::{PostgresConnectionPool};
use lipl_core::{PlaylistDb, PlaylistPost};

/// Handler for getting all playlists
pub async fn list(
    State(connection): State<Arc<PostgresConnectionPool>>
) -> Response {
    connection
        .playlist_list()
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for getting a specific playlist
pub async fn item(
    State(connection): State<Arc<PostgresConnectionPool>>,
    Path(id): Path<lipl_core::Uuid>,
) -> Response {
    connection
        .playlist_item(id)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new playlist
pub async fn post(
    State(connection): State<Arc<PostgresConnectionPool>>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Response {
    connection
        .playlist_post(playlist_post)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific playlist
pub async fn delete(
    State(connection): State<Arc<PostgresConnectionPool>>,
    Path(id): Path<lipl_core::Uuid>,
) -> Response {
    connection.playlist_delete(id)
        .map_ok_or_else(to_error_response, to_status_ok)
        .await
}

/// Handler for changing a specific playlist
pub async fn put(
    State(connection): State<Arc<PostgresConnectionPool>>,
    Path(id): Path<lipl_core::Uuid>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Response {
    connection
        .playlist_put(id, playlist_post)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}
