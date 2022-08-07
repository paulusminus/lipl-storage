use std::str::FromStr;

use lipl_postgres_repo::{Dal, pool::get, ConnectionBuilder};
use lipl_types::{LiplRepo, LyricPost, Lyric, Playlist, PlaylistPost};

const CONNECTION: &str = "host=/var/run/postgresql/ user=paul dbname=test";
const ROODKAPJE: &str = include_str!("./Roodkapje.md");
const MOLEN: &str = include_str!("./Molen.md");
const SINTERKLAAS: &str = include_str!("./Sinterklaas.md");

fn create_lyric(text: &str) -> Lyric {
    Lyric::from(
        (
            None,
            LyricPost::from_str(text).unwrap(),
        )
    )
}

#[tokio::test]
async fn test_lyric() -> Result<(), Box<dyn std::error::Error>> {

    let connection = 
        ConnectionBuilder::default()
        .host("/var/run/postgresql/")
        .user("paul")
        .dbname("test")
        .build()?;

    assert_eq!(connection.to_string().as_str(), CONNECTION);
    let pool = get(&connection.to_string(), 16)?;
    let dal = Dal::new(pool, true).await?;

    let lyric1 = create_lyric(ROODKAPJE);

    let lyric1_posted = 
        dal.post_lyric(lyric1.clone()).await?;
    assert_eq!(lyric1.id, lyric1_posted.id);

    let lyric2: Lyric = create_lyric(MOLEN);
    let posted_lyric2 = dal.post_lyric(lyric2.clone()).await?;
    assert_eq!(lyric2.id, posted_lyric2.id);

    let lyric3: Lyric = create_lyric(SINTERKLAAS);
    let posted_lyric3 = dal.post_lyric(lyric3.clone()).await?;
    assert_eq!(lyric3.id, posted_lyric3.id);

    let mut count = dal.get_lyric_summaries().await?.len();
    assert_eq!(count, 3);

    dal.delete_lyric(posted_lyric2.id).await?;
    count = dal.get_lyric_summaries().await?.len();
    assert_eq!(count, 2);

    let summaries: Vec<String> = dal.get_lyric_summaries().await?.into_iter().map(|s| s.title).collect();
    assert_eq!(summaries, vec!["Roodkapje".to_string(), "Sinterklaas".to_string()]);

    let detail = dal.get_lyric(lyric3.id).await?;
    assert_eq!(detail.parts[0][0], "Zie ginds komt de stoomboot uit Spanje weer aan".to_owned());

    let lyric4: Lyric = (
        None,
        LyricPost {
            title: "Sinterklaas".to_owned(),
            parts: vec![],
        }
    )
    .into();
    let failed_insert = dal.post_lyric(lyric4).await;
    assert_eq!(failed_insert.is_ok(), false);

    let playlist: Playlist = (
        None,
        PlaylistPost { 
            title: "Alles".to_owned(), 
            members: vec![
                lyric3.id.clone(),
                lyric1.id.clone(),
            ]
        }
    )
    .into();

    let playlist_posted = dal.post_playlist(playlist.clone()).await?;
    assert_eq!(playlist_posted.members, vec![lyric3.id, lyric1.id]);

    let playlist_retrieved1 = dal.get_playlist(playlist.id).await?;
    assert_eq!(playlist_retrieved1.members, vec![lyric3.id, lyric1.id]);

    let mut playlist2 = playlist.clone();
    playlist2.title = "Diversen".to_owned();

    dal.post_playlist(playlist2).await?;

    Ok(())
}
