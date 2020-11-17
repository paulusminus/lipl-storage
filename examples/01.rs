use std::collections::HashMap;
use std::io::{Error as IOError, ErrorKind};
use std::path::Path;
use std::time::{Instant};

use tokio::runtime::{Builder};
use tokio::stream::StreamExt;

use lipl_io::{get_lyrics};

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
        let mut hm: HashMap<String, Vec<String>> = HashMap::new();

        let lyrics = get_lyrics(&path).await.expect(&format!("No results for {}", &path));
        tokio::pin!(lyrics);
    
        while let Some(lyric) = lyrics.next().await {
            if let Some(playlists) = &lyric.member_of {
                playlists.iter().for_each(|pl| {
                    hm.entry(pl.clone()).or_insert(Vec::new()).push(lyric.id.clone());
                });
            };

            println!(
                "Title: {}, {} parts, id = {}, member of: {}",
                lyric.title.unwrap_or("<< onbekend >>".to_owned()),
                lyric.parts.len(),
                lyric.id,
                lyric.member_of.unwrap_or_default().join(", "),
            );
            println!();
        }

        hm.keys().for_each(|key| {
            println!("title: {}", key);
            println!("members:");
            hm[key].iter().for_each(|value| {
                println!("  - {}", value);
            });
            println!();
        });
    
        println!("Elapsed: {} ms", start.elapsed().as_millis());
        Ok(())
    })
}
