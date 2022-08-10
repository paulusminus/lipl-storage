

macro_rules! create_handler {
    ($name:ident, $list:ident, $summaries:ident, $item:ident, $delete:ident, $update:ident, $post_type:path, $posted_type:path) => {
        pub mod $name {
            use lipl_types::{LiplRepo, Uuid};
            use warp::{Reply, Rejection};
            use warp::reply::with_status;
            use warp::http::status::StatusCode;
            use crate::model::{Query};

            pub async fn list_summary<D, E>(db: D) -> Result<impl Reply, Rejection> 
            where D: LiplRepo<E>, E: std::error::Error
            {
                db.$summaries().await
                .map(|result| warp::reply::json(&result))
                .map_err(|_| warp::reject::not_found())
            }

            pub async fn list<D, E>(db: D, query: Query) -> Result<impl Reply, Rejection>
            where D: LiplRepo<E>, E: std::error::Error
            {
                if query.full {
                    db.$list().await
                    .map(|result| warp::reply::json(&result))
                    .map_err(|_| warp::reject::not_found())
                } else {
                    Err(warp::reject::not_found())
                }
            }

            pub async fn item<D, E>(id: Uuid, db: D) -> Result<impl Reply, Rejection>
            where D: LiplRepo<E>, E: std::error::Error
            {
                db.$item(id).await
                .map(|r| warp::reply::json(&r))
                .map_err(|_| warp::reject::not_found())
            }

            pub async fn post<D, E>(
                db: D,
                json: $post_type,
            ) -> Result<impl Reply, Rejection>
            where D: LiplRepo<E>, E: std::error::Error
            {
                let o: $posted_type = (None, json).into();
                db.$update(o).await
                .map(|result| with_status(warp::reply::json(&result), StatusCode::CREATED))
                .map_err(|_| warp::reject::custom(crate::error::PostError {}))
            }

            pub async fn delete<D, E>(id: Uuid, db: D) -> Result<impl Reply, Rejection>
            where D: LiplRepo<E>, E: std::error::Error
            {
                db.$delete(id).await
                .map(|_| with_status(warp::reply::reply(), StatusCode::NO_CONTENT))
                .map_err(|_| warp::reject::not_found())
            }

            pub async fn put<D, E>(
                id: Uuid,
                db: D,
                json: $post_type,
            ) -> Result<impl Reply, Rejection>
            where D: LiplRepo<E>, E: std::error::Error
            {
                let o: $posted_type = (Some(id), json).into();
                db.$update(o).await
                .map(|result| warp::reply::json(&result))
                .map_err(|_| warp::reject::not_found())
            }
        }
    };
}

create_handler! (
    lyric,
    get_lyrics,
    get_lyric_summaries,
    get_lyric,
    delete_lyric,
    post_lyric,
    lipl_types::LyricPost,
    lipl_types::Lyric
);

create_handler! (
    playlist,
    get_playlists,
    get_playlist_summaries,
    get_playlist,
    delete_playlist,
    post_playlist,
    lipl_types::PlaylistPost,
    lipl_types::Playlist
);
