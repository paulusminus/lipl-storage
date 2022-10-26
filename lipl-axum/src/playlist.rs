use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(handler::list).post(handler::post))
        .route("/:id", get(handler::item).delete(handler::delete).put(handler::put))
}

mod handler {
    use crate::{error::Error, to_json_response, to_status_ok, PoolState};
    use axum::{extract::Path, http::StatusCode, Json};
    use futures_util::TryFutureExt;
    use lipl_types::{Playlist, PlaylistPost, Summary};
    use super::db;

    /// Handler for getting all playlists
    pub async fn list(pool: PoolState) -> Result<(StatusCode, Json<Vec<Summary>>), Error> {
        pool.get()
            .map_err(Error::from)
            .and_then(db::list)
            .map_ok(to_json_response(StatusCode::OK))
            .await
    }

    /// Handler for getting a specific playlist
    pub async fn item(
        pool: PoolState,
        Path(id): Path<lipl_types::Uuid>,
    ) -> Result<(StatusCode, Json<Playlist>), Error> {
        pool.get()
            .map_err(Error::from)
            .and_then(|connection| async move { db::item(connection, id).await })
            .map_ok(to_json_response(StatusCode::OK))
            .await
    }

    /// Handler for posting a new playlist
    pub async fn post(
        pool: PoolState,
        Json(playlist_post): Json<PlaylistPost>,
    ) -> Result<(StatusCode, Json<Playlist>), Error> {
        pool.get()
            .map_err(Error::from)
            .and_then(|connection| async move { db::post(connection, playlist_post).await })
            .map_ok(to_json_response(StatusCode::CREATED))
            .await
    }

    /// Handler for deleting a specific playlist
    pub async fn delete(pool: PoolState, Path(id): Path<lipl_types::Uuid>) -> Result<StatusCode, Error> {
        pool.get()
            .map_err(Error::from)
            .and_then(|connection| async move { db::delete(connection, id.inner()).await })
            .map_ok(to_status_ok)
            .await
    }

    /// Handler for changing a specific playlist
    pub async fn put(
        pool: PoolState,
        Path(id): Path<lipl_types::Uuid>,
        Json(playlist_post): Json<PlaylistPost>,
    ) -> Result<(StatusCode, Json<Playlist>), Error> {
        pool.get()
            .map_err(Error::from)
            .and_then(|connection| async move { db::put(connection, id, playlist_post).await })
            .map_ok(to_json_response(StatusCode::OK))
            .await
    }
}

mod db {
    use bb8::PooledConnection;
    use bb8_postgres::PostgresConnectionManager;
    use futures_util::TryFutureExt;
    use lipl_types::{Playlist, PlaylistPost, Summary, Uuid};
    use tokio_postgres::NoTls;

    use crate::constant::sql;
    use crate::error::Error;

    pub async fn list(
        connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    ) -> Result<Vec<Summary>, Error> {
        connection
            .query(sql::playlist::LIST, &[])
            .map_err(Error::from)
            .map_ok(convert::to_list(convert::to_summary))
            .await
    }

    pub async fn item(
        connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        id: Uuid,
    ) -> Result<Playlist, Error> {
        let title = connection
            .query_one(sql::playlist::ITEM_TITLE, &[&id.inner()])
            .map_err(Error::from)
            .await?
            .get::<&str, String>(sql::playlist::column::TITLE);

        let members = connection
            .query(sql::playlist::ITEM_MEMBERS, &[&id.inner()])
            .map_err(Error::from)
            .await?
            .into_iter()
            .map(|row| Uuid::from(row.get::<&str, uuid::Uuid>(sql::playlist::column::LYRIC_ID)))
            .collect::<Vec<_>>();

        Ok(Playlist { id, title, members })
    }

    pub async fn delete(
        connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        id: uuid::Uuid,
    ) -> Result<u64, Error> {
        connection
            .execute(sql::playlist::DELETE, &[&id])
            .map_err(Error::from)
            .await
    }

    pub async fn post(
        connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        playlist_post: PlaylistPost,
    ) -> Result<Playlist, Error> {
        let id = Uuid::default();
        let members = playlist_post
            .members
            .into_iter()
            .map(|uuid| uuid.inner())
            .collect::<Vec<_>>();

        connection
            .query_one(
                sql::playlist::UPSERT,
                &[&id.inner(), &playlist_post.title, &members.as_slice()],
            )
            .map_err(Error::from)
            // .inspect_ok(|row| { println!("Row: {:#?}", row.get::<&str, Option<Vec<uuid::Uuid>>>("fn_upsert_playlist")); })
            .await?;

        item(connection, id).await
    }

    pub async fn put(
        connection: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        id: lipl_types::Uuid,
        playlist_post: PlaylistPost,
    ) -> Result<Playlist, Error> {
        let members = playlist_post
            .members
            .into_iter()
            .map(|uuid| uuid.inner())
            .collect::<Vec<_>>();

        connection
            .query_one(
                sql::playlist::UPSERT,
                &[&id.inner(), &playlist_post.title, &members.as_slice()],
            )
            .map_err(Error::from)
            // .map_ok(|row| row.get::<usize, Vec<uuid::Uuid>>(0))
            .await?;

        item(connection, id).await
    }

    mod convert {
        use crate::constant::sql;
        use lipl_types::Summary;
        use tokio_postgres::Row;

        pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Vec<T>
        where
            F: Fn(Row) -> T + Copy,
        {
            move |rows| rows.into_iter().map(f).collect::<Vec<_>>()
        }

        pub fn to_summary(row: Row) -> Summary {
            Summary {
                id: row
                    .get::<&str, uuid::Uuid>(sql::playlist::column::ID)
                    .into(),
                title: row.get::<&str, String>(sql::playlist::column::TITLE),
            }
        }
    }
}
