use axum::response::{IntoResponse, Json, Response};
use hyper::StatusCode;
use serde::Serialize;

use crate::{error::ErrorReport, Error};

pub mod lyric;
pub mod playlist;

pub(crate) fn to_json_response<T>(status_code: StatusCode) -> impl Fn(T) -> Response
where T: Serialize
{
    move |t| (status_code, Json(t)).into_response()
}

pub(crate) fn to_error_response(error: lipl_axum_postgres::Error) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorReport::from(Error::from(error)))).into_response()
}

pub(crate) fn to_status_ok<T>(_: T) -> Response {
    StatusCode::OK.into_response()
}
