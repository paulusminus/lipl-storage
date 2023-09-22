use lipl_core::{LiplRepo, Lyric, LyricPost, Playlist, PlaylistPost};
use lipl_storage_postgres::{PostgresRepo, PostgresRepoConfig};

const ROODKAPJE: &str = include_str!("./Roodkapje.md");
const MOLEN: &str = include_str!("./Molen.md");
const SINTERKLAAS: &str = include_str!("./Sinterklaas.md");

fn create_lyric(text: &str) -> Lyric {
    (None, text.parse::<LyricPost>().unwrap()).into()
}

#[tokio::test]
async fn test_lyric() -> Result<(), Box<dyn std::error::Error>> {
    let repo_config = match std::env::var("GITHUB_TOKEN") {
        Ok(_) => {
            let host = std::env::var("POSTGRES_HOST").unwrap();
            let db = std::env::var("POSTGRES_DB").unwrap();
            let user = std::env::var("POSTGRES_USER").unwrap();
            let password = std::env::var("POSTGRES_PASSWORD").unwrap();
            format!("host={host} user={user} password={password} dbname={db}")
                .parse::<PostgresRepoConfig>()?
                .clear(true)
        }
        Err(_) => {
            dotenv::from_filename("local.env").ok();
            let host = std::env::var("POSTGRES_HOST").unwrap();
            let db = std::env::var("POSTGRES_DB").unwrap();
            format!("host={host} dbname={db}")
                .parse::<PostgresRepoConfig>()?
                .clear(true)
        }
    };
    let repo = PostgresRepo::new(repo_config).await?;

    let lyric1 = create_lyric(ROODKAPJE);

    let lyric1_posted = repo.upsert_lyric(lyric1.clone()).await?;
    assert_eq!(lyric1.id, lyric1_posted.id);

    let lyric2: Lyric = create_lyric(MOLEN);
    let posted_lyric2 = repo.upsert_lyric(lyric2.clone()).await?;
    assert_eq!(lyric2.id, posted_lyric2.id);

    let lyric3: Lyric = create_lyric(SINTERKLAAS);
    let posted_lyric3 = repo.upsert_lyric(lyric3.clone()).await?;
    assert_eq!(lyric3.id, posted_lyric3.id);

    let mut count = repo.get_lyric_summaries().await?.len();
    assert_eq!(count, 3);

    repo.delete_lyric(posted_lyric2.id).await?;
    count = repo.get_lyric_summaries().await?.len();
    assert_eq!(count, 2);

    let summaries: Vec<String> = repo
        .get_lyric_summaries()
        .await?
        .into_iter()
        .map(|s| s.title)
        .collect();
    assert_eq!(
        summaries,
        vec!["Roodkapje".to_string(), "Sinterklaas".to_string()]
    );

    let detail = repo.get_lyric(lyric3.id).await?;
    assert_eq!(
        detail.parts[0][0],
        "Zie ginds komt de stoomboot uit Spanje weer aan".to_owned()
    );

    let lyric4: Lyric = (
        None,
        LyricPost {
            title: "Sinterklaas".to_owned(),
            parts: vec![],
        },
    )
        .into();
    let failed_insert = repo.upsert_lyric(lyric4).await;
    assert_eq!(failed_insert.is_ok(), false);

    let playlist: Playlist = (
        None,
        PlaylistPost {
            title: "Alles".to_owned(),
            members: vec![lyric3.id.clone(), lyric1.id.clone()],
        },
    )
        .into();

    let playlist_posted = repo.upsert_playlist(playlist.clone()).await?;
    assert_eq!(playlist_posted.members, vec![lyric3.id, lyric1.id]);

    let playlist_retrieved1 = repo.get_playlist(playlist.id).await?;
    assert_eq!(playlist_retrieved1.members, vec![lyric3.id, lyric1.id]);

    let mut playlist2 = playlist.clone();
    playlist2.title = "Diversen".to_owned();

    repo.upsert_playlist(playlist2).await?;

    Ok(())
}
