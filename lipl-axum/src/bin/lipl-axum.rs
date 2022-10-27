use std::net::SocketAddr;

use lipl_axum::{constant::{self}, create_service, exit_on_signal_int, Error};
use futures_util::TryFutureExt;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let filter =
        std::env::var(constant::RUST_LOG)
        .unwrap_or_else(|_| constant::DEFAULT_LOG_FILTER.to_owned());
    
    tracing_subscriber::fmt()
    .with_env_filter(filter)
    .init();

    create_service()
    .map_err(Error::from)
    .and_then(|service| async move {
        let addr = SocketAddr::from((constant::LOCALHOST, constant::PORT));
        axum::Server::bind(&addr)
        .serve(service.into_make_service())
        .with_graceful_shutdown(exit_on_signal_int())
        .await
        .map_err(Error::from)    
    })
    .await
}
