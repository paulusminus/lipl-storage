use std::sync::RwLock;
use std::path::PathBuf;
use warp::{Reply, Rejection};
use lipl_io::{Db, Lyric, PathBufExt, Summary, Uuid};
use std::sync::Arc;

pub async fn get_lyric_list(db: Arc<RwLock<Db<Lyric>>>) -> Result<impl Reply, Rejection> {
    let read = db.read().unwrap();
    let result = read.values().map(|l| l.to_summary()).collect::<Vec<Summary>>();
    Ok(
        warp::reply::json(
            &result
        )
    )
}

pub async fn get_lyric(path: String, db: Arc<RwLock<Db<Lyric>>>) -> Result<impl Reply, Rejection> {
    let read = db.read().unwrap();
    read.get(&PathBuf::from(&path).to_uuid())
    .map_or_else(
        |     | Err(warp::reject::not_found()),
        |lyric| Ok(warp::reply::json(&*lyric)),
    )
}

pub async fn post_lyric(lyric: Lyric, db: Arc<RwLock<Db<Lyric>>>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&Vec::<String>::from([])))
}
