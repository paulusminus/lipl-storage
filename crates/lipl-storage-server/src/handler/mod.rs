use crate::error::ErrorReport;
use axum::{
    extract::FromRequestParts,
    response::{IntoResponse, Json, Response},
};
use hyper::StatusCode;
use lipl_core::Repo;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod db;
pub mod lyric;
pub mod playlist;

#[derive(Deserialize)]
pub struct ListQuery {
    full: Option<bool>,
}

pub struct Key {
    pub id: lipl_core::Uuid,
}

impl Key {
    pub fn new(id: lipl_core::Uuid) -> Self {
        Self { id }
    }
}

impl<R: Repo + Sync> FromRequestParts<Arc<R>> for Key {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &Arc<R>,
    ) -> Result<Self, Self::Rejection> {
        parts
            .uri
            .path()
            .split('/')
            .next_back()
            .ok_or(StatusCode::NOT_FOUND)
            .and_then(|s| {
                s.parse::<lipl_core::Uuid>()
                    .map_err(|_| StatusCode::NOT_FOUND)
            })
            .map(Key::new)
    }
}

pub(crate) fn to_json_response<T>(status_code: StatusCode) -> impl Fn(T) -> Response
where
    T: Serialize,
{
    move |t| (status_code, Json(t)).into_response()
}

fn not_found_or_internal_server(error: &lipl_core::Error) -> StatusCode {
    match error {
        lipl_core::Error::NoKey(_) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
pub(crate) fn to_error_response(error: lipl_core::Error) -> Response {
    (
        not_found_or_internal_server(&error),
        Json(ErrorReport::from(error)),
    )
        .into_response()
}

pub(crate) fn to_status_ok<T>(_: T) -> Response {
    StatusCode::OK.into_response()
}
