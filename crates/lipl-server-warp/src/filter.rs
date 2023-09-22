use crate::constant::{API, VERSION};
use crate::handler::lyric as lyric_handler;
use crate::handler::playlist as playlist_handler;
use lipl_core::LiplRepo;
use std::sync::Arc;
use warp::filters::query;
use warp::{body, path, Filter};

macro_rules! join_paths {
    ($head:expr, $($rest:expr),*) => { warp::path($head)$(.and(warp::path($rest)))* };
}

macro_rules! or {
    ($head:expr, $($rest:expr),*) => { $head$(.or($rest))* };
}

macro_rules! and {
    ($head:expr, $($rest:expr),*) => { $head$(.and($rest))* };
}

macro_rules! create_fn {
    ($name:ident, $handler:ident) => {
        pub fn $name(
            repo: Arc<dyn LiplRepo>,
            name: &'static str,
        ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
            let repo_filter = warp::any().map(move || repo.clone());
            let prefix = join_paths!(API, VERSION, name);

            let list = and!(
                warp::get(),
                prefix,
                path::end(),
                repo_filter.clone(),
                query::query()
            )
            .and_then($handler::list);
            let summaries = and!(warp::get(), prefix, path::end(), repo_filter.clone())
                .and_then($handler::list_summary);
            let item = and!(warp::get(), prefix, path::param(), repo_filter.clone())
                .and_then($handler::item);
            let post = and!(
                warp::post(),
                prefix,
                path::end(),
                repo_filter.clone(),
                body::json()
            )
            .and_then($handler::post);
            let put = and!(
                warp::put(),
                prefix,
                path::param(),
                repo_filter.clone(),
                body::json()
            )
            .and_then($handler::put);
            let delete = and!(warp::delete(), prefix, path::param(), repo_filter.clone())
                .and_then($handler::delete);

            or!(list, summaries, item, post, put, delete)
        }
    };
}

create_fn!(get_lyric_routes, lyric_handler);
create_fn!(get_playlist_routes, playlist_handler);
