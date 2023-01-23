use std::sync::Arc;

use super::{to_error_response, to_json_response, to_status_ok, Key};
use axum::{extract::{State, Query}, http::StatusCode, Json, response::Response};
use futures_util::TryFutureExt;
use lipl_core::{LiplRepo, PlaylistPost};
use super::ListQuery;

/// Handler for getting all playlists
pub async fn list(
    State(connection): State<Arc<dyn LiplRepo>>,
    query: Query<ListQuery>,
) -> Response
{
    if query.full == Some(true) {
        connection
        .get_playlists()
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
    }
    else {
        connection
        .get_playlist_summaries()
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
    }
}

/// Handler for getting a specific playlist
pub async fn item(
    State(connection): State<Arc<dyn LiplRepo>>,
    key: Key,
) -> Response
{
    connection
        .get_playlist(key.id)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new playlist
pub async fn post(
    State(connection): State<Arc<dyn LiplRepo>>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Response
{
    connection
        .upsert_playlist((None, playlist_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific playlist
pub async fn delete(
    State(connection): State<Arc<dyn LiplRepo>>,
    key: Key,
) -> Response
{
    connection.delete_playlist(key.id)
        .map_ok_or_else(to_error_response, to_status_ok)
        .await
}

/// Handler for changing a specific playlist
pub async fn put(
    State(connection): State<Arc<dyn LiplRepo>>,
    key: Key,
    Json(playlist_post): Json<PlaylistPost>,
) -> Response
{
    connection
        .upsert_playlist((Some(key.id), playlist_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}
