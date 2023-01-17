use super::{to_json_response, to_status_ok, to_error_response, Key};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{Response},
};
use futures_util::TryFutureExt;
use lipl_core::{LiplRepo, LyricPost};
use super::ListQuery;

/// Handler for getting all lyrics
pub async fn list<T>(
    State(connection): State<T>,
    query: Query<ListQuery>,
) -> Response 
where
    T: LiplRepo,
{
    if query.full == Some(true) {
        connection
            .get_lyrics()
            .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
            .await
    }
    else {
        connection
            .get_lyric_summaries()
            .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
            .await
    }
}

/// Handler for getting a specific lyric
pub async fn item<T>(
    State(connection): State<T>,
    key: Key,
) -> Response 
where
    T: LiplRepo,
{
    connection
        .get_lyric(key.id)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for posting a new lyric
pub async fn post<T>(
    State(connection): State<T>,
    Json(lyric_post): Json<LyricPost>,
) -> Response
where
    T: LiplRepo,
{
    connection
        .upsert_lyric((None, lyric_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::CREATED))
        .await
}

/// Handler for deleting a specific lyric
pub async fn delete<T>(
    State(connection): State<T>,
    key: Key,
) -> Response
where
    T: LiplRepo,
{
        connection
            .delete_lyric(key.id)
            .map_ok_or_else(to_error_response, to_status_ok)
            .await
}

/// Handler for changing a specific lyric
pub async fn put<T>(
    State(connection): State<T>,
    key: Key,
    Json(lyric_post): Json<LyricPost>,
) -> Response
where
    T: LiplRepo,
{
    connection
        .upsert_lyric((Some(key.id), lyric_post).into())
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}
