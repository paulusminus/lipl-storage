use axum::response::{IntoResponse, Json, Response};
use hyper::StatusCode;
use serde::Serialize;

pub mod lyric;
pub mod playlist;

#[derive(Serialize)]
struct ErrorReport {
    error: String
}

impl ErrorReport {
    fn new(error: lipl_axum_postgres::Error) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

pub(crate) fn to_json_response<T: Serialize>(status_code: StatusCode) -> impl Fn(T) -> Response {
    move |t| (status_code, Json(t)).into_response()
}

pub(crate) fn to_error_response(error: lipl_axum_postgres::Error) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorReport::new(error))).into_response()
}

pub(crate) fn to_status_ok<T>(_: T) -> Response {
    StatusCode::OK.into_response()
}
