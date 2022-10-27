use super::{to_json_response, to_status_ok};
use lipl_axum_postgres::{PostgresConnection, Result};
use axum::{extract::Path, http::StatusCode, Json};
use futures_util::TryFutureExt;
use lipl_types::{Lyric, LyricDb, LyricPost, Summary};

/// Handler for getting all lyrics
pub async fn list(
    connection: PostgresConnection<'_>,
) -> Result<(StatusCode, Json<Vec<Summary>>)> {
    connection
        .lyric_list()
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for getting a specific lyric
pub async fn item(
    connection: PostgresConnection<'_>,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<(StatusCode, Json<Lyric>)> {
    connection
        .lyric_item(id)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new lyric
pub async fn post(
    connection: PostgresConnection<'_>,
    Json(lyric_post): Json<LyricPost>,
) -> Result<(StatusCode, Json<Lyric>)> {
    connection
        .lyric_post(lyric_post)
        .map_ok(to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific lyric
pub async fn delete(
    connection: PostgresConnection<'_>,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<StatusCode> {
    connection.lyric_delete(id).map_ok(to_status_ok).await
}

/// Handler for changing a specific lyric
pub async fn put(
    connection: PostgresConnection<'_>,
    Path(id): Path<lipl_types::Uuid>,
    Json(lyric_post): Json<LyricPost>,
) -> Result<(StatusCode, Json<Lyric>)> {
    connection
        .lyric_put(id, lyric_post)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}
