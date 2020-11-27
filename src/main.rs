#[macro_use]
extern crate log;

mod filter;
mod handler;
mod param;

use warp::Filter;
use lipl_io::model::{create_db, LyricPost, PlaylistPost, Lyric, Playlist};
use filter::get_routes;

const PORT: u16 = 3030;
const HOST: [u8; 4] = [0, 0, 0, 0];
const LYRIC: &str = "lyric";
const PLAYLIST: &str = "playlist";

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting up");

    let source_path         = param::parse_command_line()?;
    let (lyrics, playlists) = create_db(&source_path).await?;

    let routes = 
        get_routes::<Lyric, LyricPost>(lyrics, LYRIC)
        .or(
            get_routes::<Playlist, PlaylistPost>(playlists, PLAYLIST)
        )
        .with(warp::log("request"));

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
