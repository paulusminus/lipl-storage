use axum::{Router, routing::get, http::StatusCode, Json, extract::Path};
use futures_util::{TryFutureExt};
use lipl_types::{Summary, Playlist, PlaylistPost};
use crate::{error, PoolState};

pub fn playlist_router() -> Router {
    Router::new()
    .route("/api/v1/playlist", get(playlist_list).post(playlist_post))
    .route("/api/v1/playlist/:id", get(playlist_item).delete(playlist_delete).put(playlist_put))
}

/// Handler for getting all playlists
async fn playlist_list(
    pool: PoolState,
) -> Result<(StatusCode, Json<Vec<Summary>>), error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(db::list)
    .map_ok(crate::to_json_response(StatusCode::OK))
    .await
}

/// Handler for getting a specific playlist
async fn playlist_item(
    pool: PoolState,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<(StatusCode, Json<Playlist>), error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { db::item(connection, id.inner()).await })
    .map_ok(crate::to_json_response(StatusCode::OK))
    .await
}

/// Handler for posting a new playlist
async fn playlist_post(
    pool: PoolState,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>), error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { db::post(connection, playlist_post).await })
    .map_ok(crate::to_json_response(StatusCode::CREATED))
    .await
}

/// Handler for deleting a specific playlist
async fn playlist_delete(
    pool: PoolState,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<StatusCode, error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { db::delete(connection, id.inner()).await })
    .map_ok(|_| StatusCode::OK )
    .await
}

/// Handler for changing a specific playlist
async fn playlist_put(
    pool: PoolState,
    Path(id): Path<lipl_types::Uuid>,
    Json(playlist_post): Json<PlaylistPost>,
) -> Result<(StatusCode, Json<Playlist>), error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { db::put(connection, id, playlist_post).await })
    .map_ok(crate::to_json_response(StatusCode::OK))
    .await
}

mod db {
    use bb8::PooledConnection;
    use bb8_postgres::PostgresConnectionManager;
    use futures_util::TryFutureExt;
    use lipl_types::{Playlist, Uuid, PlaylistPost, Summary};
    use tokio_postgres::{NoTls};

    use crate::constant::sql;
    use crate::error::Error;

    pub async fn list(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>) -> Result<Vec<Summary>, Error> {
        connection
        .query(sql::playlist::LIST, &[])
        .map_err(Error::from)
        .map_ok(convert::to_list(convert::to_summary))
        .await
    }
    
    pub async fn item(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid) -> Result<Playlist, Error> {
        let title = 
            connection
            .query_one(sql::playlist::ITEM_TITLE, &[&id])
            .map_err(Error::from)
            .await?
            .get::<&str, String>(sql::playlist::column::TITLE);
    
        let members = 
            connection
            .query(sql::playlist::ITEM_MEMBERS, &[&id])
            .map_err(Error::from)
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
    
    pub async fn delete(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid) -> Result<u64, Error> {
        connection
        .execute(sql::playlist::DELETE, &[&id])
        .map_err(Error::from)
        .await
    }
    
    pub async fn post(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, playlist_post: PlaylistPost) -> Result<Playlist, Error> {
        let id = Uuid::default();
        let members = playlist_post.members.into_iter().map(|uuid| uuid.inner()).collect::<Vec<_>>();
    
        let posted_members = connection
        .query_one(sql::playlist::UPSERT, &[&id.inner(), &playlist_post.title, &members.as_slice()])
        .map_err(Error::from)
        .map_ok(|row| row.get::<usize, Vec<uuid::Uuid>>(0))
        .await?;
    
        let playlist = Playlist {
            id,
            title: playlist_post.title,
            members: posted_members.into_iter().map(Uuid::from).collect(),
        };
        Ok(playlist)
    }
    
    pub async fn put(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: lipl_types::Uuid, playlist_post: PlaylistPost) -> Result<Playlist, Error> {
        let members = playlist_post.members.into_iter().map(|uuid| uuid.inner()).collect::<Vec<_>>();
        let posted_members = connection
        .query_one(sql::playlist::UPSERT, &[&id.inner(), &playlist_post.title, &members.as_slice()])
        .map_err(Error::from)
        .map_ok(|row| row.get::<usize, Vec<uuid::Uuid>>(0))
        .await?;
    
        let playlist = Playlist {
            id,
            title: playlist_post.title,
            members: posted_members.into_iter().map(Uuid::from).collect(),
        };
    
        Ok(playlist)
    }

    mod convert {
        use lipl_types::Summary;
        use tokio_postgres::Row;
        use crate::constant::sql;

        pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Vec<T> 
        where F: Fn(Row) -> T + Copy
        {
            move |rows| rows.into_iter().map(f).collect::<Vec<_>>()
        }
        
        pub fn to_summary(row: Row) -> Summary {
            Summary {
                id: row.get::<&str, uuid::Uuid>(sql::playlist::column::ID).into(),
                title: row.get::<&str, String>(sql::playlist::column::TITLE),
            }
        }               
    }
}