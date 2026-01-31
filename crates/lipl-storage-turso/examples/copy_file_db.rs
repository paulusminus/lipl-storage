use lipl_core::{Repo, RepoConfig};
use lipl_storage_fs::FileRepoConfig;
use lipl_storage_turso::{TursoConfig, TursoDatabase};

pub const TEST_DATABASE_NAME: &str = "lipl.sqlite";
pub const DATA_DIR: &str = "data";

async fn create_database() -> TursoDatabase {
    let config = TursoConfig::from(TEST_DATABASE_NAME.to_owned());
    let repo = config.to_repo().await.unwrap();
    repo.schema().await.unwrap();
    repo
}

#[tokio::main]
async fn main() {
    let file_repo = DATA_DIR
        .parse::<FileRepoConfig>()
        .unwrap()
        .to_repo()
        .await
        .unwrap();

    let turso_repo = create_database().await;
    turso_repo.clear().await.unwrap();

    // Copy data from memory to Turso
    for lyric in file_repo.get_lyrics().await.unwrap() {
        dbg!(&lyric);
        turso_repo.upsert_lyric(lyric).await.unwrap();
    }

    for playlist in file_repo.get_playlists().await.unwrap() {
        dbg!(&playlist);
        turso_repo.upsert_playlist(playlist).await.unwrap();
    }
}
