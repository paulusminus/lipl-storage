
macro_rules! create_handler {
    ($name:ident, $list:ident, $item:ident, $add:ident, $delete:ident, $update:ident, $post_type:path) => {
        pub mod $name {
            use std::sync::{Arc, RwLock};
            use lipl_io::model::{Db, HasSummaries, Uuid};
            use warp::{Reply, Rejection};
            use warp::reply::with_status;
            use warp::http::status::StatusCode;
            use crate::model::{Query};

            pub async fn list_summary(db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> {
                let result = db.read().unwrap().$list().to_summaries();
                Ok(warp::reply::json(&result))
            }

            pub async fn list(db: Arc<RwLock<Db>>, query: Query) -> Result<impl Reply, Rejection> {
                if query.full {
                    let result = db.read().unwrap().$list();
                    Ok(warp::reply::json(&result))
                } else {
                    Err(warp::reject::not_found())
                }
            }

            pub async fn item(id: Uuid, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> {
                let result = db.read().unwrap().$item(&id);
                result.map_or_else(
                    | | Err(warp::reject::not_found()),
                    |r| Ok(warp::reply::json(&r)),
                )
            }

            pub async fn post(
                db: Arc<RwLock<Db>>,
                json: $post_type,
            ) -> Result<impl Reply, Rejection> {
                let result = db.write().unwrap().$add(&json);
                Ok(with_status(warp::reply::json(&result), StatusCode::CREATED))
            }

            pub async fn delete(id: Uuid, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> {
                let result = db.write().unwrap().$delete(&id).ok();
                result.map_or_else(
                    | | Err(warp::reject::not_found()),
                    |_| Ok(with_status(warp::reply::reply(), StatusCode::NO_CONTENT)),
                )
            }

            pub async fn put(
                id: Uuid,
                db: Arc<RwLock<Db>>,
                json: $post_type,
            ) -> Result<impl Reply, Rejection> {
                let result = db.write().unwrap().$update(&(Some(id), json).into()).ok();
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
    lipl_io::model::LyricPost
);

create_handler! (
    playlist,
    get_playlist_list,
    get_playlist,
    add_playlist_post,
    delete_playlist,
    update_playlist,
    lipl_io::model::PlaylistPost
);
