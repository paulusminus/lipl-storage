use std::sync::Arc;

use axum::{
    extract::FromRequestParts,
    response::{IntoResponse, Json, Response},
};
use hyper::StatusCode;
use lipl_core::LiplRepo;
use serde::{Deserialize, Serialize};

use crate::error::ErrorReport;

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

impl FromRequestParts<Arc<dyn LiplRepo>> for Key {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &Arc<dyn LiplRepo>,
    ) -> Result<Self, Self::Rejection> {
        parts
            .uri
            .path()
            .split('/')
            .last()
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

pub(crate) fn to_error_response(error: lipl_core::Error) -> Response {
    match error {
        lipl_core::Error::NoKey(_) => {
            (StatusCode::NOT_FOUND, Json(ErrorReport::from(error))).into_response()
        }
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorReport::from(error)),
        )
            .into_response(),
    }
}

pub(crate) fn to_status_ok<T>(_: T) -> Response {
    StatusCode::OK.into_response()
}
