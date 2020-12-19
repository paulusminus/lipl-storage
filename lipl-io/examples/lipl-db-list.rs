use lipl_io::model::{LiplResult};
use lipl_io::io::{list};
use std::path::PathBuf;
use clap::{Clap, ValueHint};

#[derive(Clap, Debug)]
#[clap(about = "List lyrics and playlists", author, version, name = "lipl-db-list") ]
struct Opt {
    #[clap(required = true, index = 1, parse(from_os_str), value_hint = ValueHint::FilePath)]
    source: PathBuf
}

fn main() -> LiplResult<()> {
    let opt = Opt::parse();
    list(opt.source)
}
