use std::sync::{Arc, RwLock};
use warp::{body, path, Filter};
use lipl_io::{LyricPost, PlaylistPost};

mod handler;
mod param;

const VERSION: &str = "v1";
const LYRIC: &str = "lyric";
const PLAYLIST: &str = "playlist";
const PORT: u16 = 3030;
const HOST: [u8; 4] = [0, 0, 0, 0];

#[tokio::main]
async fn main() -> tokio::io::Result<()> {

    let source_path         = param::parse_command_line()?;
    let (lyrics, playlists) = lipl_io::create_db(&source_path).await?;
    let lyric_arc           = Arc::new(RwLock::new(lyrics));
    let playlist_arc        = Arc::new(RwLock::new(playlists));
    let lyric_db            = warp::any().map(move || lyric_arc.clone());
    let playlist_db         = warp::any().map(move || playlist_arc.clone());

    let lyric_list = 
        warp::get()
        .and(path(VERSION))
        .and(path(LYRIC))
        .and(path::end())
        .and(lyric_db.clone())
        .and_then(handler::list);

    let lyric_item = 
        warp::get()
        .and(path(VERSION))
        .and(path(LYRIC))
        .and(path::param())
        .and(lyric_db.clone())
        .and_then(handler::item);

    let lyric_post = 
        warp::post()
        .and(path(VERSION))
        .and(path(LYRIC))
        .and(path::end())
        .and(body::json::<LyricPost>())
        .and(lyric_db.clone())
        .and_then(handler::post);

    let lyric_delete = 
        warp::delete()
        .and(warp::path(VERSION))
        .and(warp::path(LYRIC))
        .and(warp::path::param())
        .and(lyric_db.clone())
        .and_then(handler::delete);

    let lyric_put = 
        warp::put()
        .and(path(VERSION))
        .and(path(LYRIC))
        .and(path::param())
        .and(body::json::<LyricPost>())
        .and(lyric_db.clone())
        .and_then(handler::put);

    let playlist_list = 
        warp::get()
        .and(path(VERSION))
        .and(path(PLAYLIST))
        .and(path::end())
        .and(playlist_db.clone())
        .and_then(handler::list);

    let playlist_item = 
        warp::get()
        .and(path(VERSION))
        .and(path(PLAYLIST))
        .and(path::param())
        .and(playlist_db.clone())
        .and_then(handler::item);

    let playlist_post = 
        warp::post()
        .and(path(VERSION))
        .and(path(PLAYLIST))
        .and(path::end())
        .and(body::json::<PlaylistPost>())
        .and(playlist_db.clone())
        .and_then(handler::post);

    let playlist_delete = 
        warp::delete()
        .and(path(VERSION))
        .and(path(PLAYLIST))
        .and(path::param())
        .and(playlist_db.clone())
        .and_then(handler::delete);

    let playlist_put = 
        warp::put()
        .and(path(VERSION))
        .and(path(PLAYLIST))
        .and(path::param())
        .and(body::json::<PlaylistPost>())
        .and(playlist_db.clone())
        .and_then(handler::put);

    let routes = 
        lyric_list
        .or(lyric_item)
        .or(lyric_post)
        .or(lyric_delete)
        .or(lyric_put)
        .or(playlist_list)
        .or(playlist_item)
        .or(playlist_post)
        .or(playlist_delete)
        .or(playlist_put);

    warp::serve(routes)
        .run((HOST, PORT))
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
