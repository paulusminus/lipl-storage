use std::time::{Instant};
use lipl_io::model::{LiplResult};
use lipl_io::io::{fs_read, zip_read};
use std::path::PathBuf;

fn main() -> LiplResult<()> {
    let start = Instant::now();

    let path: PathBuf = clap::args().value_of("source").unwrap().into();
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

mod clap {
    use clap::{crate_authors, Arg, App, ArgMatches};
    pub fn args() -> ArgMatches {
        App::new("lipl-db-list")
        .about("List lyrics and playlists from directory or zipfile")
        .version("1.0")
        .author(crate_authors!("\n"))
        .arg(
            Arg::new("source")
            .value_name("source")
            .about("the source directory or zipfile")
            .required(true)
            .index(1)
        )
        .get_matches()
    }
}