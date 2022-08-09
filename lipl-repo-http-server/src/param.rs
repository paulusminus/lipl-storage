use std::{str::FromStr, pin::Pin, future::Future};
use clap::{Parser, Subcommand};
use lipl_fs_repo::FileRepo;
use lipl_postgres_repo::{PostgresRepo};
use lipl_types::{RepoError, error::RepoResult};

#[derive(Parser, Debug)]
#[clap(about = "Serving the db through http")]
pub struct Serve {
    #[clap(short, long, required = true)]
    pub port: u16,
    #[clap(short, long)]
    pub source: String,
}

#[derive(Parser, Debug)]
#[clap(about = "Show db summary on console")]
pub struct ListCommand {
    #[clap(short, long)]
    pub source: String,
}

#[derive(Parser, Debug)]
#[clap(about = "Copy db to another destination")]
pub struct CopyCommand {
    #[clap(short, long, help = "")]
    pub source: String,
    #[clap(short, long, help = "")]
    pub target: String,
}

#[derive(Subcommand, Debug)]
#[clap(about = "Utilities for db")]
pub enum DbCommand {
    List(ListCommand),
    Copy(CopyCommand),
}

#[derive(Subcommand, Debug)]
#[clap(name = "lipl-repo", author, version)]
pub enum Command {
    #[clap(name = "db", subcommand)]
    Db(DbCommand),
    #[clap(name = "serve")]
    Serve(Serve),
}

#[derive(Parser, Debug)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

pub enum DbType {
    File(Pin<Box<dyn Future<Output = RepoResult<FileRepo>>>>),
    Postgres(Pin<Box<dyn Future<Output = RepoResult<PostgresRepo>>>>),
}

impl FromStr for DbType {
    type Err = lipl_types::error::RepoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(':').collect::<Vec<&str>>();
        if splitted.len() == 2 {
            let repo_dir = splitted[1].to_owned();
            if splitted[0] == "file" {
                return Ok(
                    DbType::File(Box::pin(
                        async move {
                            FileRepo::new(repo_dir)
                        }
                    ))
                );
            }
            else if splitted[0] == "postgres" {
                return Ok(DbType::Postgres(
                    Box::pin(
                        async move {
                            PostgresRepo::new(repo_dir, false)
                            .await
                            .map_err(|_| lipl_types::error::RepoError::Argument("Invalid postgres connection string".to_owned()))
                        }
                    ))
                );
            }
            else {
                return Err(lipl_types::RepoError::Argument("Unknown prefix for db connection string".to_owned()));
            }
        }
        Err(RepoError::Argument("Unknown format for db connection string. Use '<PREFIX>:<Connection string>'".to_owned()))
    }
}

