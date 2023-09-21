use std::{fmt::{Display}, time::Instant};
use lipl_repo_fs::{FileRepo};
use lipl_core::{LiplRepo};

pub fn print<D>(d: D) 
where 
    D: Display
{
    println!("{}", d);
}

pub async fn process() -> Result<(), Box<dyn std::error::Error>> {
    let repo = FileRepo::new(
        "./data/".to_owned(),
    )
    .await?;

    println!("Lyrics");
    repo
    .get_lyric_summaries()
    .await?
    .into_iter()
    .for_each(print);

    println!();

    println!("Playlists");
    repo
    .get_playlist_summaries()
    .await?
    .into_iter()
    .for_each(print);

    repo.stop().await?;
    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    process().await?;
    println!("Took {} milliseconds", now.elapsed().as_millis());
    Ok(())
}
