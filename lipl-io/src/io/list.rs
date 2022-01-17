use std::path::{Path};
use std::time::{Instant};
use crate::model::{Db, Persist};
use lipl_types::{RepoResult};

pub fn list<P>(source: P) -> RepoResult<()> 
where P: AsRef<Path>,
{
    let start = Instant::now();

    let mut db = Db::new(source.as_ref().into());
    db.load()?;

    println!("Lyrics");
    for lyric in db.get_lyric_list() {
            println!("  - {}", lyric.title);
    };

    for playlist in db.get_playlist_list() {
        println!();
        println!("Playlist: {}", playlist.title);
        for member in playlist.members.iter() {
            if let Some(title) = db.get_lyric(member).map(|l| l.title) {
                println!("  - {}", title);
            }
        }
    }
    
    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}
