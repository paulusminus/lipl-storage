use axum::{Router, routing::get, http::StatusCode, Json, extract::Path};
use futures_util::{TryFutureExt};
use lipl_types::{Lyric, LyricPost, Summary};
use crate::{error, PoolState};

pub fn lyric_router() -> Router {
    Router::new()
    .route("/api/v1/lyric", get(lyric_list).post(lyric_post))
    .route("/api/v1/lyric/:id", get(lyric_item).delete(lyric_delete).put(lyric_put))
}

/// Handler for getting all lyrics
async fn lyric_list(
    pool: PoolState,
) -> Result<(StatusCode, Json<Vec<Summary>>), error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(db::list)
    .map_ok(crate::to_json_response(StatusCode::OK))
    .await
}

/// Handler for getting a specific lyric
async fn lyric_item(
    pool: PoolState,
    Path(id): Path<lipl_types::Uuid>
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { db::item(connection, id.inner()).await })
    .map_ok(crate::to_json_response(StatusCode::OK))
    .await
}

/// Handler for posting a new lyric
async fn lyric_post(
    pool: PoolState,
    Json(lyric_post): Json<LyricPost>
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { db::post(connection, lyric_post).await })
    .map_ok(crate::to_json_response(StatusCode::CREATED))
    .await
}

/// Handler for deleting a specific lyric
async fn lyric_delete(
    pool: PoolState,
    Path(id): Path<lipl_types::Uuid>,
) -> Result<StatusCode, error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { db::delete(connection, id.inner()).await })
    .map_ok(crate::to_status_ok)
    .await
}

/// Handler for changing a specific lyric
async fn lyric_put(
    pool: PoolState,
    Path(id): Path<lipl_types::Uuid>,
    Json(lyric_post): Json<LyricPost>
) -> Result<(StatusCode, Json<Lyric>), error::Error> {
    pool
    .get()
    .map_err(error::Error::from)
    .and_then(|connection| async move { db::put(connection, id, lyric_post).await })
    .map_ok(crate::to_json_response(StatusCode::OK))
    .await
}

mod db {
    use bb8::{PooledConnection};
    use bb8_postgres::PostgresConnectionManager;
    use futures_util::{TryFutureExt};
    use lipl_types::{Lyric, LyricPost, Uuid, Summary};
    use parts::to_text;
    use tokio_postgres::{NoTls};

    use crate::constant::sql;
    use crate::error::Error;
    
    pub async fn list(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>) -> Result<Vec<Summary>, Error> {
        connection
        .query(sql::lyric::LIST, &[])
        .map_err(Error::from)
        .map_ok(convert::to_list(convert::to_summary))
        .await
    }
    
    pub async fn item(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid) -> Result<Lyric, Error> {
        connection
        .query_one(sql::lyric::ITEM, &[&id])
        .map_err(Error::from)
        .map_ok(convert::to_lyric)
        .await
    }
    
    pub async fn delete(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: uuid::Uuid) -> Result<u64, Error> {
        connection
        .execute(sql::lyric::DELETE, &[&id])
        .map_err(Error::from)
        .await
    }
    
    pub async fn post(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, lyric_post: LyricPost) -> Result<Lyric, Error> {
        let id = lipl_types::Uuid::default();

        connection
        .execute(sql::lyric::INSERT, &[&id.inner(), &lyric_post.title, &to_text(&lyric_post.parts)])
        .map_err(Error::from)
        .await?;
    
        let lyric = Lyric {
            id,
            title: lyric_post.title,
            parts: lyric_post.parts,
        };
    
        Ok(lyric)
    }
    
    pub async fn put(connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>, id: Uuid, lyric_post: LyricPost) -> Result<Lyric, Error> {
        connection
        .execute(sql::lyric::UPDATE, &[&lyric_post.title, &to_text(&lyric_post.parts), &id.inner()])
        .map_err(Error::from)
        .await?;

        let lyric = Lyric {
            id,
            title: lyric_post.title,
            parts: lyric_post.parts,
        };

        Ok(lyric)
    }

    mod convert {
        use lipl_types::{Lyric, Summary};
        use tokio_postgres::Row;

        use crate::constant::sql;

        pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Vec<T> 
        where F: Fn(Row) -> T + Copy
        {
            move |rows| rows.into_iter().map(f).collect::<Vec<_>>()
        }
        
        pub fn to_lyric(row: Row) -> Lyric {
            Lyric {
                id: row.get::<&str, uuid::Uuid>(sql::lyric::column::ID).into(),
                title: row.get::<&str, String>(sql::lyric::column::TITLE),
                parts: parts::to_parts(row.get::<&str, String>(sql::lyric::column::PARTS)),
            }
        }
        
        pub fn to_summary(row: Row) -> Summary {
            Summary {
                id: row.get::<&str, uuid::Uuid>(sql::lyric::column::ID).into(),
                title: row.get::<&str, String>(sql::lyric::column::TITLE),
            }
        }
    }
}