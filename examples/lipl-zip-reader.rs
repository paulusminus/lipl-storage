use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::time::{Instant};
use tokio::runtime::{Builder};
use uuid::Uuid;
use zip::read::{ZipArchive, ZipFile};

use lipl_io::model;
use lipl_io::io;
use model::PathBufExt;

fn to_uuid(z: &ZipFile) -> Uuid {
    PathBuf::from(z.name()).to_uuid()
}

fn filename(z: &ZipFile) -> String {
    z.name().into()
}

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let result = rt.block_on(async {
        let zip_file = File::open("./out/lipl.zip")?;
        let zip = &mut ZipArchive::new(zip_file)?;

        let mut lyric_hm: HashMap<Uuid, model::Lyric> = HashMap::new();
        let mut playlist_hm: HashMap<Uuid, model::Playlist> = HashMap::new();

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;
            if file.is_file() {
                let uuid = to_uuid(&file);
                let filename = filename(&file);
                if filename.ends_with(".txt") {
                    lyric_hm.insert(
                        uuid,
                        io::get_lyric(file, uuid).await?
                    );
                }
                else if filename.ends_with(".yaml") {
                    playlist_hm.insert(
                        uuid,
                        (uuid, io::get_playlist(Ok(file)).unwrap()).into()
                    );
                }
            }
        };
        
        Ok(())
    });

    println!("Elapsed: {:?}", start.elapsed());
    result
}
