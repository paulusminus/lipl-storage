use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::routing::get;
use axum::{async_trait, Json, Router};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use hyper::StatusCode;
use tokio_postgres::NoTls;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub use crate::error::Error;

pub mod constant;
mod error;
mod ext;
mod lyric;
mod message;
mod playlist;

pub(crate) type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct DatabaseConnection(PooledConnection<'static, PostgresConnectionManager<NoTls>>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    ConnectionPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        ConnectionPool::from_ref(state)
            .get_owned()
            .await
            .map_err(Error::from)
            .map(Self)
    }
}

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

pub async fn create_service() -> Result<Router<Pool<PostgresConnectionManager<NoTls>>>, Error> {
    let manager = PostgresConnectionManager::new_from_stringlike(constant::PG_CONNECTION, NoTls)?;
    let pool = Pool::builder().build(manager).await?;
    // let shared_pool = Arc::new(pool);

    Ok(Router::with_state(pool)
        .route("/api/v1/lyric", get(lyric::list).post(lyric::post))
        .route(
            "/api/v1/lyric/:id",
            get(lyric::item).delete(lyric::delete).put(lyric::put),
        )
        .route("/api/v1/playlist", get(playlist::list).post(playlist::post))
        .route(
            "/api/v1/playlist/:id",
            get(playlist::item)
                .delete(playlist::delete)
                .put(playlist::put),
        )
        // .layer(Extension(shared_pool.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new().br(true).gzip(true)))
}
