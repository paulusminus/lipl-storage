use tokio::stream::StreamExt;
use lipl_io::{get_lyrics};

const DIR_NAME: &str = "./tests/fs/";

#[tokio::test]
async fn test_get_lyrics() -> Result<(), Box<dyn std::error::Error>> {
    let stream = get_lyrics(DIR_NAME).await?;

    tokio::pin!(stream);
    let song1 = stream.next().await.unwrap();

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
        song1.parts,
    );

    assert_eq!(
        Some("Whatever".to_owned()),
        song1.title,
    );

    assert_eq!(
        Some(vec!["Kerst".to_owned(), "Kinderliedjes".to_owned()]),
        song1.member_of,
    );

    assert_eq!(
        song1.id,
        "a6223cbc-95f2-46ce-b444-08ea59ee3fd9".to_owned(),
    );

    let song2 = stream.next().await.unwrap();

    assert_eq!(
        song2.id,
        "b65e14d1-fb98-4df8-bfd3-48143371cb17".to_owned(),
    );

    assert_eq!(
        song2.title,
        None,
    );

    Ok(())
}
