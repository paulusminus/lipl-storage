extern crate lipl_io;

use tokio::io::BufReader;
use tokio::fs::File;
use lipl_io::to_parts_async;

async fn get_data(filename: &str) -> BufReader<File> {
    let file = File::open(filename).await.unwrap();
    BufReader::new(file)
}

#[tokio::test]
async fn test_to_parts() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        to_parts_async(get_data("./tests/fs/test.txt").await).await?,
        (
            Some("title: Whatever  \nmembers: [Kerst, Kinderliedjes]\n".to_owned()),
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
        )
    );

    Ok(())
}
