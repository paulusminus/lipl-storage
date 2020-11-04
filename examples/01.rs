use std::io::{ErrorKind};
use lipl_io::{get_lyrics};
use tokio::runtime::{Builder};
use tokio::stream::StreamExt;
use std::time::{Instant};

fn main() -> Result<(), std::io::Error> {

    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let start = Instant::now();

        let mut args = std::env::args();
        if args.len() < 2 {
            return Err(std::io::Error::new(ErrorKind::Other, "Argument directory missing"));
        }
    
        let path = args.nth(1).unwrap();
        if !std::path::Path::new(&path).exists() {
            return Err(std::io::Error::new(ErrorKind::Other, "Directory not found"));
        }
    
        let result = get_lyrics(&path).await.expect(&format!("No results for {}", path));
        tokio::pin!(result);
    
        while let Some(lyric) = result.next().await {
            println!(
                "Title: {}, {} parts, id = {}, member of: {}",
                lyric.title.unwrap_or("<< onbekend >>".to_owned()),
                lyric.parts.len(),
                lyric.id,
                lyric.member_of.unwrap_or_default().join(", ")

            );
            println!();
        }
    
        println!("Elapsed: {} ms", start.elapsed().as_millis());
        Ok(())
    })
}
