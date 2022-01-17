use std::fmt::{Display};
use lipl_fs_repo::{FileSystem};
use lipl_types::{LiplRepo, PlaylistPost, RepoResult};
use lipl_fs_repo::elapsed::{Elapsed};

pub fn print<D>(d: D) 
where 
    D: Display
{
    println!("{}", d);
}

pub async fn process() -> RepoResult<()> {
    let repo = FileSystem::new("./data/", "yaml", "txt")?;

    println!("Lyrics");
    repo.get_lyric_summaries().await?.into_iter().for_each(print);

    // for lyric in repo.get_lyrics().await? {
    //     repo.post_lyric(lyric).await?;
    // }

    println!();

    // for summary in repo.get_lyric_summaries().await? {
    //     let lyric = repo.get_lyric(summary.id).await?;
    //     println!("{}", lyric);
    // }

    println!("Playlists");
    repo.get_playlists().await?.into_iter().map(PlaylistPost::from).for_each(print);
    // println!("Playlists");
    // repo.get_playlist_summaries().await?.iter().for_each(print);

    // repo.delete_lyric("KGxasqUC1Uojk1viLGbMZK".to_owned()).await?;

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> RepoResult<()>{
    println!("Elapsed: {} milliseconds", process.elapsed().await?);
    Ok(())
}
