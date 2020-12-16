use std::sync::{Arc, RwLock};

use anyhow::Result;
use tokio::sync::oneshot;
use tokio::signal;
use warp::Filter;

use crate::constant;
use crate::message;
use crate::param;
use crate::lyric_filter::get_routes as get_lyric_routes;
use crate::playlist_filter::get_routes as get_playlist_routes;

use lipl_io::io::fs_read;

pub async fn serve(param: param::Serve) -> Result<()> {
    let (tx, rx) = oneshot::channel::<()>();
    let signals = signal::ctrl_c();
    
    tokio::task::spawn(async move {
        signals.await
        .map(|_| tx.send(()))
    });

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(constant::LOG_LEVEL)).init();
    info!("{}", message::STARTING);

    let db = Arc::new(RwLock::new(fs_read(&param.source)?));

    let routes = 
        get_lyric_routes(db.clone(), constant::LYRIC)
        .or(
            get_playlist_routes(db.clone(), constant::PLAYLIST)
        )
        .with(warp::log(constant::LOG_NAME));

    let (_address, server) = 
        warp::serve(routes)
        .try_bind_with_graceful_shutdown((constant::HOST, param.port), async {
            rx.await.ok();
            info!("{}", message::STOPPING);
        })?;

    server.await;

    Ok(())
}