use super::{to_json_response, to_status_ok, to_error_response};
use axum::{extract::{Path, State}, http::StatusCode, Json, response::{Response}};
use futures_util::TryFutureExt;
use lipl_core::{LyricDb, LyricPost};

/// Handler for getting all lyrics
pub async fn list<T>(
    State(connection): State<T>,
) -> Response 
where
    T: LyricDb,
{
    connection
        .lyric_list()
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for getting a specific lyric
pub async fn item<T>(
    State(connection): State<T>,
    Path(id): Path<lipl_core::Uuid>,
) -> Response 
where
    T: LyricDb,
{
    connection
        .lyric_item(id)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new lyric
pub async fn post<T>(
    State(connection): State<T>,
    Json(lyric_post): Json<LyricPost>,
) -> Response
where
    T: LyricDb,
{
    connection
        .lyric_post(lyric_post)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific lyric
pub async fn delete<T>(
    State(connection): State<T>,
    Path(id): Path<lipl_core::Uuid>,
) -> Response
where
    T: LyricDb,
{
    connection
        .lyric_delete(id)
        .map_ok_or_else(to_error_response, to_status_ok)
        .await
}

/// Handler for changing a specific lyric
pub async fn put<T>(
    State(connection): State<T>,
    Path(id): Path<lipl_core::Uuid>,
    Json(lyric_post): Json<LyricPost>,
) -> Response
where
    T: LyricDb,
{
    connection
        .lyric_put(id, lyric_post)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}
