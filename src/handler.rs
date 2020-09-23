use warp::{Reply, Rejection};
use super::model::{Store, Lyric};

fn lyric_id_from_path(path: String) -> i32 {
    path.parse::<i32>()
    .unwrap_or_default()
}

pub async fn get_lyric_list(store: Store) -> Result<impl Reply, Rejection> {
    Ok(
        warp::reply::json(
            &store.get_summaries()
        )
    )
}

pub async fn get_lyric(path: String, store: Store) -> Result<impl Reply, Rejection> {
    store
    .get_lyric(
        lyric_id_from_path(path)
    )
    .map_or_else(
        |     | Err(warp::reject::not_found()),
        |lyric| Ok(warp::reply::json(&lyric)),
    )
}

pub async fn post_lyric(lyric: Lyric, store: Store) -> Result<impl Reply, Rejection> {
    store.add_lyric(lyric.parts, lyric.title)
    .map_or_else(
        |     | Err(warp::reject::reject()),
        |lyric| Ok(warp::reply::json(&lyric)),
    )
}
