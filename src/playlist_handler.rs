use std::collections::HashMap;
use std::sync::RwLock;
use std::path::PathBuf;
use warp::{Reply, Rejection};
use lipl_io::{Playlist, PlaylistPost, PathBufExt, Summary, Uuid};
use std::sync::Arc;

type Db<T> = Arc<RwLock<HashMap<Uuid, T>>>;

pub async fn get_playlist_list(db: Db<Playlist>) -> Result<impl Reply, Rejection> {
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

pub async fn get_playlist(path: String, db: Db<Playlist>) -> Result<impl Reply, Rejection> {
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

pub async fn post_playlist(playlist_post: PlaylistPost, db: Db<Playlist>) -> Result<impl Reply, Rejection> {
    let playlist: Playlist = playlist_post.into();
    {
        db.write()
        .unwrap()
        .insert(playlist.id.clone(), playlist.clone());
    }
    Ok(warp::reply::json(&playlist))
}

pub async fn delete_playlist(path: String, db: Db<Playlist>) -> Result<impl Reply, Rejection> {
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

pub async fn put_playlist(path: String, playlist_put: PlaylistPost, db: Db<Playlist>) -> Result<impl Reply, Rejection> {
    let db_result = {
        PathBuf::from(&path).try_to_uuid().ok()
        .and_then(|uuid| {
            db.write()
            .unwrap()
            .get_mut(&uuid)
            .map(|e| { *e = Playlist::from(playlist_put) })
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |_| Ok(warp::reply::reply()),
    )
}
