use lipl_io::io::{fs_read};
use lipl_io::model::{LiplResult, Lyric};

const DIR_NAME: &str = "./tests/fs/";

#[test]
fn test_get_lyrics() -> LiplResult<()> {
    let db = fs_read(DIR_NAME)?;
    let mut lyrics: Vec<Lyric> = db.get_lyric_list().into_iter().cloned().collect();
    lyrics.sort_by(|a, b| a.id.cmp(&b.id));

    let song1 = &lyrics[0];
    let song2 = &lyrics[1];
    
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
