use std::path::{PathBuf};
use clap::{Parser, ValueHint, Subcommand};

#[derive(Parser, Debug)]
#[clap(about = "Serving the db through http")]
pub struct Serve {
    #[clap(short, long, required = true)]
    pub port: u16,
    #[clap(value_hint = ValueHint::AnyPath)]
    pub source: PathBuf,
}

#[derive(Parser, Debug)]
#[clap(about = "Show db summary on console")]
pub struct ListCommand {
    #[clap(parse(from_os_str), value_hint = ValueHint::AnyPath)]
    pub source: PathBuf,
}

#[derive(Parser, Debug)]
#[clap(about = "Copy db to another destination")]
pub struct CopyCommand {
    #[clap(parse(from_os_str), value_hint = ValueHint::AnyPath)]
    pub source: PathBuf,
    #[clap(parse(from_os_str), value_hint = ValueHint::AnyPath)]
    pub target: PathBuf,
}

#[derive(Subcommand, Debug)]
#[clap(about = "Utilities for db")]
pub enum DbCommand {
//    #[clap(name = "list")]
    List(ListCommand),
//    #[clap(name="copy")]
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
