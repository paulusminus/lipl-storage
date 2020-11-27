use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use warp::{body, path, Filter};
use lipl_io::{HasId, HasSummary, LyricPost, PlaylistPost, Lyric, Playlist, Uuid, Deserialize, Serialize};

mod handler;
mod param;

const VERSION: &str = "v1";
const LYRIC: &str = "lyric";
const PLAYLIST: &str = "playlist";
const PORT: u16 = 3030;
const HOST: [u8; 4] = [0, 0, 0, 0];

fn get_routes<T, U>(db: HashMap<Uuid, T>, name: &'static str) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where T: From<U> + HasSummary + HasId + Serialize + Clone + Send + Sync,
U: for<'de> Deserialize<'de> + Send {
    let arc = Arc::new(RwLock::new(db));
    let db  = warp::any().map(move || arc.clone());

    let list = 
        warp::get()
        .and(path(VERSION))
        .and(path(name))
        .and(path::end())
        .and(db.clone())
        .and_then(handler::list);

    let item =
        warp::get()
        .and(path(VERSION))
        .and(path(name))
        .and(path::param())
        .and(db.clone())
        .and_then(handler::item);

    let post =
        warp::post()
        .and(path(VERSION))
        .and(path(name))
        .and(path::end())
        .and(body::json::<U>())
        .and(db.clone())
        .and_then(handler::post);

    let delete =
        warp::delete()
        .and(warp::path(VERSION))
        .and(warp::path(name))
        .and(warp::path::param())
        .and(db.clone())
        .and_then(handler::delete);

    let put =
        warp::put()
        .and(path(VERSION))
        .and(path(name))
        .and(path::param())
        .and(body::json::<U>())
        .and(db.clone())
        .and_then(handler::put);

    list.or(item).or(post).or(put).or(delete)
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {

    let source_path         = param::parse_command_line()?;
    let (lyrics, playlists) = lipl_io::create_db(&source_path).await?;

    let lyric_routes = get_routes::<Lyric, LyricPost>(lyrics, LYRIC);
    let playlist_routes = get_routes::<Playlist, PlaylistPost>(playlists, PLAYLIST);
    let routes = lyric_routes.or(playlist_routes);

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
