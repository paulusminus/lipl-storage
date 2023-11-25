use std::net::{IpAddr, SocketAddr};

use axum::Router;
use futures_util::TryFutureExt;
use lipl_core::Result;
use lipl_storage_server::{constant, create_services, exit_on_signal_int, router_from_environment};

async fn run(router: Router) -> Result<()> {
    let localhost = if constant::USE_IPV6 {
        IpAddr::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    } else {
        IpAddr::from([0, 0, 0, 0])
    };
    let addr = SocketAddr::from((localhost, constant::PORT));
    axum::Server::bind(&addr)
        .serve(
            router
                .layer(create_services().into_inner())
                .into_make_service(),
        )
        .with_graceful_shutdown(exit_on_signal_int())
        .map_err(|error| lipl_core::Error::Axum(Box::new(error)))
        .await
}

fn log_filter() -> String {
    std::env::var(constant::RUST_LOG).unwrap_or_else(|_| constant::DEFAULT_LOG_FILTER.to_owned())
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(log_filter())
        .init();

    if let Err(error) = router_from_environment()
        .and_then(|router| run(router).err_into())
        .await
    {
        tracing::error!("Failed with error {error}");
    }
}
