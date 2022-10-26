use super::db;
use crate::{error, to_json_response, to_status_ok, DatabaseConnection};
use axum::{extract::Path, http::StatusCode, Json};
use futures_util::TryFutureExt;
use lipl_types::{Lyric, LyricPost, Summary};

/// Handler for getting all lyrics
pub async fn list(
    DatabaseConnection(connection): DatabaseConnection,
) -> Result<(StatusCode, Json<Vec<Summary>>), error::Error> {
    super::db::list(connection)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for getting a specific lyric
pub async fn item(
    DatabaseConnection(connection): DatabaseConnection,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    db::item(connection, id.inner())
        .map_ok(to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new lyric
pub async fn post(
    DatabaseConnection(connection): DatabaseConnection,
    Json(lyric_post): Json<LyricPost>,
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    db::post(connection, lyric_post)
        .map_ok(to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific lyric
pub async fn delete(
    DatabaseConnection(connection): DatabaseConnection,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<StatusCode, error::Error> {
    db::delete(connection, id.inner())
        .map_ok(to_status_ok)
        .await
}

/// Handler for changing a specific lyric
pub async fn put(
    DatabaseConnection(connection): DatabaseConnection,
    Path(id): Path<lipl_types::Uuid>,
    Json(lyric_post): Json<LyricPost>,
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    db::put(connection, id, lyric_post)
        .map_ok(to_json_response(StatusCode::OK))
        .await
}
