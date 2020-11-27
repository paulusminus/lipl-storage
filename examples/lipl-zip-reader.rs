use std::time::{Instant};
use tokio::runtime::{Builder};

use lipl_io::io::zip_read;

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let result = rt.block_on(async {
        let path = "./out/lipl.zip";
        let (lyrics, playlists) = zip_read(path).await?;

        for lyric in lyrics.values() {
            println!("{}", lyric);
        }

        for playlist in playlists.values() {
            println!();
            println!("{}", playlist);
        }

        Ok(())
    });

    println!("Elapsed: {:?}", start.elapsed());
    result
}
