
macro_rules! create_handler {
    ($name:ident, $list:ident, $item:ident, $add:ident, $delete:ident, $update:ident, $post_type:path) => {
        pub mod $name {
            use std::sync::{Arc};
            use tokio::sync::{RwLock};
            use lipl_io::model::{Db, HasSummaries};
            use lipl_types::{Uuid};
            use warp::{Reply, Rejection};
            use warp::reply::with_status;
            use warp::http::status::StatusCode;
            use crate::model::{Query};

            type SharedDb = Arc<RwLock<Db>>;

            pub async fn list_summary(db: SharedDb) -> Result<impl Reply, Rejection> {
                let result = db.read().await.$list().to_summaries();
                Ok(warp::reply::json(&result))
            }

            pub async fn list(db: SharedDb, query: Query) -> Result<impl Reply, Rejection> {
                if query.full {
                    let result = db.read().await.$list();
                    Ok(warp::reply::json(&result))
                } else {
                    Err(warp::reject::not_found())
                }
            }

            pub async fn item(id: Uuid, db: SharedDb) -> Result<impl Reply, Rejection> {
                let result = db.read().await.$item(&id);
                result.map_or_else(
                    | | Err(warp::reject::not_found()),
                    |r| Ok(warp::reply::json(&r)),
                )
            }

            pub async fn post(
                db: SharedDb,
                json: $post_type,
            ) -> Result<impl Reply, Rejection> {
                let result = db.write().await.$add(&json);
                Ok(with_status(warp::reply::json(&result), StatusCode::CREATED))
            }

            pub async fn delete(id: Uuid, db: SharedDb) -> Result<impl Reply, Rejection> {
                let result = db.write().await.$delete(&id).ok();
                result.map_or_else(
                    | | Err(warp::reject::not_found()),
                    |_| Ok(with_status(warp::reply::reply(), StatusCode::NO_CONTENT)),
                )
            }

            pub async fn put(
                id: Uuid,
                db: SharedDb,
                json: $post_type,
            ) -> Result<impl Reply, Rejection> {
                let result = db.write().await.$update(&(Some(id), json).into()).ok();
                result.map_or_else(
                    | | Err(warp::reject::not_found()),
                    |r| Ok(warp::reply::json(&r)),
                )
            }
        }
    };
}

create_handler! (
    lyric,
    get_lyric_list,
    get_lyric,
    add_lyric_post,
    delete_lyric,
    update_lyric,
    lipl_types::LyricPost
);

create_handler! (
    playlist,
    get_playlist_list,
    get_playlist,
    add_playlist_post,
    delete_playlist,
    update_playlist,
    lipl_types::PlaylistPost
);
