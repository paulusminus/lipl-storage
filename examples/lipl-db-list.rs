use std::time::{Instant};
use lipl_io::model;
use lipl_io::{get_path};

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();

    let path = get_path()?;
    let (lyrics, playlists) = 
        if path.is_file() {
            lipl_io::io::zip_read(path)?
        }
        else {
            model::create_db(path)?
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
