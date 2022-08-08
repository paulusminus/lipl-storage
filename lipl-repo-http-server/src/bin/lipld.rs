use anyhow::Result;
use clap::Parser;
use tracing::{info};

use lipl_repo_http_server::{param, serve, db, message};

#[tokio::main(flavor = "current_thread") ]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("{}", message::STARTING);

    let arguments = param::Arguments::parse();
    match arguments.command {
        param::Command::Serve(serve_args) => {
            serve::serve(serve_args).await
        },
        param::Command::Db(db) => {
            match db {
                param::DbCommand::Copy(copy_args) => {
                    db::repo_copy(copy_args).await
                },
                param::DbCommand::List(list_args) => {
                    db::repo_list(list_args).await
                },
            }
        }
    }
}
