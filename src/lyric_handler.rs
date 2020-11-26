use std::collections::HashMap;
// use std::collections::HashMap;
use std::sync::RwLock;
use std::path::PathBuf;
use warp::{Reply, Rejection};
use lipl_io::{Lyric, LyricPost, PathBufExt, Summary, Uuid};
use std::sync::Arc;

type Db<T> = Arc<RwLock<HashMap<Uuid, T>>>;

pub async fn get_lyric_list(db: Db<Lyric>) -> Result<impl Reply, Rejection> {
    let db_result = {
        let read = db.read().unwrap();
        read.values().map(|l| l.to_summary()).collect::<Vec<Summary>>()
    };
    Ok(
        warp::reply::json(
            &db_result
        )
    )
}

pub async fn get_lyric(path: String, db: Db<Lyric>) -> Result<impl Reply, Rejection> {
    let db_result = {
        PathBuf::from(&path).try_to_uuid().ok()
        .and_then(|uuid| {
            db.read()
            .unwrap()
            .get(&uuid)
            .map(|l| l.clone())    
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |l| Ok(warp::reply::json(&l)),
    )
}

pub async fn post_lyric(lyric_post: LyricPost, db: Db<Lyric>) -> Result<impl Reply, Rejection> {
    let lyric: Lyric = lyric_post.into();
    {
        db.write()
        .unwrap()
        .insert(lyric.id.clone(), lyric.clone());
    }
    Ok(warp::reply::json(&lyric))
}

pub async fn delete_lyric(path: String, db: Db<Lyric>) -> Result<impl Reply, Rejection> {
    let db_result = {
        PathBuf::from(&path).try_to_uuid().ok()
        .and_then(|uuid| {
            db.write()
            .unwrap()
            .remove(&uuid)
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |_| Ok(warp::reply::reply()),
    )
}

pub async fn put_lyric(path: String, lyric_put: LyricPost, db: Db<Lyric>) -> Result<impl Reply, Rejection> {
    let db_result = {
        PathBuf::from(&path).try_to_uuid().ok()
        .and_then(|uuid| {
            db.write()
            .unwrap()
            .get_mut(&uuid)
            .map(|e| { *e = Lyric::from(lyric_put) })
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |_| Ok(warp::reply::reply()),
    )
}
