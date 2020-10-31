use std::error::Error;
use lipl_io::{get_lyrics};
use tokio::stream::StreamExt;
use std::time::{Instant};

const DIR: &str = "/home/paul/Code/rust/filesystem/data/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let result = get_lyrics(DIR).await?;
    tokio::pin!(result);

    while let Some(lyric) = result.next().await {
        println!("id: {}", lyric.id);
        println!("Number of parts: {}", lyric.parts.len());
        
        if let Some(yaml) = lyric.yaml {
            println!("{}", yaml);
        }
    }

    println!("Elapsed: {} ms", start.elapsed().as_millis());
    Ok(())
}
