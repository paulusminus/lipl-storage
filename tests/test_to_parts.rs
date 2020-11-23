extern crate lipl_io;

use futures::io::{AllowStdIo, BufReader};
use std::fs::File;
use lipl_io::to_parts_async;

const FILE_NAME: &str = "./tests/fs/2SQ3bh2LfXfcTbbHqyRjF5";

#[tokio::test]
async fn test_to_parts() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(FILE_NAME).unwrap();
    let test = AllowStdIo::new(file);
    let reader = BufReader::new(test);
    let result = to_parts_async(reader).await?;

    assert_eq!(
        result.1,
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
        result.0,
        Some("title: Whatever  \nmember_of:\n  - Kerst\n  - Kinderliedjes\n".to_owned()),
    );

    Ok(())
}
