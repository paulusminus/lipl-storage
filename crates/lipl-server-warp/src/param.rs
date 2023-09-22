use crate::repo::RepoConfig;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
pub struct ServeCommand {
    #[arg(long, short)]
    pub port: u16,
    #[arg(long, short)]
    pub source: Box<RepoConfig>,
}

#[derive(Parser)]
pub struct CopyCommand {
    #[arg(long, short)]
    pub source: Box<RepoConfig>,
    #[arg(long, short)]
    pub target: Box<RepoConfig>,
}

#[derive(Parser)]
pub struct ListCommand {
    #[arg(long, short)]
    pub source: RepoConfig,
    #[arg(long, short)]
    pub yaml: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct LiplApp {
    #[command(subcommand)]
    pub command: LiplCommand,
}

#[derive(Subcommand)]
pub enum LiplCommand {
    Serve(ServeCommand),
    Copy(CopyCommand),
    List(ListCommand),
}
