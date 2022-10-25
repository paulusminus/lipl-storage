use std::sync::Arc;

use axum::{Router, routing::get, Extension, http::StatusCode, Json, extract::Path};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use futures_util::{TryFutureExt};
use lipl_types::{Lyric, LyricPost, Summary};
use parts::to_text;
use tokio_postgres::{NoTls, Row};
use crate::error;
use crate::constant::sql;

pub fn lyric_router() -> Router {
    Router::new()
    .route("/api/v1/lyric", get(lyric_list).post(lyric_post))
    .route("/api/v1/lyric/:id", get(lyric_item).delete(lyric_delete).put(lyric_put))
}

fn to_lyric(row: Row) -> Lyric {
    Lyric {
        id: row.get::<&str, uuid::Uuid>(sql::lyric::column::ID).into(),
        title: row.get::<&str, String>(sql::lyric::column::TITLE),
        parts: parts::to_parts(row.get::<&str, String>(sql::lyric::column::PARTS)),
    }
}

fn to_summary(row: Row) -> Summary {
    Summary {
        id: row.get::<&str, uuid::Uuid>(sql::lyric::column::ID).into(),
        title: row.get::<&str, String>(sql::lyric::column::TITLE),
    }
}

fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Vec<T> 
where F: Fn(Row) -> T + Copy
{
    move |rows| rows.into_iter().map(f).collect::<Vec<_>>()
}

fn to_response<T>(t: T) -> (StatusCode, Json<T>) {
    (StatusCode::OK, Json(t))
}

async fn list(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, error::Error> {
    connection
    .query(sql::lyric::LIST, &[])
    .map_err(error::Error::from)
    .await
}

async fn item(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid) -> Result<Row, error::Error> {
    connection
    .query_one(sql::lyric::ITEM, &[&id])
    .map_err(error::Error::from)
    .await
}

async fn delete(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid) -> Result<u64, error::Error> {
    connection
    .execute(sql::lyric::DELETE, &[&id])
    .map_err(error::Error::from)
    .await
}

async fn post(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, lyric_post: LyricPost) -> Result<Lyric, error::Error> {
    let id = lipl_types::Uuid::default();
    connection
    .execute(sql::lyric::INSERT, &[&id.inner(), &lyric_post.title, &to_text(&lyric_post.parts)])
    .map_err(error::Error::from)
    .await?;

    let lyric = Lyric {
        id,
        title: lyric_post.title,
        parts: lyric_post.parts,
    };

    Ok(lyric)
}

async fn put(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid, lyric_post: LyricPost) -> Result<u64, error::Error> {
    connection
    .execute(sql::lyric::UPDATE, &[&lyric_post.title, &to_text(&lyric_post.parts), &id])
    .map_err(error::Error::from)
    .await
}


/// Handler for getting all lyrics
async fn lyric_list(state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>) -> Result<(StatusCode, Json<Vec<Summary>>), error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(list)
    .map_ok(to_list(to_summary))
    .map_ok(to_response)
    .await
}

/// Handler for posting a new lyric
async fn lyric_post(
    state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>,
    Json(lyric_post): Json<LyricPost>
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { post(connection, lyric_post).await })
    .map_ok(|lyric| (StatusCode::CREATED, Json(lyric)))
    .await
}

/// Handler for getting a specific lyric
async fn lyric_item(state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>, Path(id): Path<lipl_types::Uuid>) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { item(connection, id.inner()).await })
    .map_ok(to_lyric)
    .map_ok(to_response)
    .await
}

/// Handler for deleting a specific lyric
async fn lyric_delete(state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>, Path(id): Path<lipl_types::Uuid>) -> Result<StatusCode, error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { delete(connection, id.inner()).await })
    .map_ok(|_| StatusCode::OK )
    .await
}

/// Handler for changing a specific lyric
async fn lyric_put(state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>, Path(id): Path<lipl_types::Uuid>, Json(lyric_post): Json<LyricPost>) -> Result<StatusCode, error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { put(connection, id.inner(), lyric_post).await })
    .map_ok(|_| StatusCode::OK )
    .await
}
