use std::sync::{Arc, RwLock};
use warp::{Filter};

// mod model;
mod handler;
mod param;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {

    let path = param::parse_command_line()?;
    let (lyrics, _playlists) = lipl_io::create_db(&path).await?;
    let lyric_arc = Arc::new(RwLock::new(lyrics));
    // let playlist_arc = Arc::new(playlists);
    let lyric_db_filter = warp::any().map(move || lyric_arc.clone());
    // let playlist_db_filter = warp::any().map(move || playlist_arc.clone());

    let get_items = 
        warp::get()
        .and(warp::path("v1"))
        .and(warp::path("lyric"))
        .and(warp::path::end())
        .and(lyric_db_filter.clone())
        .and_then(handler::get_lyric_list);

    let get_item = 
        warp::get()
        .and(warp::path("v1"))
        .and(warp::path("lyric"))
        .and(warp::path::param())
        .and(lyric_db_filter.clone())
        .and_then(handler::get_lyric);

    let post_item = 
        warp::post()
        .and(warp::path("v1"))
        .and(warp::path("lyric"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(lyric_db_filter.clone())
        .and_then(handler::post_lyric);

    let routes = 
        get_items
        .or(get_item)
        .or(post_item);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
