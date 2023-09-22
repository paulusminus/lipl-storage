macro_rules! create_handler {
    ($name:ident, $list:ident, $summaries:ident, $item:ident, $delete:ident, $update:ident, $post_type:path, $posted_type:path) => {
        pub mod $name {
            use crate::error::RepoError;
            use crate::model::Query;
            use lipl_core::{LiplRepo, Uuid};
            use std::sync::Arc;
            use warp::http::status::StatusCode;
            use warp::reply::{json, with_status};
            use warp::{Rejection, Reply};

            pub async fn list_summary(repo: Arc<dyn LiplRepo>) -> Result<impl Reply, Rejection> {
                let data = repo.$summaries().await.map_err(reject)?;
                Ok(json(&data))
            }

            pub async fn list(
                repo: Arc<dyn LiplRepo>,
                query: Query,
            ) -> Result<impl Reply, Rejection> {
                if query.full {
                    let data = repo.$list().await.map_err(reject)?;
                    Ok(json(&data))
                } else {
                    Err(warp::reject::not_found())
                }
            }

            fn reject<E: Into<RepoError>>(e: E) -> Rejection {
                warp::reject::custom::<RepoError>(e.into())
            }

            pub async fn item(
                id: String,
                repo: Arc<dyn LiplRepo>,
            ) -> Result<impl Reply, Rejection> {
                let uuid = id.parse::<Uuid>().map_err(reject)?;
                let data = repo.$item(uuid).await.map_err(reject)?;
                Ok(json(&data))
            }

            pub async fn post(
                repo: Arc<dyn LiplRepo>,
                object: $post_type,
            ) -> Result<impl Reply, Rejection> {
                let o: $posted_type = (None, object).into();
                let data = repo.$update(o).await.map_err(reject)?;
                Ok(with_status(json(&data), StatusCode::CREATED))
            }

            pub async fn delete(
                id: String,
                repo: Arc<dyn LiplRepo>,
            ) -> Result<impl Reply, Rejection> {
                let uuid = id.parse::<Uuid>().map_err(reject)?;
                repo.$delete(uuid).await.map_err(reject)?;
                Ok(with_status(warp::reply::reply(), StatusCode::NO_CONTENT))
            }

            pub async fn put(
                id: String,
                repo: Arc<dyn LiplRepo>,
                object: $post_type,
            ) -> Result<impl Reply, Rejection> {
                let uuid = id.parse::<Uuid>().map_err(reject)?;
                let o: $posted_type = (Some(uuid), object).into();
                let data = repo.$update(o).await.map_err(reject)?;
                Ok(json(&data))
            }
        }
    };
}

create_handler!(
    lyric,
    get_lyrics,
    get_lyric_summaries,
    get_lyric,
    delete_lyric,
    upsert_lyric,
    lipl_core::LyricPost,
    lipl_core::Lyric
);

create_handler!(
    playlist,
    get_playlists,
    get_playlist_summaries,
    get_playlist,
    delete_playlist,
    upsert_playlist,
    lipl_core::PlaylistPost,
    lipl_core::Playlist
);
