use crate::param::{ListCommand, CopyCommand, DbType, get_file_repo, get_postgres_repo};
use anyhow::Result;
use lipl_types::LiplRepo;
use log::{info};

pub async fn list(repo: impl LiplRepo) -> Result<()> {
    let now = std::time::Instant::now();

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

pub async fn repo_list(args: ListCommand) -> Result<()> {

    match args.source.parse::<DbType>()? {
        DbType::File(s) => {
            let repo = get_file_repo(s).await?;
            list(repo).await?;
        },
        DbType::Postgres(s) => {
            let repo = get_postgres_repo(s).await?;
            list(repo).await?;
        }
    }

    Ok(())
}

pub async fn copy(source: impl LiplRepo, target: impl LiplRepo) -> Result<()> {
    for lyric in source.get_lyrics().await? {
        log::info!("Copying lyric {} with id {}", lyric.title, lyric.id);
        target.post_lyric(lyric).await?;
    }

    for playlist in source.get_playlists().await? {
        log::info!("Copying playlist {} with id {}", playlist.title, playlist.id);
        target.post_playlist(playlist).await?;
    }

    Ok(())
}


pub async fn repo_copy(args: CopyCommand) -> Result<()> {
    info!(
        "Start copying {} to {}",
        &args.source,
        &args.target,
    );

    let source_db_type = args.source.parse::<DbType>()?;
    let target_db_type = args.target.parse::<DbType>()?;

    match source_db_type {
        DbType::File(file) => {
            let source_repo = get_file_repo(file).await?;

            match target_db_type {
                DbType::File(file) => {
                    let target_repo = get_file_repo(file).await?;
                    copy(source_repo, target_repo).await?;
                },
                DbType::Postgres(postgres) => {
                    let target_repo = get_postgres_repo(postgres).await?;
                    copy(source_repo, target_repo).await?;
                }
            }
        },
        DbType::Postgres(postgres) => {
            let source_repo = get_postgres_repo(postgres).await?;

            match target_db_type {
                DbType::File(file) => {
                    let target_repo = get_file_repo(file).await?;
                    copy(source_repo, target_repo).await?;
                },
                DbType::Postgres(postgres) => {
                    let target_repo = get_postgres_repo(postgres).await?;
                    copy(source_repo, target_repo).await?;
                }
            }

        },
    }

     Ok(())

}