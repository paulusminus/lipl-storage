use axum::Router;
use axum::routing::get;
use hyper::StatusCode;
use lipl_core::{Repo, ToRepo};
use std::sync::Arc;
use tokio::signal::unix::{SignalKind, signal};
use tower::ServiceBuilder;
use tower::layer::util::{Identity, Stack};
use tower_http::auth::AddAuthorizationLayer;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::compression::CompressionLayer;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest,
    DefaultOnResponse, TraceLayer,
};
use tracing::Level;

use crate::environment::{password, username};
#[cfg(feature = "pwa")]
pub use crate::error::Error;
use crate::handler::{db, lyric, playlist};

pub mod constant;
pub mod environment;
mod error;
mod handler;
mod message;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "pwa")]
pub fn router_from_environment() -> Result<Router> {
    use environment::www_root;
    use tower_http::services::ServeDir;

    let router = environment::repo()?.fallback_service(ServeDir::new(www_root()));
    Ok(router)
}

#[cfg(not(feature = "pwa"))]
pub fn router_from_environment() -> Result<Router> {
    let router = environment::repo()?;
    Ok(router)
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

#[allow(clippy::type_complexity)]
pub fn create_services() -> ServiceBuilder<
    Stack<CompressionLayer, Stack<TraceLayer<SharedClassifier<ServerErrorsAsFailures>>, Identity>>,
> {
    ServiceBuilder::new().layer(logging()).layer(compression())
}

async fn health() -> StatusCode {
    StatusCode::OK
}

pub fn create_router<S>(state: S) -> Router
where
    S: Repo + 'static + Send + Sync,
{
    Router::new()
        .route(&format!("{}/health", constant::PREFIX), get(health))
        .nest(
            constant::PREFIX,
            Router::new()
                .route("/lyric", get(lyric::list::<S>).post(lyric::post::<S>))
                .route(
                    "/lyric/{id}",
                    get(lyric::item::<S>)
                        .delete(lyric::delete::<S>)
                        .put(lyric::put::<S>),
                )
                .route(
                    "/playlist",
                    get(playlist::list::<S>).post(playlist::post::<S>),
                )
                .route(
                    "/playlist/{id}",
                    get(playlist::item::<S>)
                        .delete(playlist::delete::<S>)
                        .put(playlist::put::<S>),
                )
                .route("/db", get(db::get::<S>).put(db::put::<S>))
                .layer(AddAuthorizationLayer::basic(
                    &username().unwrap(),
                    &password().unwrap(),
                ))
                .with_state(Arc::new(state)),
        )
}
