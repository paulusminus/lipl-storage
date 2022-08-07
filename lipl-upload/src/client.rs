use flate2::bufread::GzDecoder;
use hyper::{Body, Client, Request, Response, Uri};
use hyper::body::{aggregate, Buf};
use hyper::client::{HttpConnector};
use hyper::header::{ACCEPT, ACCEPT_ENCODING, CONTENT_ENCODING, USER_AGENT};
use hyper::http::method::Method;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use crate::UploadResult;

pub struct UploadClient {
    client: Client<HttpsConnector<HttpConnector>>,
    prefix: String,
}

pub struct ResponseResult {
    reader: Box<dyn std::io::Read>
}


fn create_request(uri: Uri, method: Method, body: Body) -> UploadResult<Request<Body>> {
    Request::builder()
    .uri(uri)
    .method(method)
    .header(ACCEPT, "application/json")
    .header(ACCEPT_ENCODING, "gzip")
    .header(USER_AGENT, "Rust hyper")
    .body(body)
    .map_err(crate::error::UploadError::from)
}


impl UploadClient {
    pub fn new(prefix: String) -> Self {
        let https = HttpsConnectorBuilder::new().with_webpki_roots().https_or_http().enable_http1().build();
        let client = Client::builder().build(https);
        UploadClient {
            client,
            prefix,
        }
    }

    pub async fn get_object<T: DeserializeOwned>(&self, uri: &str) -> UploadResult<T> {
        let uri = format!("{}{}", self.prefix, uri).parse::<Uri>()?;
        let request = create_request(uri, Method::GET, Body::empty())?;
        let response_result = self.send(request).await?;
        let t: T = serde_json::from_reader(response_result.reader)?;
        Ok(t)
    }

    pub async fn insert_object<T: Serialize, U: DeserializeOwned>(&self, uri: &str, object: T) -> UploadResult<U> {
        let uri = format!("{}{}", self.prefix, uri).parse::<Uri>()?;
        let body = serde_json::ser::to_string(&object)?;
        let request = create_request(uri, Method::POST, Body::from(body))?;
        let response_result = self.send(request).await?;
        let u: U = serde_json::from_reader(response_result.reader)?;
        Ok(u)
    }

    pub async fn delete_object(&self, uri: &str) -> UploadResult<()> {
        let uri = format!("{}{}", self.prefix, uri).parse::<Uri>()?;
        let request = create_request(uri, Method::DELETE, Body::empty())?;
        let _response_result = self.send(request).await?;
        Ok(())
    }

    async fn send(&self, request: Request<Body>) -> UploadResult<ResponseResult> {
        let response: Response<Body> = self.client.clone().request(request).await?;
        let is_zip = is_zip_encoded(&response);
        let reader_to_be_wrapped = aggregate(response).await?.reader();
        Ok(
            ResponseResult {
                reader: wrap_reader(reader_to_be_wrapped, is_zip)
            }
        )
    }
}

fn is_zip_encoded(response: &Response<Body>) -> bool {
    response.headers().get_all(CONTENT_ENCODING).iter().any(|e| e == "gzip")
}

fn wrap_reader<'a>(reader: impl std::io::BufRead + Send + 'a, is_zip: bool) -> Box<dyn std::io::Read + 'a> {
    if is_zip {
        Box::new(GzDecoder::new(reader))
    }
    else {
        Box::new(reader)
    }
}
