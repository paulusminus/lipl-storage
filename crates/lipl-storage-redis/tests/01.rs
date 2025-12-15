use futures_util::future::try_join_all;
use lipl_core::{Repo, ToRepo};
use lipl_storage_redis::{RedisRepoConfig, new_lyric, new_playlist};

#[tokio::test(flavor = "multi_thread")]
async fn main() {
    let db = RedisRepoConfig::default().to_repo().await.unwrap();

    let lyrics = try_join_all([
        db.upsert_lyric(new_lyric("Roodkapje", "Zeg roodkapje waar ga je hene")),
        db.upsert_lyric(new_lyric(
            "Daar bij die molen",
            "Ik zie de molen al versierd",
        )),
        db.upsert_lyric(new_lyric(
            "Daar bij de waterkant",
            "Ik heb je voor het eerst ontmoet. Daar bij de waterkant",
        )),
        db.upsert_lyric(new_lyric(
            "Sofietje",
            "Zij dronk ranja met een rietje. Mijn sofietje. Op een Amsterdams terras",
        )),
    ])
    .await
    .unwrap();

    let _playlist = db
        .upsert_playlist(new_playlist(
            "Alles",
            lyrics.iter().map(|lyric| lyric.id).collect(),
        ))
        .await
        .unwrap();

    assert_eq!(db.get_playlist_summaries().await.unwrap().len(), 1);

    assert_eq!(db.get_lyric_summaries().await.unwrap().len(), 4);

    db.delete_lyric(lyrics[2].id).await.unwrap();

    assert_eq!(db.get_lyric_summaries().await.unwrap().len(), 3);
}
