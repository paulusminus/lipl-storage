use std::time::{Instant};

use lipl_io::{get_path};
use lipl_io::model;

use lipl_io::io::zip_write;

fn main() -> model::LiplResult<()> {
    let start = Instant::now();

    let path = get_path()?;

    let zip_path = "./out/lipl.zip";
    let (lyrics, playlists) = model::create_db(&path)?;

    zip_write(zip_path, lyrics, playlists)?;

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
