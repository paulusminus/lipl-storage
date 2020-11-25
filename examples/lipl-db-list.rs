use std::time::{Instant};
use tokio::runtime::{Builder};

use lipl_io::{create_db, get_path};

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let result = rt.block_on(async {
        let path = get_path()?;
        let (lyrics, playlists) = create_db(&path).await?;

        for lyric in lyrics.values() {
            println!("{}", lyric);
        };

        for playlist in playlists.values() {
            println!();
            println!("{}", playlist);
        }
    
        Ok(())
    });

    println!("Elapsed: {:?} ms", start.elapsed());
    result
}
