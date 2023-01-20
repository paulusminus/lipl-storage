use std::convert::Infallible;
use std::error::Error;
use tracing::error;

use serde::Serialize;
use warp::{Rejection, hyper::StatusCode, Reply};
use crate::error::RepoError;

#[derive(Serialize)]
struct ErrorMessage<'a> {
    code: u16,
    message: &'a str,
}

impl<'a> ErrorMessage<'a> {
    fn new(code: StatusCode, message: &'a str) -> ErrorMessage {
        ErrorMessage { code: code.as_u16(), message }
    }
}

pub fn json_response(code: StatusCode, message: &str) -> Result<impl Reply, Infallible> {
    let json = warp::reply::json(&ErrorMessage::new(code, message));
    Ok(    
        warp::reply::with_status(json, code)
    )
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(e) = err.find::<RepoError>() {
        match e {
            RepoError::Model(m) => {
                json_response(StatusCode::NOT_FOUND, &m.to_string())
            },
            #[cfg(feature = "file")]
            RepoError::File(f) => {
                json_response(StatusCode::INTERNAL_SERVER_ERROR, &f.to_string())
            },
            #[cfg(feature = "postgres")]
            RepoError::Postgres(p) => {
                json_response(StatusCode::INTERNAL_SERVER_ERROR, &p.to_string())
            },
            RepoError::Backend(b) => {
                json_response(StatusCode::INTERNAL_SERVER_ERROR, &b.to_string())
            }
        }
    }
    else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        let message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        json_response(StatusCode::BAD_REQUEST, message)
    }
    else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        json_response(StatusCode::METHOD_NOT_ALLOWED, "METHOD_NOT_ALLOWED")
    } 
    else {
        error!("unhandled rejection: {:?}", err);
        json_response(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_REJECTION")
    }
}