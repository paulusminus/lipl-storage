use std::fmt::{Display};
use lipl_fs_repo::{FileSystem, LiplRepo};
use lipl_fs_repo::model::{PlaylistPost, Result};
use lipl_fs_repo::elapsed::{Elapsed};

pub fn print<D>(d: D) 
where 
    D: Display
{
    println!("{}", d);
}

pub async fn process() -> Result<()> {
    let repo = FileSystem::new("../data/", "yaml", "txt")?;

    println!("Lyrics");
    repo.get_lyric_summaries().await?.iter().for_each(print);
    println!();

    // for summary in repo.get_lyric_summaries().await? {
    //     let lyric = repo.get_lyric(summary.id).await?;
    //     println!("{}", lyric);
    // }

    for playlist in repo.get_playlists().await? {
        println!("{}", PlaylistPost::from(playlist));
    }
    // println!("Playlists");
    // repo.get_playlist_summaries().await?.iter().for_each(print);

    // repo.delete_lyric("KGxasqUC1Uojk1viLGbMZK".to_owned()).await?;

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()>{
    println!("Elapsed: {} milliseconds", process.elapsed().await?);
    Ok(())
}
