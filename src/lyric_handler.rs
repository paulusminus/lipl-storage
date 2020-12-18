use std::sync::{Arc, RwLock};
use warp::{Reply, Rejection};
use warp::reply::with_status;
use warp::http::status::StatusCode;
use lipl_io::model::{Db, HasSummary, Lyric, LyricPost, PathBufExt, Summary};
use crate::constant::{CREATED, NO_CONTENT};

pub async fn list(db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> 
{
    let db_result = {
        let read = db.read().unwrap();
        read.get_lyric_list().iter().map(|l| l.to_summary()).collect::<Vec<Summary>>()
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
            .get_lyric(&uuid)
            .cloned()
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |l| Ok(warp::reply::json(&l)),
    )
}

pub async fn post(json: LyricPost, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> 
{
    let result = {
        db.write()
        .unwrap()
        .add_lyric_post(json)
    };
    Ok(with_status(warp::reply::json(&result), StatusCode::from_u16(CREATED).unwrap()))
}

pub async fn delete(path: String, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> {
    let db_result = {
        path.try_to_uuid().ok()
        .map(|uuid| {
            db.write()
            .unwrap()
            .delete_lyric(&uuid)
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |_| Ok(with_status(warp::reply::reply(), StatusCode::from_u16(NO_CONTENT).unwrap())),
    )
}

pub async fn put(path: String, json: LyricPost, db: Arc<RwLock<Db>>) -> Result<impl Reply, Rejection> 
{
    let db_result = {
        path.try_to_uuid().ok()
        .map(|uuid| {
            let lyric: Lyric = (Some(uuid), json).into();
            db.write()
            .unwrap()
            .update_lyric(&lyric)
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |_| Ok(with_status(warp::reply::reply(), StatusCode::from_u16(NO_CONTENT).unwrap())),
    )
}
