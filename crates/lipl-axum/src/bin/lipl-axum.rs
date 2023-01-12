use std::{net::SocketAddr};

use axum::Router;
use futures_util::TryFutureExt;
use lipl_axum::{constant, create_service, exit_on_signal_int, Error};

async fn run(service: Router<()>) -> Result<(), Error> {
    let addr = SocketAddr::from((constant::LOCALHOST, constant::PORT));
    axum::Server::bind(&addr)
    .serve(service.into_make_service())
    .with_graceful_shutdown(exit_on_signal_int())
    .err_into()
    .await
}

fn log_filter() -> String {
    std::env::var(constant::RUST_LOG)
    .unwrap_or_else(|_| constant::DEFAULT_LOG_FILTER.to_owned())
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(log_filter())
        .init();

    lipl_axum::create_pool()
        .map_ok(create_service)
        .and_then(run)
        .await
}
