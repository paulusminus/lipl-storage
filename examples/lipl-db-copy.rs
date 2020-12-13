use std::path::PathBuf;
use std::time::{Instant};
use lipl_io::model;
use lipl_io::io::{fs_read, zip_write};

fn main() -> model::LiplResult<()> {
    let start = Instant::now();

    let matches = clap::args();
    let get_value = |s: &str| matches.value_of(s).unwrap();
    let source_path: PathBuf = get_value("source").into();
    let target_path: PathBuf = get_value("target").into();

    let (lyrics, playlists) = fs_read(&source_path)?;

    zip_write(target_path, lyrics, playlists)?;

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}

mod clap {
    use clap::{crate_authors, crate_version, Arg, App, ArgMatches};
    pub fn args() -> ArgMatches {
        App::new("lipl-db-copy")
        .about("List lyrics and playlists from directory or zipfile")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::new("source")
            .value_name("source")
            .about("the source directory or zipfile")
            .required(true)
            .index(1)
        )
        .arg(
            Arg::new("target")
            .value_name("target")
            .about("the target directory or zipfile")
            .required(true)
            .index(2)
        )
        .get_matches()
    }
}