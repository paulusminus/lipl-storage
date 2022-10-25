use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Extension, Json, Router, routing::IntoMakeService};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use futures_util::TryFutureExt;
use hyper::StatusCode;
use tokio_postgres::{NoTls};
use tower_http::trace::TraceLayer;

use crate::error::Error;

mod constant;
mod error;
mod lyric;
mod message;
mod playlist;

pub(crate) type PoolState = Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>;

pub(crate) fn to_json_response<T>(status_code: StatusCode) -> impl Fn(T) -> (StatusCode, Json<T>) {
    move |t| (status_code, Json(t))
}

pub(crate) fn to_status_ok<T>(_: T) -> StatusCode {
    StatusCode::OK
}

#[inline]
async fn exit_on_signal_int() {
    match tokio::signal::ctrl_c().await {
        Ok(_) => { message::exit_on_signal_int(); },
        Err(error) => { message::error_on_receiving_signal(error); }
    };
}

pub async fn create_service() -> Result<IntoMakeService<Router>, Error> {
    let manager = 
        PostgresConnectionManager::new_from_stringlike(constant::PG_CONNECTION, NoTls)?;

    let pool = Pool::builder().build(manager).await?;

    let shared_pool = Arc::new(pool);

    let service =
        Router::new().nest(
            "/api/v1",
            Router::new()
            .nest("/lyric", lyric::router())
            .nest("/playlist", playlist::router())
        )
        .layer(Extension(shared_pool.clone()))
        .layer(TraceLayer::new_for_http())
        .into_make_service();
    
    Ok(service)
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    
    let filter =
        std::env::var(constant::RUST_LOG)
        .unwrap_or_else(|_| constant::DEFAULT_LOG_FILTER.to_owned());
    
    tracing_subscriber::fmt()
    .with_env_filter(filter)
    .init();

    create_service()
    .and_then(|service| async move {
        let addr = SocketAddr::from((constant::LOCALHOST, constant::PORT));
        axum::Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(exit_on_signal_int())
        .await
        .map_err(Error::from)    
    })
    .await
}

