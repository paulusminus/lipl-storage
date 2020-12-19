use std::sync::{Arc, RwLock};
use warp::{body, path, Filter};
use lipl_io::model::{Db};
use crate::lyric_handler as handler;
use crate::constant::{API, VERSION};

pub fn get_routes(db: Arc<RwLock<Db>>, name: &'static str) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let db_filter  = warp::any().map(move || db.clone());
    let prefix     = warp::path(API).and(warp::path(VERSION));

    let list = 
        warp::get()
        .and(prefix)
        .and(path(name))
        .and(path::end())
        .and(db_filter.clone())
        .and_then(handler::list);

    let item =
        warp::get()
        .and(prefix)
        .and(path(name))
        .and(path::param())
        .and(db_filter.clone())
        .and_then(handler::item);

    let post =
        warp::post()
        .and(prefix)
        .and(path(name))
        .and(path::end())
        .and(body::json())
        .and(db_filter.clone())
        .and_then(handler::post);

    let delete =
        warp::delete()
        .and(prefix)
        .and(warp::path(name))
        .and(warp::path::param())
        .and(db_filter.clone())
        .and_then(handler::delete);

    let put =
        warp::put()
        .and(prefix)
        .and(path(name))
        .and(path::param())
        .and(body::json())
        .and(db_filter)
        .and_then(handler::put);

    list.or(item).or(post).or(put).or(delete)
}
