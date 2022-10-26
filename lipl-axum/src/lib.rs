use std::sync::Arc;

use axum::{extract::Extension, Json, Router};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use hyper::StatusCode;
use tokio_postgres::NoTls;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub use crate::error::Error;

pub mod constant;
mod error;
mod lyric;
mod message;
mod playlist;

pub(crate) type ConnectionPool = Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>;

pub(crate) fn to_json_response<T>(status_code: StatusCode) -> impl Fn(T) -> (StatusCode, Json<T>) {
    move |t| (status_code, Json(t))
}

pub(crate) fn to_status_ok<T>(_: T) -> StatusCode {
    StatusCode::OK
}

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

pub async fn create_service() -> Result<Router, Error> {
    let manager = PostgresConnectionManager::new_from_stringlike(constant::PG_CONNECTION, NoTls)?;
    let pool = Pool::builder().build(manager).await?;
    let shared_pool = Arc::new(pool);

    Ok(Router::new()
        .nest(
            constant::PREFIX,
            Router::new()
                .nest("/lyric", lyric::router())
                .nest("/playlist", playlist::router()),
        )
        .layer(Extension(shared_pool.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new().br(true).gzip(true)))
}
