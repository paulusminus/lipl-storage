use axum::routing::get;
use axum::Router;
use futures_util::TryFutureExt;
use lipl_core::ToRepo;
use tokio::signal::unix::{signal, SignalKind};
use tower::ServiceBuilder;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::compression::CompressionLayer;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest,
    DefaultOnResponse, TraceLayer,
};
use tracing::Level;

pub use crate::error::Error;
use crate::handler::{lyric, playlist};

pub mod constant;
pub mod environment;
mod error;
mod handler;
mod message;

pub type Result<T> = std::result::Result<T, Error>;

pub async fn router_from_environment() -> Result<Router> {
    futures_util::future::ready(environment::repo_type())
        .and_then(|repo_type| create_service(repo_type).err_into())
        .await
}

#[inline]
fn logging() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
        .on_request(DefaultOnRequest::default())
        .on_body_chunk(DefaultOnBodyChunk::default())
        .on_eos(DefaultOnEos::default())
        .on_failure(DefaultOnFailure::default())
}

#[inline]
fn compression() -> CompressionLayer {
    CompressionLayer::new().br(true).gzip(true)
}

#[cfg(windows)]
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

#[cfg(unix)]
#[inline]
pub async fn exit_on_signal_int() {
    let mut wait_on_term_stream = signal(SignalKind::terminate()).unwrap();
    let mut wait_on_term_int = signal(SignalKind::interrupt()).unwrap();

    tokio::select! {
        on_int = wait_on_term_int.recv() => {
            if on_int.is_some() {
                message::exit_on_signal_int();
            }
        }
        on_term = wait_on_term_stream.recv() => {
            if on_term.is_some() {
                message::exit_on_signal_term();
            }
        }
    }
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
                        .layer(logging())
                        .layer(compression())
                        .into_inner(),
                )
                .with_state(state)
        })
        .await
}
