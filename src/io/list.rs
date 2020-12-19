use std::path::{Path};
use std::time::{Instant};
use crate::model::{LiplResult, Db, Persist};

pub fn list<P: AsRef<Path>>(source: P) -> LiplResult<()> {
    let start = Instant::now();

    let mut db = Db::new(source.as_ref().into());
    db.load()?;

    println!("Lyrics");
    for lyric in db.get_lyric_list() {
        if let Some(title) = &lyric.title {
            println!("  - {}", title);
        }
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
