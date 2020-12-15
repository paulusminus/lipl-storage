#[macro_use]
extern crate log;

mod constant;
mod lyric_filter;
mod lyric_handler;
mod message;
mod param;
mod playlist_filter;
mod playlist_handler;

use std::sync::{Arc, RwLock};
use anyhow::Result;
use tokio::sync::oneshot;
use tokio::signal;
use warp::Filter;

use lipl_io::io::fs_read;

use lyric_filter::get_routes as get_lyric_routes;
use playlist_filter::get_routes as get_playlist_routes;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = oneshot::channel::<()>();
    let signals = signal::ctrl_c();
    
    tokio::task::spawn(async move {
        signals.await
        .map(|_| tx.send(()))
    });

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(constant::LOG_LEVEL)).init();
    info!("{}", message::STARTING);

    let source_path = param::parse_command_line()?;
    let db          = Arc::new(RwLock::new(fs_read(&source_path)?));

    let routes = 
        get_lyric_routes(db.clone(), constant::LYRIC)
        .or(
            get_playlist_routes(db.clone(), constant::PLAYLIST)
        )
        .with(warp::log(constant::LOG_NAME));

    let (_address, server) = 
        warp::serve(routes)
        .try_bind_with_graceful_shutdown((constant::HOST, constant::PORT), async {
            rx.await.ok();
            info!("{}", message::STOPPING);
        })?;

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
