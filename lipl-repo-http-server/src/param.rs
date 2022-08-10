use std::{str::FromStr, pin::Pin, future::Future, fmt::Debug};
use clap::{Parser, Subcommand};
use lipl_fs_repo::FileRepo;
use lipl_postgres_repo::{PostgresRepo};
use lipl_types::{ModelError, error::ModelResult};

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
    pub source: DbType,
    #[clap(long)]
    pub yaml: bool,
}

#[derive(Parser, Debug)]
#[clap(about = "Copy db to another destination")]
pub struct CopyCommand {
    #[clap(short, long)]
    pub source: DbType,
    #[clap(short, long)]
    pub target: DbType,
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
    File(String, Pin<Box<dyn Future<Output = ModelResult<FileRepo>>>>),
    Postgres(String, Pin<Box<dyn Future<Output = ModelResult<PostgresRepo>>>>),
}

impl Debug for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DbType::File(file, _) => format!("File connection: {file}"),
            DbType::Postgres(postgres, _) => format!("Postgres connection: {postgres}"),
        })
    }
}


impl FromStr for DbType {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(':').collect::<Vec<&str>>();
        if splitted.len() == 2 {
            let repo_dir = splitted[1].to_owned();
            if splitted[0] == "file" {
                return Ok(
                    DbType::File(
                        repo_dir.clone(),
                        Box::pin(
                            async move {
                                FileRepo::new(repo_dir)
                            }
                        )
                    )
                );
            }
            else if splitted[0] == "postgres" {
                return Ok(
                    DbType::Postgres(
                        repo_dir.clone(),
                        Box::pin(
                            async move {
                                PostgresRepo::new(repo_dir, false)
                                .await
                                .map_err(|_| ModelError::Argument("Invalid postgres connection string".to_owned()))
                            }
                        )
                    )
                );
            }
            else {
                return Err(ModelError::Argument("Unknown prefix for db connection string".to_owned()));
            }
        }
        Err(ModelError::Argument("Unknown format for db connection string. Use '<PREFIX>:<Connection string>'".to_owned()))
    }
}

