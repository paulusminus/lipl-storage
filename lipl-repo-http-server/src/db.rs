use crate::param::{ListCommand, CopyCommand};
use anyhow::Result;
use lipl_fs_repo::FileRepo;
use lipl_types::{LiplRepo};
use log::{info};

pub async fn repo_list(args: ListCommand) -> Result<()> {
    let now = std::time::Instant::now();
    let path = args.source.to_owned().to_string_lossy().to_string();
    let repo = lipl_fs_repo::FileRepo::new(
        path, 
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

pub async fn copy(args: CopyCommand) -> Result<()> {
    info!(
        "Start copying {} to {}",
        &args.source.to_string_lossy(),
        &args.target.to_string_lossy(),
     );

    let source = FileRepo::new(
        args.source.to_string_lossy().to_string(), 
    )?;

    let target = FileRepo::new(
        args.target.to_string_lossy().to_string(),
    )?;

    for lyric in source.get_lyrics().await? {
        log::info!("Copying lyric {} with id {}", lyric.title, lyric.id);
        target.post_lyric(lyric).await?;
    }

    for playlist in source.get_playlists().await? {
        log::info!("Copying playlist {} with id {}", playlist.title, playlist.id);
        target.post_playlist(playlist).await?;
    }

    info!(
        "Finished copying {} to {}",
        args.source.to_string_lossy(),
        args.target.to_string_lossy(),
    );
    Ok(())
}