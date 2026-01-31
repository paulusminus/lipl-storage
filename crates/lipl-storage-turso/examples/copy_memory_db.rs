use lipl_core::{Playlist, Repo, RepoConfig, Uuid};
use lipl_storage_turso::{TursoConfig, TursoDatabase};

pub const TEST_DATABASE_NAME: &str = "lipl.sqlite";

async fn create_database() -> TursoDatabase {
    let config = TursoConfig::from(TEST_DATABASE_NAME.to_owned());
    let repo = config.to_repo().await.unwrap();
    repo.schema().await.unwrap();
    repo
}

#[tokio::main]
async fn main() {
    let memory_repo = lipl_storage_memory::MemoryRepoConfig {
        sample_data: true,
        transaction_log: None,
    }
    .to_repo()
    .await
    .unwrap();

    let turso_repo = create_database().await;
    turso_repo.clear().await.unwrap();

    // Copy data from memory to Turso
    for lyric in memory_repo.get_lyrics().await.unwrap() {
        dbg!(&lyric);
        turso_repo.upsert_lyric(lyric).await.unwrap();
    }

    for playlist in memory_repo.get_playlists().await.unwrap() {
        dbg!(&playlist);
        turso_repo.upsert_playlist(playlist).await.unwrap();
    }

    let playlists = turso_repo.get_playlists().await.unwrap();
    dbg!(playlists.first());
    assert!(!playlists.is_empty());

    let id = Uuid::default();
    let playlist = Playlist {
        id,
        title: "New Playlist".to_string(),
        members: vec![],
    };
    turso_repo.upsert_playlist(playlist).await.unwrap();

    let playlist = turso_repo.get_playlist(id).await.unwrap();
    assert_eq!(playlist.title, *"New Playlist");
}
