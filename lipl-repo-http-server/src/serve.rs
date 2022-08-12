use anyhow::Result;
use lipl_types::LiplRepo;
use tokio::signal;
use tracing::{info, error};
use warp::Filter;

use crate::constant;
use crate::message;
use crate::param;
use crate::filter::{get_lyric_routes, get_playlist_routes};
use crate::param::DbType;

async fn run<R, E>(repo: R, port: u16) -> Result<()> 
where
    R: LiplRepo<E> + 'static,
    E: std::error::Error + 'static,
{
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
            };
            info!("{}", message::FINISHED);
        })?;

    server.await;

    Ok(())

}

pub async fn serve(param: param::Serve) -> Result<()> {

    match param.source.parse::<DbType>()? {
        DbType::File(_, file) => {
            let repo = file.await?;
            run(repo, param.port).await?;

        },
        DbType::Postgres(_, postgres) => {
            let repo = postgres.await?;
            run(repo, param.port).await?;
        }
    }
    Ok(())
}