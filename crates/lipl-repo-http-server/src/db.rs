use std::sync::Arc;

use lipl_core::{LiplRepo, RepoDb};
use tracing::{info};

pub async fn list(repo: Arc<dyn LiplRepo>, yaml: bool) -> anyhow::Result<()>
{
    let db = RepoDb {
        lyrics: repo.get_lyrics().await?,
        playlists: repo.get_playlists().await?,
    };

    println!("{}", if yaml { db.to_yaml().unwrap() } else { db.to_string() }) ;
    Ok(())
}

pub async fn copy(source: Arc<dyn LiplRepo>, target: Arc<dyn LiplRepo>) -> anyhow::Result<()>
{
    for lyric in source.get_lyrics().await? {
        info!("Copying lyric {} with id {}", lyric.title, lyric.id);
        target.post_lyric(lyric).await.unwrap();
    }

    for playlist in source.get_playlists().await? {
        info!("Copying playlist {} with id {}", playlist.title, playlist.id);
        target.post_playlist(playlist).await.unwrap();
    }

    Ok(())
}
