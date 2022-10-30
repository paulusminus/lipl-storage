

macro_rules! create_handler {
    ($name:ident, $list:ident, $summaries:ident, $item:ident, $delete:ident, $update:ident, $post_type:path, $posted_type:path) => {
        pub mod $name {
            use lipl_core::{LiplRepo, Uuid};
            use warp::{Reply, Rejection};
            use warp::reply::{json, with_status};
            use warp::http::status::StatusCode;
            use crate::model::{Query};
            use crate::error::{RepoError};

            pub async fn list_summary<D, E>(db: D) -> Result<impl Reply, Rejection> 
            where D: LiplRepo<Error = E>, E: Into<RepoError>
            {
                let data = db.$summaries().await.map_err(reject)?;
                Ok(json(&data))
            }

            pub async fn list<D, E>(db: D, query: Query) -> Result<impl Reply, Rejection>
            where D: LiplRepo<Error = E>, E: Into<RepoError>
            {
                if query.full {
                    let data = db.$list().await.map_err(reject)?;
                    Ok(json(&data))
                } else {
                    Err(warp::reject::not_found())
                }
            }

            fn reject<E: Into<RepoError>>(e: E) -> Rejection {
                warp::reject::custom::<RepoError>(e.into())
            }

            pub async fn item<D, E>(id: String, db: D) -> Result<impl Reply, Rejection>
            where D: LiplRepo<Error = E>, E: Into<RepoError>
            {
                let uuid = id.parse::<Uuid>().map_err(reject)?;
                let data = db.$item(uuid).await.map_err(reject)?;
                Ok(json(&data))
            }

            pub async fn post<D, E>(
                db: D,
                object: $post_type,
            ) -> Result<impl Reply, Rejection>
            where D: LiplRepo<Error = E>, E: Into<RepoError>
            {
                let o: $posted_type = (None, object).into();
                let data = db.$update(o).await.map_err(reject)?;
                Ok(with_status(json(&data), StatusCode::CREATED))
            }

            pub async fn delete<D, E>(id: String, db: D) -> Result<impl Reply, Rejection>
            where D: LiplRepo<Error = E>, E: Into<RepoError>
            {
                let uuid = id.parse::<Uuid>().map_err(reject)?;
                db.$delete(uuid).await.map_err(reject)?;
                Ok(with_status(warp::reply::reply(), StatusCode::NO_CONTENT))
            }

            pub async fn put<D, E>(
                id: String,
                db: D,
                object: $post_type,
            ) -> Result<impl Reply, Rejection>
            where D: LiplRepo<Error = E>, E: Into<RepoError>
            {
                let uuid = id.parse::<Uuid>().map_err(reject)?;
                let o: $posted_type = (Some(uuid), object).into();
                let data = db.$update(o).await.map_err(reject)?;
                Ok(json(&data))
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
    lipl_core::LyricPost,
    lipl_core::Lyric
);

create_handler! (
    playlist,
    get_playlists,
    get_playlist_summaries,
    get_playlist,
    delete_playlist,
    post_playlist,
    lipl_core::PlaylistPost,
    lipl_core::Playlist
);
