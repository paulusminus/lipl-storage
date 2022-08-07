use anyhow::Result;
use lipl_types::LiplRepo;
use tokio::sync::oneshot;
use tokio::signal;
use warp::Filter;

use crate::constant;
use crate::message;
use crate::param;
use crate::filter::{get_lyric_routes, get_playlist_routes};
use crate::param::DbType;
use crate::param::get_file_repo;
use crate::param::get_postgres_repo;

async fn run(repo: impl LiplRepo + 'static, port: u16) -> Result<()> {
    let (tx, rx) = oneshot::channel::<()>();
    let signals = signal::ctrl_c();
    
    tokio::task::spawn(async move {
        signals.await
        .map(|_| tx.send(()))
    });

    let routes = 
        get_lyric_routes(repo.clone(), constant::LYRIC)
        .or(
            get_playlist_routes(repo.clone(), constant::PLAYLIST)
        )
        .with(warp::log(constant::LOG_NAME));

    let (_address, server) = 
        warp::serve(routes)
        .try_bind_with_graceful_shutdown((constant::HOST, port), async move {
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

pub async fn serve(param: param::Serve) -> Result<()> {

    match param.source.parse::<DbType>()? {
        DbType::File(s) => {
            run(get_file_repo(s)?, param.port).await?;

        },
        DbType::Postgres(s) => {
            run(get_postgres_repo(s).await?, param.port).await?;
        }
    }
    Ok(())

}