use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Response, Json};
use futures_util::TryFutureExt;
use lipl_core::{LiplRepo, RepoDb};

use super::{to_error_response, to_json_response};

/// Handler for getting the database
pub async fn get(State(connection): State<Arc<dyn LiplRepo>>) -> Response {
    connection
        .get_db()
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}

/// Handler for replacing the database
pub async fn put(State(connection): State<Arc<dyn LiplRepo>>, Json(db): Json<RepoDb>) -> Response {
    connection
        .replace_db(db)
        .map_ok_or_else(to_error_response, to_json_response(StatusCode::OK))
        .await
}
