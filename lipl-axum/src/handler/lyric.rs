use std::sync::Arc;

use super::{to_json_response, to_status_ok, to_error_response};
use axum::{extract::{Path, State}, http::StatusCode, Json, response::{Response}};
use futures_util::TryFutureExt;
use lipl_axum_postgres::{PostgresConnection};
use lipl_core::{LyricDb, LyricPost};

/// Handler for getting all lyrics
pub async fn list(
    State(connection): State<Arc<PostgresConnection>>
) -> Response {
    connection
        .lyric_list()
        .map_ok(to_json_response(StatusCode::OK))
        .await
        .unwrap_or_else(to_error_response)
}

/// Handler for getting a specific lyric
pub async fn item(
    State(connection): State<Arc<PostgresConnection>>,
    Path(id): Path<lipl_core::Uuid>,
) -> Response {
    connection
        .lyric_item(id)
        .map_ok(to_json_response(StatusCode::OK))
        .await
        .unwrap_or_else(to_error_response)
}

/// Handler for posting a new lyric
pub async fn post(
    State(connection): State<Arc<PostgresConnection>>,
    Json(lyric_post): Json<LyricPost>,
) -> Response {
    connection
        .lyric_post(lyric_post)
        .map_ok(to_json_response(StatusCode::CREATED))
        .await
        .unwrap_or_else(to_error_response)
}

/// Handler for deleting a specific lyric
pub async fn delete(
    State(connection): State<Arc<PostgresConnection>>,
    Path(id): Path<lipl_core::Uuid>,
) -> Response {
    connection.lyric_delete(id)
        .map_ok(to_status_ok)
        .await
        .unwrap_or_else(to_error_response)
}

/// Handler for changing a specific lyric
pub async fn put(
    State(connection): State<Arc<PostgresConnection>>,
    Path(id): Path<lipl_core::Uuid>,
    Json(lyric_post): Json<LyricPost>,
) -> Response {
    connection
        .lyric_put(id, lyric_post)
        .map_ok(to_json_response(StatusCode::OK))
        .await
        .unwrap_or_else(to_error_response)
}
