use std::convert::Infallible;
use std::error::Error;
use tracing::error;

use serde::Serialize;
use warp::{Rejection, hyper::StatusCode, Reply};

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: &'static str,
}

impl ErrorMessage {
    fn new(code: StatusCode, message: &'static str) -> ErrorMessage {
        ErrorMessage { code: code.as_u16(), message }
    }
}

pub fn json_response(code: StatusCode, message: &'static str) -> Result<impl Reply, Infallible> {
    let json = warp::reply::json(&ErrorMessage::new(code, message));
    Ok(
        
        warp::reply::with_status(json, code)
    )
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        return json_response(StatusCode::NOT_FOUND, "NOT_FOUND");
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
        return json_response(StatusCode::BAD_REQUEST, message);
    }
    else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        return json_response(StatusCode::METHOD_NOT_ALLOWED, "METHOD_NOT_ALLOWED");
    } 
    else {
        error!("unhandled rejection: {:?}", err);
        return json_response(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_REJECTION");
    }
}