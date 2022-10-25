use axum::extract::Extension;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls};
use tower_http::trace::TraceLayer;

use std::{net::SocketAddr, sync::Arc};

mod constant;
mod error;
mod lyric;
mod message;
mod playlist;

async fn exit_on_signal_int() {
    match tokio::signal::ctrl_c().await {
        Ok(_) => { message::exit_on_signal_int(); },
        Err(error) => { message::error_on_receiving_signal(error); }
    };
}

#[tokio::main]
pub async fn main() -> Result<(), error::Error> {
    
    let filter = std::env::var(constant::RUST_LOG).unwrap_or_else(|_| constant::DEFAULT_LOG_FILTER.to_owned());
    
    tracing_subscriber::fmt()
    .with_env_filter(filter)
    .init();

    let manager = 
        PostgresConnectionManager::new_from_stringlike(constant::PG_CONNECTION, NoTls)?;
    let pool = Pool::builder().build(manager).await?;

    let shared_pool = Arc::new(pool);

    let service =
        lyric::lyric_router()
        .merge(playlist::playlist_router())
        .layer(Extension(shared_pool.clone()))
        .layer(TraceLayer::new_for_http())
        .into_make_service();

    let addr = SocketAddr::from((constant::LOCALHOST, constant::PORT));
    
    axum::Server::bind(&addr)
    .serve(service)
    .with_graceful_shutdown(exit_on_signal_int())
    .await
    .map_err(error::Error::from)
}

