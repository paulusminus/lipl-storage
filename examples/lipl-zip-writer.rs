use std::time::{Instant};
use tokio::runtime::{Builder};

use lipl_io::{get_path};
use lipl_io::model;

use lipl_io::io::zip_write;

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let result = rt.block_on(async {
        let path = get_path()?;

        let zip_path = "./out/lipl.zip";
        let (lyrics, playlists) = model::create_db(&path).await?;

        zip_write(zip_path, lyrics, playlists)?;
    
        Ok(())
    });

    println!("Elapsed: {:?}", start.elapsed());
    result
}
