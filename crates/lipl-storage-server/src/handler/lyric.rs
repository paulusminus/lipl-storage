use std::sync::Arc;

use super::ListQuery;
use super::{to_error_response, to_json_response, to_status_ok, Key};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
    Json,
};
use futures_util::TryFutureExt;
use lipl_core::{LiplRepo, LyricPost};

/// Handler for getting all lyrics
pub async fn list(
    State(connection): State<Arc<dyn LiplRepo>>,
    query: Query<ListQuery>,
) -> Response {
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
pub async fn item(State(connection): State<Arc<dyn LiplRepo>>, key: Key) -> Response {
    connection
        .get_lyric(key.id)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new lyric
pub async fn post(
    State(connection): State<Arc<dyn LiplRepo>>,
    Json(lyric_post): Json<LyricPost>,
) -> Response {
    connection
        .upsert_lyric((None, lyric_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific lyric
pub async fn delete(State(connection): State<Arc<dyn LiplRepo>>, key: Key) -> Response {
    connection
        .delete_lyric(key.id)
        .map_ok_or_else(to_error_response, to_status_ok)
        .await
}

/// Handler for changing a specific lyric
pub async fn put(
    State(connection): State<Arc<dyn LiplRepo>>,
    key: Key,
    Json(lyric_post): Json<LyricPost>,
) -> Response {
    connection
        .upsert_lyric((Some(key.id), lyric_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}
