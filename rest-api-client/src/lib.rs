use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

mod error;
mod http_client;

pub use error::ApiError;
pub use http_client::ApiClient;
pub type ApiResult<T> = std::result::Result<T, ApiError>;

#[async_trait]
pub trait ApiRequest {
    async fn get<T: DeserializeOwned>(&self, uri: &str) -> ApiResult<T>;
    async fn post<T: Serialize + Send + Sync, U: DeserializeOwned>(&self, uri: &str, object: T) -> ApiResult<U>;
    async fn delete(&self, uri: &str) -> ApiResult<()>;
}
