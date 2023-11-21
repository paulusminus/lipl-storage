use axum::routing::get;
use axum::Router;
use futures_util::TryFutureExt;
use lipl_core::ToRepo;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub use crate::error::Error;
use crate::handler::{lyric, playlist};
pub use crate::param::app::LiplApp;

pub mod constant;
pub mod environment;
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

pub async fn create_service<T>(t: T) -> lipl_core::Result<Router>
where
    T: ToRepo,
{
    t.to_repo()
        .map_ok(|state| {
            Router::new()
                .nest(
                    constant::PREFIX,
                    Router::new()
                        .route("/lyric", get(lyric::list).post(lyric::post))
                        .route(
                            "/lyric/:id",
                            get(lyric::item).delete(lyric::delete).put(lyric::put),
                        )
                        .route("/playlist", get(playlist::list).post(playlist::post))
                        .route(
                            "/playlist/:id",
                            get(playlist::item)
                                .delete(playlist::delete)
                                .put(playlist::put),
                        ),
                )
                .layer(
                    ServiceBuilder::new()
                        .layer(TraceLayer::new_for_http())
                        .layer(CompressionLayer::new().br(true).gzip(true))
                        .into_inner(),
                )
                .with_state(state)
        })
        .await
}
