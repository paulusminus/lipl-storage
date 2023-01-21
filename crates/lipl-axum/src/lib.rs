use std::sync::Arc;

use axum::routing::{get};
use axum::{Router};
use lipl_core::{LiplRepo};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub use crate::error::Error;
pub use crate::param::LiplApp;
use crate::handler::{lyric, playlist};

pub mod constant;
mod error;
mod handler;
mod message;
mod param;

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

#[cfg(not(feature = "postgres"))]
pub async fn create_pool() -> Result<Arc<dyn LiplRepo>> {
    Ok(
        Arc::new(
            lipl_repo_memory::MemoryRepo::default()
        )
    )
} 

#[cfg(feature = "postgres")]
pub async fn create_pool(use_postgres: bool) -> Result<Arc<dyn LiplRepo>> {
    if use_postgres {
        let pool = lipl_axum_postgres::connection_pool(crate::constant::PG_CONNECTION).await?;
        Ok(
            Arc::new(pool)
        )    
    }
    else {
        Ok(
            Arc::new(
                lipl_repo_memory::MemoryRepo::default(),
            )
        )
    }
}

pub fn create_service(t: Arc<dyn LiplRepo>) -> Router<()>
{
    Router::new().nest(constant::PREFIX, Router::new()
        .route("/lyric", get(lyric::list).post(lyric::post))
        .route("/lyric/:id", get(lyric::item).delete(lyric::delete).put(lyric::put))
        .route("/playlist", get(playlist::list).post(playlist::post))
        .route("/playlist/:id", get(playlist::item).delete(playlist::delete).put(playlist::put))
    )
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new().br(true).gzip(true))
            .into_inner()
    )
    .with_state(t)
}
