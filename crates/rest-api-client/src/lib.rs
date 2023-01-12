use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

mod error;
mod http_client;

pub use error::Error;
pub use http_client::ApiClient;
pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait ApiRequest {
    async fn get<T: DeserializeOwned>(&self, uri: &str) -> Result<T>;
    async fn post<T: Serialize + Send + Sync, U: DeserializeOwned>(&self, uri: &str, object: T) -> Result<U>;
    async fn delete(&self, uri: &str) -> Result<()>;
}
