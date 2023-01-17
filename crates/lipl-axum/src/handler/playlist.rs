use super::{to_error_response, to_json_response, to_status_ok, Key};
use axum::{extract::{State, Query}, http::StatusCode, Json, response::Response};
use futures_util::TryFutureExt;
use lipl_core::{LiplRepo, PlaylistPost};
use super::ListQuery;

/// Handler for getting all playlists
pub async fn list<T>(
    State(connection): State<T>,
    query: Query<ListQuery>,
) -> Response
where
    T: LiplRepo,
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
pub async fn item<T>(
    State(connection): State<T>,
    key: Key,
) -> Response
where
    T: LiplRepo,
{
    connection
        .get_playlist(key.id)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new playlist
pub async fn post<T>(
    State(connection): State<T>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Response
where
    T: LiplRepo,
{
    connection
        .upsert_playlist((None, playlist_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific playlist
pub async fn delete<T>(
    State(connection): State<T>,
    key: Key,
) -> Response
where
    T: LiplRepo,
{
    connection.delete_playlist(key.id)
        .map_ok_or_else(to_error_response, to_status_ok)
        .await
}

/// Handler for changing a specific playlist
pub async fn put<T>(
    State(connection): State<T>,
    key: Key,
    Json(playlist_post): Json<PlaylistPost>,
) -> Response
where
    T: LiplRepo,
{
    connection
        .upsert_playlist((Some(key.id), playlist_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}
