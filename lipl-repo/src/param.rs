use std::path::{PathBuf};
use clap::{Clap, ValueHint};

#[derive(Clap, Debug)]
#[clap(about = "Serving the db through http")]
pub struct Serve {
    #[clap(short, long, required = true)]
    pub port: u16,
    #[clap(value_hint = ValueHint::AnyPath)]
    pub source: PathBuf,
}

#[derive(Clap, Debug)]
#[clap(about = "Show db summary on console")]
pub struct ListCommand {
    #[clap(name = "source", parse(from_os_str), value_hint = ValueHint::AnyPath)]
    pub source: PathBuf,
}

#[derive(Clap, Debug)]
#[clap(about = "Copy db to another destination")]
pub struct CopyCommand {
    #[clap(name = "source", parse(from_os_str), value_hint = ValueHint::AnyPath)]
    pub source: PathBuf,
    #[clap(name = "target", parse(from_os_str), value_hint = ValueHint::AnyPath)]
    pub target: PathBuf,
}

#[derive(Clap, Debug)]
#[clap(about = "Utilities for db")]
pub enum DbCommand {
    #[clap(name = "list")]
    List(ListCommand),
    #[clap(name="copy")]
    Copy(CopyCommand),
}

#[derive(Clap, Debug)]
#[clap(name = "lipl-repo", author, version)]
pub enum Command {
    #[clap(name = "db")]
    Db(DbCommand),
    #[clap(name = "serve")]
    Serve(Serve),
}

#[derive(Clap, Debug)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}
