#[macro_use]
extern crate log;

mod constant;
mod db;
mod lyric_filter;
mod lyric_handler;
mod message;
mod model;
mod param;
mod playlist_filter;
mod playlist_handler;
mod serve;

use anyhow::Result;
use clap::Clap;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(constant::LOG_LEVEL)).init();
    info!("{}", message::STARTING);

    let arguments = param::Arguments::parse();
    match arguments.command {
        param::Command::Serve(serve_args) => {
            serve::serve(serve_args).await?;
        },
        param::Command::Db(db) => {
            match db {
                param::DbCommand::Copy(copy_args) => {
                    db::copy(copy_args)?;
                },
                param::DbCommand::List(list_args) => {
                    db::list(list_args)?;
                },
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
