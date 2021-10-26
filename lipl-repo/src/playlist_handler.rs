use std::sync::{Arc, RwLock};
use warp::{Reply, Rejection};
use warp::reply::{with_status};
use warp::http::status::StatusCode;
use crate::model::Query;

use lipl_io::model::{Db, HasSummary, Uuid, Playlist, PlaylistPost, Summary};

pub async fn list_summary(db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> 
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

pub async fn list(db: Arc<RwLock<Db>>, query: Query) -> Result<impl Reply, Rejection> 
{
    if query.full {
        let db_result: Vec<Playlist> = {
            let read = db.read().unwrap();
            read.get_playlist_list().into_iter().cloned().collect()
        };
        Ok(
            warp::reply::json(
                &db_result
            )
        )    
    }
    else {
        Err(warp::reject::not_found())
    }
}

pub async fn item(id: Uuid, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection>
{
    let db_result = {
        db.read()
        .unwrap()
        .get_playlist(&id)
        .cloned()
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
    Ok(with_status(warp::reply::json(&result), StatusCode::CREATED))
}

pub async fn delete(id: Uuid, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> {
    let db_result = {
        db.write()
        .unwrap()
        .delete_playlist(&id)
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |_| Ok(with_status(warp::reply::reply(), StatusCode::NO_CONTENT)),
    )
}

pub async fn put(id: Uuid, json: PlaylistPost, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> 
{
    let db_result = {
        let playlist: Playlist = (Some(id), json).into();
        db.write()
        .unwrap()
        .update_playlist(&playlist)
        .ok()
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |p| Ok(warp::reply::json(&p)),
    )
}
