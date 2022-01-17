extern crate lipl_io;

use std::fs::File;
use std::io::BufReader;
use lipl_io::io::lyricpost_from_reader;

const FILE_NAME: &str = "./tests/fs/2SQ3bh2LfXfcTbbHqyRjF5.txt";

#[test]
fn test_to_parts() {
    let file = File::open(FILE_NAME).unwrap();
    let reader = BufReader::new(file);
    let result = lyricpost_from_reader(reader).unwrap();

    assert_eq!(
        result.parts,
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
    );

    assert_eq!(
        result.title,
        "Whatever".to_owned(),
    );
}
