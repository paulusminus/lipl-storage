use std::sync::Arc;

use axum::routing::{get};
use axum::{Router, RouterService};
use lipl_axum_postgres::{connection_pool};
use tower::{ServiceBuilder};
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

pub async fn create_service() -> lipl_axum_postgres::Result<RouterService> {
    connection_pool(constant::PG_CONNECTION)
    .await
    .map(|pool| 
        Router::new().nest(constant::PREFIX, Router::new()
            .route(
            "/lyric",
    get(lyric::list).post(lyric::post),
            )
            .route(
            "/lyric/:id",
   get(lyric::item).delete(lyric::delete).put(lyric::put),
            )
            .route(
            "/playlist",
    get(playlist::list).post(playlist::post),
            )
            .route(
            "/playlist/:id",
  get(playlist::item).delete(playlist::delete).put(playlist::put),
            )
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new().br(true).gzip(true))
                .into_inner()
        )
        .with_state(Arc::new(pool))
    )      
}
