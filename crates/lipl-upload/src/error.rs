#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Https connection: {0}")]
    Https(&'static str),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Api: {0}")]
    Api(#[from] rest_api_client::Error),
}