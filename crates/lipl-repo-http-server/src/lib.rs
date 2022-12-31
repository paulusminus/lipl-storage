pub mod handler;
pub mod constant;
pub mod db;
mod error;
mod filter;
pub mod message;
mod model;
pub mod param;
mod recover;
mod repo;
pub mod serve;

use param::{LiplApp, LiplCommand};
use clap::{Parser};
use futures::TryFutureExt;

pub async fn run() -> anyhow::Result<()> {
    let cli = LiplApp::parse();
    match cli.command {
        LiplCommand::Serve(serve) => {
            serve.source.build_repo()
            .and_then(|source| crate::serve::run(source, serve.port))
            .await
        },
        LiplCommand::Copy(copy) => {
            copy.source.build_repo()
            .and_then(|source| copy.target.build_repo().map_ok(|target| (source, target)))
            .and_then(|(source, target)| crate::db::copy(source, target))
            .await
        },
        LiplCommand::List(list) => {
            list.source.build_repo()
            .and_then(|source| crate::db::list(source, list.yaml))
            .await
        }
    }
}
