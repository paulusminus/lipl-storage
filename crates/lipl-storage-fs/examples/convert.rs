use lipl_core::Repo;
use lipl_storage_fs::FileRepo;
use std::{fmt::Display, time::Instant};

pub fn print<D>(d: D)
where
    D: Display,
{
    println!("{}", d);
}

pub async fn convert_etag() -> Result<(), Box<dyn std::error::Error>> {
    let repo = FileRepo::new("./data/".to_owned())?;

    println!("Lyrics");
    let lyrics = repo.get_lyrics().await?;
    for lyric in lyrics {
        repo.upsert_lyric(lyric).await?;
    }

    println!();

    repo.stop().await?;
    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    convert_etag().await?;
    println!("Took {} milliseconds", now.elapsed().as_millis());
    Ok(())
}
