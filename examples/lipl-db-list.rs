use std::time::{Instant};
use lipl_io::model;
use lipl_io::{get_path};
use lipl_io::io::{fs_read, zip_read};

fn main() -> model::LiplResult<()> {
    let start = Instant::now();

    let path = get_path()?;
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
