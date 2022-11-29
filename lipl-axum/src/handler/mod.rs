use axum::response::{IntoResponse, Json, Response};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{error::ErrorReport};

pub mod lyric;
pub mod playlist;

#[derive(Deserialize)]
pub struct ListQuery {
    full: Option<bool>,
}

pub(crate) fn to_json_response<T>(status_code: StatusCode) -> impl Fn(T) -> Response
where T: Serialize
{
    move |t| (status_code, Json(t)).into_response()
}

pub(crate) fn to_error_response<E: std::error::Error>(error: E) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorReport::from(error))).into_response()
}

pub(crate) fn to_status_ok<T>(_: T) -> Response {
    StatusCode::OK.into_response()
}
