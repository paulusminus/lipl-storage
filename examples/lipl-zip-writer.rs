use std::time::{Instant};

use lipl_io::{get_path};
use lipl_io::model;

use lipl_io::io::{fs_read, zip_write};

fn main() -> model::LiplResult<()> {
    let start = Instant::now();

    let path = get_path()?;

    let zip_path = "./out/lipl.zip";
    let (lyrics, playlists) = fs_read(&path)?;

    zip_write(zip_path, lyrics, playlists)?;

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
