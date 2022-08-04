use lipl_fs_repo::FileRepo;

use anyhow::Result;
use tokio::sync::oneshot;
use tokio::signal;
use warp::Filter;

use crate::constant;
use crate::message;
use crate::param;
use crate::filter::{get_lyric_routes, get_playlist_routes};

pub async fn serve(param: param::Serve) -> Result<()> {
    let (tx, rx) = oneshot::channel::<()>();
    let signals = signal::ctrl_c();
    
    tokio::task::spawn(async move {
        signals.await
        .map(|_| tx.send(()))
    });

    let repo = FileRepo::new(
        param.source.to_string_lossy().to_string(),
        "yaml".to_owned(),
        "txt".to_owned(),
    )?;

    let routes = 
        get_lyric_routes(repo.clone(), constant::LYRIC)
        .or(
            get_playlist_routes(repo.clone(), constant::PLAYLIST)
        )
        .with(warp::log(constant::LOG_NAME));

    let (_address, server) = 
        warp::serve(routes)
        .try_bind_with_graceful_shutdown((constant::HOST, param.port), async move {
            rx.await.ok();
            info!("{}", message::STOPPING);
            if let Err(error) = repo.stop().await {
                error!("{}", error);
            };
            info!("{}", message::BACKUP_COMPLETE);
        })?;

    server.await;

    Ok(())
}