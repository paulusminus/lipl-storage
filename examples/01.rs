use std::error::Error;
use lipl_io::{get_lyrics};
use tokio::stream::StreamExt;
use std::time::{Instant};
use serde::{Deserialize, Serialize};

const DIR: &str = "/home/paul/Code/rust/filesystem/data/";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Whatever {
    title: Option<String>,
    member_of: Option<Vec<String>>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let result = get_lyrics(DIR).await?;
    tokio::pin!(result);

    while let Some(lyric) = result.next().await {
        println!("id: {}", lyric.id);
        println!("Number of parts: {}", lyric.parts.len());
     
        if let Some(yaml) = lyric.yaml {
            let whatever: Whatever = serde_yaml::from_str(&yaml).unwrap();
            if let Some(title) = whatever.title {
                println!("Title: {}", title);
            }
            if let Some(member_of) = whatever.member_of {
                println!("Member of: {}", member_of.join(", "));
            }
        }
        println!();
    }

    println!("Elapsed: {} ms", start.elapsed().as_millis());
    Ok(())
}
