use axum::{response::{IntoResponse, Json, Response}, extract::FromRequestParts};
use futures_util::FutureExt;
use hyper::StatusCode;
use lipl_core::LiplRepo;
use serde::{Deserialize, Serialize};

use crate::{error::ErrorReport};

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
        Self {
            id,
        }
    }
}

impl<T> FromRequestParts<T> for Key 
where
    T: LiplRepo + Clone + Send + Sync + 'static,
{
    type Rejection = StatusCode;

    fn from_request_parts<'life0,'life1,'async_trait>(parts: &'life0 mut axum::http::request::Parts, _state: &'life1 T) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Self, Self::Rejection> > + core::marker::Send+'async_trait>> where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        async move {
            tracing::info!("Path: {}", parts.uri.path());
            parts.uri.path().split('/').last().ok_or(StatusCode::NOT_FOUND)
                .and_then(|s| s.parse::<lipl_core::Uuid>().map_err(|_| StatusCode::NOT_FOUND))
                .map(Key::new)
        }
        .boxed()
    }
}

pub(crate) fn to_json_response<T>(status_code: StatusCode) -> impl Fn(T) -> Response
where T: Serialize
{
    move |t| (status_code, Json(t)).into_response()
}

pub(crate) fn to_error_response(error: lipl_core::Error) -> Response {
    match error {
        lipl_core::Error::NoKey(_) => (StatusCode::NOT_FOUND, Json(ErrorReport::from(error))).into_response(),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorReport::from(error))).into_response()
    }
    
}

pub(crate) fn to_status_ok<T>(_: T) -> Response {
    StatusCode::OK.into_response()
}

// pub(crate) fn not_found() -> Response {
//     StatusCode::NOT_FOUND.into_response()
// }
