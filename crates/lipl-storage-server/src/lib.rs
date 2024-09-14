use axum::routing::get;
use axum::Router;
use futures_util::TryFutureExt;
use hyper::StatusCode;
use lipl_core::ToRepo;
use tokio::signal::unix::{signal, SignalKind};
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::compression::CompressionLayer;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest,
    DefaultOnResponse, TraceLayer,
};
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::Level;

pub use crate::error::Error;
use crate::handler::{db, lyric, playlist};

pub mod constant;
pub mod environment;
mod error;
mod handler;
mod message;

pub type Result<T> = std::result::Result<T, Error>;

pub fn username() -> Result<String> {
    std::env::var("LIPL_USERNAME").map_err(Into::into)
}

pub fn password() -> Result<String> {
    std::env::var("LIPL_PASSWORD").map_err(Into::into)
}

#[cfg(feature = "pwa")]
pub async fn router_from_environment() -> Result<Router> {
    use std::env::var;

    use tower_http::services::ServeDir;

    futures_util::future::ready(environment::repo_type())
        .and_then(|repo_type| create_router(repo_type).err_into())
        .await
        .map(|router| {
            router.nest_service(
                "/",
                ServeDir::new(var("WWW_ROOT").unwrap_or(".".to_owned())),
            )
        })
}

#[cfg(not(feature = "pwa"))]
pub async fn router_from_environment() -> Result<Router> {
    futures_util::future::ready(environment::repo_type())
        .and_then(|repo_type| create_router(repo_type).err_into())
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

#[allow(clippy::type_complexity)]
pub fn create_services() -> ServiceBuilder<
    Stack<CompressionLayer, Stack<TraceLayer<SharedClassifier<ServerErrorsAsFailures>>, Identity>>,
> {
    ServiceBuilder::new().layer(logging()).layer(compression())
}

async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn create_router<T>(t: T) -> lipl_core::Result<Router>
where
    T: ToRepo,
{
    let username = std::env::var("LIPL_USERNAME")?;
    let password = std::env::var("LIPL_PASSWORD")?;

    t.to_repo()
        .map_ok(|state| {
            Router::new()
                .route(&format!("{}/health", constant::PREFIX), get(health))
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
                        )
                        .route("/db", get(db::get).put(db::put))
                        .layer(ValidateRequestHeaderLayer::basic(&username, &password))
                        .with_state(state),
                )
        })
        .await
}
