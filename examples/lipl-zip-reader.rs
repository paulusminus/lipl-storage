use std::fs::File;
use std::path::PathBuf;
use std::time::{Instant};
use tokio::runtime::{Builder};
use uuid::Uuid;
use zip::read::{ZipArchive, ZipFile};

use lipl_io::{PathBufExt, get_lyric, get_playlist, Playlist};

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

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;
            if file.is_file() {
                let uuid = to_uuid(&file);
                let filename = filename(&file);
                if filename.ends_with(".txt") {
                    let lyric = get_lyric(file, uuid).await?;
                    println!("{}", lyric);

                }
                else if filename.ends_with(".yaml") {
                    let playlist_post = get_playlist(Ok(file)).unwrap();
                    println!("{}", Playlist {
                        id: uuid,
                        title: playlist_post.title,
                        members: playlist_post.members.iter().map(|s| PathBuf::from(s).to_uuid()).collect(),
                    });
                }
            }
        };
        
        Ok(())
    });

    println!("Elapsed: {:?}", start.elapsed());
    result
}
