use std::io::{BufRead, Read};
use async_trait::async_trait;
use flate2::bufread::GzDecoder;
use hyper::{client::{Client, HttpConnector}, Response, Body, body::{aggregate, Buf}, Method, Uri, header::{CONTENT_ENCODING, ACCEPT, ACCEPT_ENCODING, USER_AGENT}, Request};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::{de::DeserializeOwned, Serialize};
use crate::{ApiResult, ApiRequest};

pub struct ApiClient {
    client: Client<HttpsConnector<HttpConnector>>,
    prefix: String,
}

impl ApiClient {
    pub fn new(prefix: String) -> Self {
        let https = HttpsConnectorBuilder::new().with_webpki_roots().https_or_http().enable_http1().build();
        let client = Client::builder().build(https);
        Self {
            client,
            prefix,
        }
    }

    async fn send(&self, request: Request<Body>) -> ApiResult<Box<dyn Read>> {
        let response: Response<Body> = self.client.clone().request(request).await?;
        let is_zip = is_zip_encoded(&response);
        let reader_to_be_wrapped = aggregate(response).await?.reader();
        Ok(
            wrap_reader(reader_to_be_wrapped, is_zip)
        )
    }

}

#[async_trait]
impl ApiRequest for ApiClient {
    async fn get<T: DeserializeOwned>(&self, uri: &str) -> ApiResult<T> {
        let uri = format!("{}{}", self.prefix, uri).parse::<Uri>()?;
        let request = new_api_request(uri, Method::GET, Body::empty())?;
        let response_result = self.send(request).await?;
        let t: T = serde_json::from_reader(response_result)?;
        Ok(t)
    }

    async fn insert<T: Serialize + Send, U: DeserializeOwned>(&self, uri: &str, object: T) -> ApiResult<U> {
        let uri = format!("{}{}", self.prefix, uri).parse::<Uri>()?;
        let body = serde_json::ser::to_string(&object)?;
        let request = new_api_request(uri, Method::POST, body.into())?;
        let response_result = self.send(request).await?;
        let u: U = serde_json::from_reader(response_result)?;
        Ok(u)
    }

    async fn delete(&self, uri: &str) -> ApiResult<()> {
        let uri = format!("{}{}", self.prefix, uri).parse::<Uri>()?;
        let request = new_api_request(uri, Method::DELETE, Body::empty())?;
        let _response_result = self.send(request).await?;
        Ok(())
    }
}

fn new_api_request(uri: Uri, method: Method, body: Body) -> ApiResult<Request<Body>> {
    Request::builder()
    .uri(uri)
    .method(method)
    .header(ACCEPT, "application/json")
    .header(ACCEPT_ENCODING, "gzip")
    .header(USER_AGENT, "Rust hyper")
    .body(body)
    .map_err(crate::error::ApiError::from)
}

fn is_zip_encoded(response: &Response<Body>) -> bool {
    response.headers().get_all(CONTENT_ENCODING).iter().any(|e| e == "gzip")
}

fn wrap_reader<'a>(reader: impl BufRead + Send + 'a, is_zip: bool) -> Box<dyn Read + 'a> {
    if is_zip {
        Box::new(GzDecoder::new(reader))
    }
    else {
        Box::new(reader)
    }
}
