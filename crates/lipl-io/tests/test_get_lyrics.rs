use lipl_io::model::{Db, Persist};
use lipl_core::{Lyric};

const DIR_NAME: &str = "./tests/fs/";

#[test]
fn test_get_lyrics() {
    let mut db = Db::new(DIR_NAME.into());
    db.load().unwrap();
    let mut lyrics: Vec<Lyric> = db.get_lyric_list();
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
        "Whatever".to_owned(),
        song1.title,
    );

    assert_eq!(
        song1.id.to_string(),
        "2SQ3bh2LfXfcTbbHqyRjF5".to_owned(),
    );

    
    assert_eq!(
        song2.id.to_string(),
        "7yyNirdwBpAh3BdoGxvJ25".to_owned(),
    );

    assert_eq!(
        song2.title,
        "".to_owned(),
    );
}
