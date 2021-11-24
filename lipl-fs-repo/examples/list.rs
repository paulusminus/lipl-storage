use lipl_fs_repo::{time_it, to_std_output, FileSystem, LiplRepo, Result};

pub async fn process() -> Result<()> {
    let repo = FileSystem::new("../data/", "yaml", "txt")?;

    println!("Lyrics");
    repo.get_lyric_summaries().await?.iter().for_each(to_std_output);
    println!();

    // for summary in repo.get_lyric_summaries().await? {
    //     let lyric = repo.get_lyric(summary.id).await?;
    //     repo.post_lyric(lyric).await?;
    // }

    println!("Playlists");
    repo.get_playlist_summaries().await?.iter().for_each(to_std_output);

    // repo.delete_lyric("KGxasqUC1Uojk1viLGbMZK".to_owned()).await?;

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()>{
    time_it(process).await
}
