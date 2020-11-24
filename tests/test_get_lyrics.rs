use futures::StreamExt;
use lipl_io::{get_lyrics};

const DIR_NAME: &str = "./tests/fs/";

#[tokio::test]
async fn test_get_lyrics() -> Result<(), Box<dyn std::error::Error>> {
    let stream = get_lyrics(DIR_NAME).await?;

    tokio::pin!(stream);
    let song2 = stream.next().await.unwrap();
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
        song1.id.to_string(),
        "0ba4ef4d-0ce3-41d0-ac81-605ad1ae9358".to_owned(),
    );

    
    assert_eq!(
        song2.id.to_string(),
        "388b39a0-9acc-4cf0-98cb-a3d2035ecc3a".to_owned(),
    );

    assert_eq!(
        song2.title,
        None,
    );

    Ok(())
}
