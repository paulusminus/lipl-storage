use futures_util::future::try_join_all;
use lipl_repo_redis::{RedisRepoConfig, new_lyric, new_playlist};
use lipl_core::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let db = RedisRepoConfig::default().to_repo().await?;

    let lyrics = try_join_all([
        db.upsert_lyric(
            new_lyric("Roodkapje", "Zeg roodkapje waar ga je hene")
        ),
        db.upsert_lyric(
            new_lyric("Daar bij die molen", "Ik zie de molen al versierd")
        ),
        db.upsert_lyric(
            new_lyric("Daar bij de waterkant", "Ik heb je voor het eerst ontmoet. Daar bij de waterkant")
        ),
        db.upsert_lyric(
            new_lyric("Sofietje", "Zij dronk ranja met een rietje. Mijn sofietje. Op een Amsterdams terras")
        ),
    ])
    .await?;

    let playlist = 
        db.upsert_playlist(new_playlist("Alles", lyrics.iter().map(|lyric| lyric.id).collect())).await?;

    for playlist in db.get_playlist_summaries().await? {
        tracing::info!("{playlist}");
    }

    for lyric in db.get_lyric_summaries().await? {
        tracing::info!("{lyric}");
    }

    tracing::info!("About to remove lyric with id {}", lyrics[2].id);
    db.delete_lyric(lyrics[2].id).await?;

    for lyric in db.get_lyric_summaries().await? {
        tracing::info!("{lyric}");
    }

    let modified_playlist = db.get_playlist(playlist.id).await?;
    tracing::info!("Playlist: {}", modified_playlist);

    Ok(())
}

