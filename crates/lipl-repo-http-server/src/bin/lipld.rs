use anyhow::Result;
use clap::Parser;
use tracing_subscriber::fmt::format::FmtSpan;
use std::env::var;
use tracing::{info};

use lipl_repo_http_server::{param, serve, db, message};

#[tokio::main(flavor = "current_thread") ]
async fn main() -> Result<()> {

    let arguments = param::Arguments::parse();
    match arguments.command {
        param::Command::Serve(serve_args) => {
            let filter = 
                var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_owned());
            tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .init();

            info!("{}", message::STARTING);
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
