use futures::Stream;
use futures::stream::StreamExt;
use std::io::Error;

pub async fn to_parts_async(mut s: impl Stream<Item=Result<String, Error>> + Unpin) -> Result<Vec<Vec<String>>, Error>
{
    let mut new_part = true;
    let mut result: Vec<Vec<String>> = vec![];
    while let Some(line) = s.next().await {
        let trimmed: String = line?.trim().into();
        if trimmed.is_empty() {
            new_part = true;
        }
        else if new_part {
            result.push(vec![trimmed]);
            new_part = false;
        }
        else if let Some(last) = result.last_mut() {
            last.push(trimmed);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use futures::io::{AsyncBufReadExt, BufReader, Cursor, Error};
    use futures::{Stream};
    use super::to_parts_async;

    fn get_data() -> impl Stream<Item=Result<String, Error>> + Unpin {
        const BUFFER: &[u8] = b"Hallo allemaal\r\n  Wat fijn dat u er bent\t\n\n En dan ook nog\neen tweede couplet";
        let cursor = Cursor::new(BUFFER);
        let line_stream = BufReader::new(cursor).lines();
        line_stream
    }

    #[tokio::test]
    async fn test_async_to_parts() {
        assert_eq!(
            to_parts_async(&mut get_data()).await.unwrap(),
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
    }
}
