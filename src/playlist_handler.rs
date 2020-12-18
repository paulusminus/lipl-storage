use std::sync::{Arc, RwLock};
use warp::{Reply, Rejection};
use warp::reply::{with_status};
use warp::http::status::StatusCode;

use lipl_io::model::{Db, HasSummary, Playlist, PlaylistPost, PathBufExt, Summary};
use crate::constant::{CREATED, NO_CONTENT};

pub async fn list(db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> 
{
    let db_result = {
        let read = db.read().unwrap();
        read.get_playlist_list().iter().map(|l| l.to_summary()).collect::<Vec<Summary>>()
    };
    Ok(
        warp::reply::json(
            &db_result
        )
    )
}

pub async fn item(path: String, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection>
{
    let db_result = {
        path.try_to_uuid().ok()
        .and_then(|uuid| {
            db.read()
            .unwrap()
            .get_playlist(&uuid)
            .cloned()
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |l| Ok(warp::reply::json(&l)),
    )
}

pub async fn post(json: PlaylistPost, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> 
{
    let result = {
        db.write()
        .unwrap()
        .add_playlist_post(&json)
    };
    Ok(with_status(warp::reply::json(&result), StatusCode::from_u16(CREATED).unwrap()))
}

pub async fn delete(path: String, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> {
    let db_result = {
        path.try_to_uuid().ok()
        .map(|uuid| {
            db.write()
            .unwrap()
            .delete_playlist(&uuid)
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |_| Ok(with_status(warp::reply::reply(), StatusCode::from_u16(NO_CONTENT).unwrap())),
    )
}

pub async fn put(path: String, json: PlaylistPost, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> 
{
    let db_result = {
        path.try_to_uuid().ok()
        .map(|uuid| {
            let playlist: Playlist = (Some(uuid), json).into();
            db.write()
            .unwrap()
            .update_playlist(&playlist)
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |p| Ok(warp::reply::json(&p)),
    )
}
