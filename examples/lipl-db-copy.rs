use std::path::PathBuf;
use std::time::{Instant};
use lipl_io::model::{LiplResult, PathBufExt, ZIP};
use lipl_io::io::{fs_read, fs_write, zip_read, zip_write};
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
    let start = Instant::now();
    let opt = Opt::parse();

    let (lyrics, playlists) = if opt.source.has_extension(ZIP) { zip_read(opt.source)? } else { fs_read(opt.source)? };

    if opt.target.has_extension(ZIP) { 
        zip_write(opt.target, lyrics, playlists)?
    }
    else {
        fs_write(opt.target, lyrics, playlists)?;
    };

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
