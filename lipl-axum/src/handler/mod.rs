use axum::Json;
use hyper::StatusCode;

pub mod lyric;
pub mod playlist;

pub(crate) fn to_json_response<T>(status_code: StatusCode) -> impl Fn(T) -> (StatusCode, Json<T>) {
    move |t| (status_code, Json(t))
}

pub(crate) fn to_status_ok<T>(_: T) -> StatusCode {
    StatusCode::OK
}
