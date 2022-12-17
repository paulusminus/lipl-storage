use lipl_core::LiplRepo;
use tokio::signal;
use tracing::{info, error};
use warp::Filter;

use crate::constant;
use crate::message;
use crate::filter::{get_lyric_routes, get_playlist_routes};

pub async fn run<R>(repo: R, port: u16) -> anyhow::Result<()> 
where
    R: LiplRepo + 'static,
{
    let filter =
        std::env::var(constant::RUST_LOG)
        .unwrap_or_else(|_| constant::DEFAULT_LOG_FILTER.to_owned());
    
    tracing_subscriber::fmt()
    .with_env_filter(filter)
    .init();

    // Cache warmup
    let _lyrics = repo.get_lyrics().await;
    let _playlists = repo.get_playlists().await;

    let routes = 
        get_lyric_routes(repo.clone(), constant::LYRIC)
        .or(
            get_playlist_routes(repo.clone(), constant::PLAYLIST)
        )
        .with(warp::trace::request())
        .recover(crate::recover::handle_rejection);

    let (_address, server) = 
        warp::serve(routes)
        .try_bind_with_graceful_shutdown((constant::HOST, port), async move {
            signal::ctrl_c().await.unwrap();
            info!("{}", message::STOPPING);
            if let Err(error) = repo.stop().await {
                error!("{}", error);
            }
            else {
                info!("{}", message::FINISHED);
            }
        })?;

    server.await;

    Ok(())

}
