use std::sync::Arc;

use axum::{Router, routing::get, Extension, http::StatusCode, Json, extract::Path};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use futures_util::{TryFutureExt};
use lipl_types::{Summary, Uuid, Playlist, PlaylistPost};
use tokio_postgres::{NoTls, Row};
use crate::error;
use crate::constant::sql;

pub fn playlist_router() -> Router {
    Router::new()
    .route("/api/v1/playlist", get(playlist_list).post(playlist_post))
    .route("/api/v1/playlist/:id", get(playlist_item).delete(playlist_delete).put(playlist_put))
}

fn to_summary(row: Row) -> Summary {
    Summary {
        id: row.get::<&str, uuid::Uuid>(sql::playlist::column::ID).into(),
        title: row.get::<&str, String>(sql::playlist::column::TITLE),
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
    .query(sql::playlist::LIST, &[])
    .map_err(error::Error::from)
    .await
}

async fn item(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid) -> Result<Playlist, error::Error> {
    let title = 
        connection
        .query_one(sql::playlist::ITEM_TITLE, &[&id])
        .map_err(error::Error::from)
        .await?
        .get::<&str, String>(sql::playlist::column::TITLE);

    let members = 
        connection
        .query(sql::playlist::ITEM_MEMBERS, &[&id])
        .map_err(error::Error::from)
        .await?
        .into_iter()
        .map(|row| Uuid::from(row.get::<&str, uuid::Uuid>(sql::playlist::column::LYRIC_ID)))
        .collect::<Vec<_>>();

    Ok(
        Playlist {
            id: Uuid::from(id),
            title,
            members
        }
    )
}

async fn delete(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid) -> Result<u64, error::Error> {
    connection
    .execute(sql::playlist::DELETE, &[&id])
    .map_err(error::Error::from)
    .await
}

async fn post(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, playlist_post: PlaylistPost) -> Result<Playlist, error::Error> {
    let id = Uuid::default();
    let members = playlist_post.members.into_iter().map(|uuid| uuid.inner()).collect::<Vec<_>>();

    let posted_members = connection
    .query_one(sql::playlist::UPSERT, &[&id.inner(), &playlist_post.title, &members.as_slice()])
    .map_err(error::Error::from)
    .map_ok(|row| row.get::<usize, Vec<uuid::Uuid>>(0))
    .await?;

    let playlist = Playlist {
        id,
        title: playlist_post.title,
        members: posted_members.into_iter().map(Uuid::from).collect(),
    };
    Ok(playlist)
}

async fn put(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid, playlist_post: PlaylistPost) -> Result<Vec<Row>, error::Error> {
    let members = playlist_post.members.into_iter().map(|uuid| uuid.inner()).collect::<Vec<_>>();
    connection
    .query(sql::playlist::UPSERT, &[&id, &playlist_post.title, &members.as_slice()])
    .map_err(error::Error::from)
    .await
}


/// Handler for getting all playlists
async fn playlist_list(state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>) -> Result<(StatusCode, Json<Vec<Summary>>), error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(list)
    .map_ok(to_list(to_summary))
    .map_ok(to_response)
    .await
}

/// Handler for posting a new playlist
async fn playlist_post(
    state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>), error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { post(connection, playlist_post).await })
    .map_ok(|playlist| (StatusCode::CREATED, Json(playlist)))
    .await
}

/// Handler for getting a specific playlist
async fn playlist_item(state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>, Path(id): Path<lipl_types::Uuid>) -> Result<(StatusCode, Json<Playlist>), error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { item(connection, id.inner()).await })
    .map_ok(to_response)
    .await
}

/// Handler for deleting a specific playlist
async fn playlist_delete(state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>, Path(id): Path<lipl_types::Uuid>) -> Result<StatusCode, error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { delete(connection, id.inner()).await })
    .map_ok(|_| StatusCode::OK )
    .await
}

/// Handler for changing a specific playlist
async fn playlist_put(state: Extension<Arc<Pool<PostgresConnectionManager<NoTls>>>>, Path(id): Path<lipl_types::Uuid>, Json(playlist_post): Json<PlaylistPost>) -> Result<StatusCode, error::Error> {
    state.get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { put(connection, id.inner(), playlist_post).await })
    .map_ok(|_| StatusCode::OK )
    .await
}
