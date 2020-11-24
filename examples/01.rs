use std::io::{Error as IOError, ErrorKind};
use std::path::Path;
use std::time::{Instant};
use tokio::runtime::{Builder};

use lipl_io::{create_hashmap, get_lyrics, get_playlists, UuidExt};

fn get_path() -> Result<String, IOError> {
    let mut args = std::env::args();

    if args.len() < 2 {
        return Err(IOError::new(ErrorKind::Other, "Argument directory missing"));
    }

    let path = args.nth(1).ok_or(std::io::Error::new(ErrorKind::Other, "Cannot parse argument 1"))?;
    if !Path::new(&path).exists() {
        return Err(IOError::new(ErrorKind::Other, "Directory not found"));
    }

    Ok(path)
}


fn main() -> Result<(), std::io::Error> {

    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let start = Instant::now();

        let path = get_path()?;

        let lyrics = create_hashmap(get_lyrics(&path).await?).await;

        for (uuid, lyric) in lyrics.iter() {
            println!(
                "Lyric: {}, {} parts, id = {}",
                lyric.title.as_ref().unwrap_or(&"<< onbekend >>".to_owned()),
                lyric.parts.len(),
                uuid.to_base58(),
            );
        };

        let playlists = create_hashmap(get_playlists(&path).await?).await;

        for (uuid, playlist) in playlists {
            println!();
            println!("Playlist: {}, id = {}", playlist.title, uuid.to_base58());
            for member in playlist.members {
                println!("  - {}", member.to_base58());
            }
        }
    
        println!("Elapsed: {:?} ms", start.elapsed());
        Ok(())
    })
}
