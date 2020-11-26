use std::sync::{Arc, RwLock};
use warp::{Filter};
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

    let path = param::parse_command_line()?;
    let (lyrics, playlists) = lipl_io::create_db(&path).await?;
    let lyric_arc = Arc::new(RwLock::new(lyrics));
    let playlist_arc = Arc::new(RwLock::new(playlists));
    let lyric_db_filter = warp::any().map(move || lyric_arc.clone());
    let playlist_db_filter = warp::any().map(move || playlist_arc.clone());

    let get_lyrics = 
        warp::get()
        .and(warp::path(VERSION))
        .and(warp::path(LYRIC))
        .and(warp::path::end())
        .and(lyric_db_filter.clone())
        .and_then(handler::list);

    let get_lyric = 
        warp::get()
        .and(warp::path(VERSION))
        .and(warp::path(LYRIC))
        .and(warp::path::param())
        .and(lyric_db_filter.clone())
        .and_then(handler::item);

    let post_lyric = 
        warp::post()
        .and(warp::path(VERSION))
        .and(warp::path(LYRIC))
        .and(warp::path::end())
        .and(warp::body::json::<LyricPost>())
        .and(lyric_db_filter.clone())
        .and_then(handler::post);

    let delete_lyric = 
        warp::delete()
        .and(warp::path(VERSION))
        .and(warp::path(LYRIC))
        .and(warp::path::param())
        .and(lyric_db_filter.clone())
        .and_then(handler::delete);

    let put_lyric = 
        warp::put()
        .and(warp::path(VERSION))
        .and(warp::path(LYRIC))
        .and(warp::path::param())
        .and(warp::body::json::<LyricPost>())
        .and(lyric_db_filter.clone())
        .and_then(handler::put);

    let get_playlists = 
        warp::get()
        .and(warp::path(VERSION))
        .and(warp::path(PLAYLIST))
        .and(warp::path::end())
        .and(playlist_db_filter.clone())
        .and_then(handler::list);

    let get_playlist = 
        warp::get()
        .and(warp::path(VERSION))
        .and(warp::path(PLAYLIST))
        .and(warp::path::param())
        .and(playlist_db_filter.clone())
        .and_then(handler::item);

    let post_playlist = 
        warp::post()
        .and(warp::path(VERSION))
        .and(warp::path(PLAYLIST))
        .and(warp::path::end())
        .and(warp::body::json::<PlaylistPost>())
        .and(playlist_db_filter.clone())
        .and_then(handler::post);

    let delete_playlist = 
        warp::delete()
        .and(warp::path(VERSION))
        .and(warp::path(PLAYLIST))
        .and(warp::path::param())
        .and(playlist_db_filter.clone())
        .and_then(handler::delete);

    let put_playlist = 
        warp::put()
        .and(warp::path(VERSION))
        .and(warp::path(PLAYLIST))
        .and(warp::path::param())
        .and(warp::body::json::<PlaylistPost>())
        .and(playlist_db_filter.clone())
        .and_then(handler::put);

    let routes = 
        get_lyrics
        .or(get_lyric)
        .or(post_lyric)
        .or(delete_lyric)
        .or(put_lyric)
        .or(get_playlists)
        .or(get_playlist)
        .or(post_playlist)
        .or(delete_playlist)
        .or(put_playlist);

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
