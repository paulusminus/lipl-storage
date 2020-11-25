use std::fs::File;
use std::io::Read;
use std::time::{Instant};
use tokio::runtime::{Builder};
use zip::read::ZipArchive;

use lipl_io::{create_db, get_path, UuidExt, PathBufExt};

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let result = rt.block_on(async {
        let zip_file = File::open("./out/lipl.zip")?;
        let mut zip = ZipArchive::new(zip_file)?;

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;
            if file.is_file() {
                if file.name().ends_with(".txt") {
                    println!("Lyric: {}", file.sanitized_name().to_uuid());

                }
                if file.name().ends_with(".yaml") {
                    println!("Playlist: {}", file.sanitized_name().to_uuid());
                }
            }
        };
        
        Ok(())
    });

    println!("Elapsed: {:?}", start.elapsed());
    result
}
