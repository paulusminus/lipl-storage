use std::error::Error;
use lipl_io::{get_lyrics};
use tokio::stream::StreamExt;

const DIR: &str = "/home/paul/Code/rust/filesystem/data/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = get_lyrics(DIR).await?;
    tokio::pin!(result);

    while let Some(lyric) = result.next().await {
        println!("{}", lyric.id);
        println!("Number of parts: {}", lyric.parts.len());
        
        if lyric.yaml.is_some() {
            println!("{}", lyric.yaml.unwrap());
        }
    }

    Ok(())
}
