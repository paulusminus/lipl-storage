use crate::param::{ListCommand, CopyCommand};
use lipl_io::io::{copy as db_copy, list as db_list};
use anyhow::Result;
use lipl_types::{LiplRepo};
use log::{info};

pub async fn repo_list(args: ListCommand) -> Result<()> {
    let now = std::time::Instant::now();
    let path = args.source.to_owned().to_string_lossy().to_string();
    let repo = lipl_fs_repo::FileRepo::new(
        path, 
        "yaml".to_owned(),
        "txt".to_owned(),
    )?;

    println!("Lyrics:");
    let lyrics = repo.get_lyrics().await?;
    
    for lyric in lyrics.iter() {
        println!(" - {}, {} parts", lyric.title, lyric.parts.len());
    }

    let playlists = repo.get_playlists().await?;
    for playlist in playlists {
        println!();
        println!("{}", playlist.title);

        for member in playlist.members {
            if let Some(lyric) = lyrics.iter().filter(|lyric| lyric.id == member).last() {
                println!(" - {}", lyric.title);
            };
        }
    }

    println!();
    println!("Elapsed: {} milliseconds", now.elapsed().as_millis());
    Ok(())
}

pub fn list(args: ListCommand) -> Result<()> {
    db_list(args.source)?;
    Ok(())
}

pub fn copy(args: CopyCommand) -> Result<()> {
    info!(
        "Start copying {} to {}",
        &args.source.to_string_lossy(),
        &args.target.to_string_lossy(),
     );

     db_copy(&args.source, &args.target)?;

     info!(
        "Finished copying {} to {}",
        args.source.to_string_lossy(),
        args.target.to_string_lossy(),
    );
    Ok(())
}