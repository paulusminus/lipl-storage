use axum::routing::get;
use axum::{Router, RouterService};
use lipl_axum_postgres::{connection_pool};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub use crate::error::Error;

pub mod constant;
mod error;
mod handler;
mod message;

pub type Result<T> = std::result::Result<T, Error>;

#[inline]
pub async fn exit_on_signal_int() {
    match tokio::signal::ctrl_c().await {
        Ok(_) => {
            message::exit_on_signal_int();
        }
        Err(error) => {
            message::error_on_receiving_signal(error);
        }
    };
}

fn prefixed(path: &'static str) -> String {
    format!("{}/{}", constant::PREFIX, path)
}

pub async fn create_service() -> lipl_axum_postgres::Result<RouterService> {
    connection_pool(constant::PG_CONNECTION)
    .await
    .map(|pool| Router::new()
        .route(
            &prefixed("lyric"),
            get(handler::lyric::list).post(handler::lyric::post),
        )
        .route(
            &prefixed("lyric/:id"),
            get(handler::lyric::item)
                .delete(handler::lyric::delete)
                .put(handler::lyric::put),
        )
        .route(
            &prefixed("playlist"),
            get(handler::playlist::list).post(handler::playlist::post),
        )
        .route(
            &prefixed("playlist/:id"),
            get(handler::playlist::item)
                .delete(handler::playlist::delete)
                .put(handler::playlist::put),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new().br(true).gzip(true))
        )
        .with_state(pool)
    )      
}
