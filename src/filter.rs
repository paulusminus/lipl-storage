use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use warp::{body, path, Filter};
use lipl_io::{Deserialize, Serialize};
use lipl_io::model::{HasId, HasSummary, Uuid};
use crate::handler;
use crate::constant::{API, VERSION};

pub fn get_routes<T, U>(db: HashMap<Uuid, T>, name: &'static str) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where T: From<U> + HasSummary + HasId + Serialize + Clone + Send + Sync,
U: for<'de> Deserialize<'de> + Send
{
    let arc = Arc::new(RwLock::new(db));
    let db  = warp::any().map(move || arc.clone());
    let prefix = warp::path(API).and(warp::path(VERSION));

    let list = 
        warp::get()
        .and(prefix)
        .and(path(name))
        .and(path::end())
        .and(db.clone())
        .and_then(handler::list);

    let item =
        warp::get()
        .and(prefix)
        .and(path(name))
        .and(path::param())
        .and(db.clone())
        .and_then(handler::item);

    let post =
        warp::post()
        .and(prefix)
        .and(path(name))
        .and(path::end())
        .and(body::json::<U>())
        .and(db.clone())
        .and_then(handler::post);

    let delete =
        warp::delete()
        .and(prefix)
        .and(warp::path(name))
        .and(warp::path::param())
        .and(db.clone())
        .and_then(handler::delete);

    let put =
        warp::put()
        .and(prefix)
        .and(path(name))
        .and(path::param())
        .and(body::json::<U>())
        .and(db.clone())
        .and_then(handler::put);

    list.or(item).or(post).or(put).or(delete)
}
