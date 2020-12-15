use std::path::{Path};
use std::time::{Instant};
use crate::model::{LiplResult, PathBufExt, ZIP};
use crate::io::{fs_read, zip_read};

pub fn list<P: AsRef<Path>>(source: P) -> LiplResult<()> {
    let start = Instant::now();

    let db = 
        if source.as_ref().is_file_type(ZIP) {
            zip_read(source)?
        }
        else {
            fs_read(source)?
        };

    println!("Lyrics");
    for lyric in db.get_lyric_list() {
        if let Some(title) = &lyric.title {
            println!("  - {}", title);
        }
        // println!("{}", lyric);
    };

    for playlist in db.get_playlist_list() {
        println!();
        println!("Playlist: {}", playlist.title);
        for member in playlist.members.iter() {
            if let Some(title) = db.get_lyric(member).and_then(|l| l.title.as_ref()) {
                println!("  - {}", title);
            }
        }
    }
    
    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
