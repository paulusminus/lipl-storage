use std::io::Error;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncRead};
use tokio::stream::StreamExt;

pub async fn to_parts_async<T>(reader: BufReader<T>) -> Result<(Option<String>, Vec<Vec<String>>), Error>
where T: AsyncRead + Unpin
{
    let mut lines = reader.lines();
    let mut new_part = true;
    let mut result: Vec<Vec<String>> = vec![];
    let mut yaml: Option<String> = None;
    let mut yaml_start: bool = false;

    let mut line_no = 0;
    while let Some(line) = lines.next().await {
        line_no += 1;
        let line_result: String = line?;
        if line_result == *"---" {
            if line_no == 1 {
                yaml_start = true;
                yaml = Some("".to_owned());
                continue;
            }
            else if yaml_start {
                yaml_start = false;
                continue;
            }
        }

        if yaml_start {
            if let Some(v) = yaml.as_mut() {
                v.extend(vec![line_result, "\n".to_owned()]);
            }
            continue;
        }
        
        if line_result.trim().is_empty() {
            new_part = true;
            continue;
        }
        
        if new_part {
            result.push(vec![line_result.trim().to_owned()]);
            new_part = false;
            continue;
        }

        result.last_mut().unwrap().push(line_result.trim().to_owned());
    }

    Ok(
        (
            yaml,
            result,
        )
    )
}
