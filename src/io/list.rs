use std::path::{Path};
use std::time::{Instant};
use crate::model::{LiplResult, PathBufExt, ZIP};
use crate::io::{fs_read, zip_read};

pub fn list<P: AsRef<Path>>(source: P) -> LiplResult<()> {
    let start = Instant::now();

    let (lyrics, playlists) = 
        if source.as_ref().is_file_type(ZIP) {
            zip_read(source)?
        }
        else {
            fs_read(source)?
        };

    println!("Lyrics");
    for lyric in lyrics.values() {
        if let Some(title) = &lyric.title {
            println!("  - {}", title);
        }
        // println!("{}", lyric);
    };

    for playlist in playlists.values() {
        println!();
        println!("Playlist: {}", playlist.title);
        for member in playlist.members.iter() {
            if let Some(title) = &lyrics[member].title {
                println!("  - {}", title);
            }
        }
    }
    
    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
