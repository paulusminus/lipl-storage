use axum::routing::get;
use axum::{Router};
use lipl_axum_postgres::{ConnectionPool, connection_pool};
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub use crate::error::Error;

pub mod constant;
mod error;
mod ext;
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

pub async fn create_service() -> lipl_axum_postgres::Result<Router<ConnectionPool>> {
    let pool = connection_pool(constant::PG_CONNECTION).await?;

    Ok(Router::with_state(pool)
        .route(
            "/api/v1/lyric",
            get(handler::lyric::list).post(handler::lyric::post),
        )
        .route(
            "/api/v1/lyric/:id",
            get(handler::lyric::item)
                .delete(handler::lyric::delete)
                .put(handler::lyric::put),
        )
        .route(
            "/api/v1/playlist",
            get(handler::playlist::list).post(handler::playlist::post),
        )
        .route(
            "/api/v1/playlist/:id",
            get(handler::playlist::item)
                .delete(handler::playlist::delete)
                .put(handler::playlist::put),
        )
        // .layer(Extension(shared_pool.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new().br(true).gzip(true)))
}
