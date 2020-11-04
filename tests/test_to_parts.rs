extern crate lipl_io;

use tokio::io::BufReader;
use tokio::fs::File;
use lipl_io::to_parts_async;

const FILE_NAME: &str = "./tests/fs/2SQ3bh2LfXfcTbbHqyRjF5";

async fn get_data() -> BufReader<File> {
    let file = File::open(FILE_NAME).await.unwrap();
    BufReader::new(file)
}

#[tokio::test]
async fn test_to_parts() -> Result<(), Box<dyn std::error::Error>> {
    let result = to_parts_async(get_data().await).await?;

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
