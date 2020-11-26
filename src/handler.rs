use std::collections::HashMap;
use std::sync::RwLock;
use std::path::PathBuf;
use warp::{Reply, Rejection};
use lipl_io::{HasSummary, HasId, PathBufExt, Summary, Uuid, Serialize};
use std::sync::Arc;

type Db<T> = Arc<RwLock<HashMap<Uuid, T>>>;

pub async fn list<T>(db: Db<T>) -> Result<impl Reply, Rejection> 
where T: HasSummary {
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

pub async fn item<T>(path: String, db: Db<T>) -> Result<impl Reply, Rejection>
where T: Serialize + Clone {
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

pub async fn post<T, U>(json: U, db: Db<T>) -> Result<impl Reply, Rejection> 
where T: From<U> + Clone + Serialize + HasId {
    let t: T = json.into();
    {
        db.write()
        .unwrap()
        .insert(t.id().clone(), t.clone());
    }
    Ok(warp::reply::json(&t))
}

pub async fn delete<T>(path: String, db: Db<T>) -> Result<impl Reply, Rejection> {
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

pub async fn put<T, U>(path: String, json: U, db: Db<T>) -> Result<impl Reply, Rejection> 
where T: From<U> {
    let db_result = {
        PathBuf::from(&path).try_to_uuid().ok()
        .and_then(|uuid| {
            db.write()
            .unwrap()
            .get_mut(&uuid)
            .map(|e| { *e = T::from(json) })
        })
    };
    db_result.map_or_else(
        | | Err(warp::reject::not_found()),
        |_| Ok(warp::reply::reply()),
    )
}
