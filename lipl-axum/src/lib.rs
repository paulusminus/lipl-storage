use axum::routing::{get};
use axum::{Router};
use lipl_axum_postgres::{PostgresConnectionPool};
use lipl_core::{LyricDb, PlaylistDb};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub use crate::error::Error;
use crate::handler::{lyric, playlist};

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

pub async fn create_pool() -> lipl_axum_postgres::Result<PostgresConnectionPool> {
    let pool = lipl_axum_postgres::connection_pool(crate::constant::PG_CONNECTION).await?;
    Ok(pool)
}

pub fn create_service<T>(t: T) -> Router<()>
where
    T: LyricDb + PlaylistDb + Clone + Send + Sync + 'static,
{
    Router::new().nest(constant::PREFIX, Router::new()
        .route("/lyric", get(lyric::list::<T>).post(lyric::post::<T>))
        .route("/lyric/:id", get(lyric::item::<T>).delete(lyric::delete::<T>).put(lyric::put::<T>))
        .route("/playlist", get(playlist::list::<T>).post(playlist::post::<T>))
        .route("/playlist/:id", get(playlist::item::<T>).delete(playlist::delete::<T>).put(playlist::put::<T>))
    )
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new().br(true).gzip(true))
            .into_inner()
    )
    .with_state(t)
}
