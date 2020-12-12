#[macro_use]
extern crate log;

mod constant;
mod filter;
mod handler;
mod message;
mod param;

use warp::Filter;
use tokio::sync::oneshot;
use tokio::signal;
use lipl_io::model::{create_db, LyricPost, PlaylistPost, Lyric, Playlist};
use filter::get_routes;


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let (tx, rx) = oneshot::channel::<()>();
    let signals = signal::ctrl_c();
    
    tokio::task::spawn(async move {
        signals.await
        .map(|_| tx.send(()))
    });

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(constant::LOG_LEVEL)).init();
    info!("{}", message::STARTING);

    let source_path         = param::parse_command_line()?;
    let (lyrics, playlists) = create_db(&source_path)?;

    let routes = 
        get_routes::<Lyric, LyricPost>(lyrics, constant::LYRIC)
        .or(
            get_routes::<Playlist, PlaylistPost>(playlists, constant::PLAYLIST)
        )
        .with(warp::log(constant::LOG_NAME));

    let (_address, server) = 
        warp::serve(routes)
        .bind_with_graceful_shutdown((constant::HOST, constant::PORT), async {
            rx.await.ok();
            info!("{}", message::STOPPING);
        });

    server.await;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
