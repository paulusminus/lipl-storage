use tokio::stream::StreamExt;
use lipl_io::{get_lyrics, Lyric};

const DIR_NAME: &str = "./tests/fs/";

#[tokio::test]
async fn test_get_lyrics() -> Result<(), Box<dyn std::error::Error>> {
    let stream = get_lyrics(DIR_NAME).await?;
    let songs: Vec<Lyric> = stream.collect().await;

    assert_eq!(
        vec![
            vec![
                "Hallo allemaal".to_owned(),
                "Wat fijn dat u er bent".to_owned(),
            ],
            vec![
                "En dan ook nog".to_owned(),
                "een tweede couplet".to_owned()
            ]
        ],
        songs[0].parts,
    );

    assert_eq!(
        Some("title: Whatever  \nmembers: [Kerst, Kinderliedjes]\n".to_owned()),
        songs[0].yaml,
    );

    assert_eq!(
        songs[0].id,
        "test",
    );

    assert_eq!(
        songs[1].id,
        "test2",
    );

    assert_eq!(
        songs[1].yaml,
        None,
    );

    Ok(())
}
