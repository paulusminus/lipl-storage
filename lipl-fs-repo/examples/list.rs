use std::{fmt::{Display}, time::Instant};
use lipl_fs_repo::{FileRepo};
use lipl_types::{LiplRepo};

pub fn print<D>(d: D) 
where 
    D: Display
{
    println!("{}", d);
}

pub async fn process() -> anyhow::Result<()> {
    let repo = FileRepo::new(
        "./data/".to_owned(),
    )?;

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

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()>{
    let now = Instant::now();
    process().await?;
    println!("Took {} milliseconds", now.elapsed().as_millis());
    Ok(())
}
