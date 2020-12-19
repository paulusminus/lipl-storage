use std::path::PathBuf;
use lipl_io::model::{LiplResult};
use lipl_io::io::{copy};
use clap::{Clap, ValueHint};

#[derive(Clap, Debug)]
#[clap(about = "Copy Lipl Db", author, version, name = "lipl-db-copy") ]
struct Opt {
    #[clap(required = true, index = 1, parse(from_os_str), value_hint = ValueHint::FilePath)]
    source: PathBuf,
    #[clap(required = true, index = 2, parse(from_os_str), value_hint = ValueHint::FilePath)]
    target: PathBuf,
}

fn main() -> LiplResult<()> {
    let opt = Opt::parse();
    copy(opt.source, opt.target)
}
