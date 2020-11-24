use std::io::{Error as IOError, ErrorKind};
use std::path::Path;
use std::time::{Instant};
use futures::StreamExt;
use tokio::runtime::{Builder};

use lipl_io::{get_lyrics, get_playlists, Lyric, Playlist};

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

        let lyrics: Vec<Lyric> = get_lyrics(&path).await?.collect().await;

        for lyric in lyrics.iter() {
            println!(
                "Lyric: {}, {} parts, id = {}",
                lyric.title.as_ref().unwrap_or(&"<< onbekend >>".to_owned()),
                lyric.parts.len(),
                lyric.id,
            );
        };

        let playlists: Vec<Playlist> = get_playlists(&path).await?.collect().await;

        for playlist in playlists {
            println!();
            println!("Playlist: {}", playlist.title);
            for member in playlist.members {
                println!("  - {}, {:?}", member, lyrics.iter().filter(|l| l.id == member).collect::<Vec<&Lyric>>()[0].title);
            }
        }
    
        println!("Elapsed: {:?} ms", start.elapsed());
        Ok(())
    })
}
