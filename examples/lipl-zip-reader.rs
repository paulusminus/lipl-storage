use std::time::{Instant};
use lipl_io::io::zip_read;
use lipl_io::model::LiplResult;

fn main() -> LiplResult<()> {
    let start = Instant::now();

    let path = "./out/lipl.zip";
    let (lyrics, playlists) = zip_read(path)?;

    for lyric in lyrics.values() {
        println!("{}", lyric);
    }

    for playlist in playlists.values() {
        println!();
        println!("{}", playlist);
    }

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
