use std::time::{Instant};
use lipl_io::model::{LiplResult};
use lipl_io::io::{fs_read, zip_read};
use std::path::PathBuf;
use clap::{Clap, ValueHint};

#[derive(Clap, Debug)]
#[clap(about = "List lyrics and playlists", author, version, name = "lipl-db-list") ]
struct Opt {
    #[clap(required = true, index = 1, parse(from_os_str), value_hint = ValueHint::FilePath)]
    source: PathBuf
}

fn main() -> LiplResult<()> {
    let start = Instant::now();

    let opt = Opt::parse();
    let path = opt.source;
    let (lyrics, playlists) = 
        if path.is_file() {
            zip_read(path)?
        }
        else {
            fs_read(path)?
        };

    for lyric in lyrics.values() {
        println!("{}", lyric);
    };

    for playlist in playlists.values() {
        println!();
        println!("{}", playlist);
    }
    
    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
