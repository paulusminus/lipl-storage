use std::sync::Arc;

use super::ListQuery;
use super::{Key, to_error_response, to_json_response, to_status_ok};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use futures_util::TryFutureExt;
use lipl_core::{LyricPost, Repo};

/// Handler for getting all lyrics
pub async fn list<R: Repo>(State(connection): State<Arc<R>>, query: Query<ListQuery>) -> Response {
    if query.full == Some(true) {
        connection
            .get_lyrics()
            .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
            .await
    } else {
        connection
            .get_lyric_summaries()
            .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
            .await
    }
}

/// Handler for getting a specific lyric
pub async fn item<R: Repo>(State(connection): State<Arc<R>>, key: Key) -> Response {
    connection
        .get_lyric(key.id)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new lyric
pub async fn post<R: Repo>(
    State(connection): State<Arc<R>>,
    Json(lyric_post): Json<LyricPost>,
) -> Response {
    connection
        .upsert_lyric((None, lyric_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific lyric
pub async fn delete<R: Repo>(State(connection): State<Arc<R>>, key: Key) -> Response {
    connection
        .delete_lyric(key.id)
        .map_ok_or_else(to_error_response, to_status_ok)
        .await
}

/// Handler for changing a specific lyric
pub async fn put<R: Repo>(
    State(connection): State<Arc<R>>,
    key: Key,
    Json(lyric_post): Json<LyricPost>,
) -> Response {
    connection
        .upsert_lyric((Some(key.id), lyric_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}
