use std::fmt::{Display};
use lipl_fs_repo::{FileRepo};
use lipl_types::{RepoResult, LiplRepo};
use lipl_fs_repo::elapsed::{Elapsed};

pub fn print<D>(d: D) 
where 
    D: Display
{
    println!("{}", d);
}

pub async fn process() -> RepoResult<()> {
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
async fn main() -> RepoResult<()>{
    println!("Elapsed: {} milliseconds", process.elapsed().await?);
    Ok(())
}
