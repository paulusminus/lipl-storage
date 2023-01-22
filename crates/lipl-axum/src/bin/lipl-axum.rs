use std::{net::SocketAddr};

use axum::Router;
use clap::Parser;
use futures_util::TryFutureExt;
use lipl_axum::{constant, create_service, exit_on_signal_int, LiplApp};
use lipl_core::{Result};

async fn run(service: Router) -> Result<()> {
    let addr = SocketAddr::from((constant::LOCALHOST, constant::PORT));
    axum::Server::bind(&addr)
    .serve(service.into_make_service())
    .with_graceful_shutdown(exit_on_signal_int())
    .map_err(|error| lipl_core::Error::Axum(Box::new(error)))
    .await 
}

fn log_filter() -> String {
    std::env::var(constant::RUST_LOG)
    .unwrap_or_else(|_| constant::DEFAULT_LOG_FILTER.to_owned())
}

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(log_filter())
        .init();

    create_service(LiplApp::parse())
        .and_then(run)
        .await
}
