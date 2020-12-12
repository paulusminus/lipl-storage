use std::io::{Read, BufRead, BufReader};

use uuid::Uuid;
use crate::model;
use crate::model::{Frontmatter, LiplResult, Lyric};

pub fn get_lyric(reader: impl Read, id: Uuid) -> LiplResult<Lyric> {
    let async_reader = BufReader::new(reader);
    let (yaml, parts) = parts_from_reader(async_reader)?;

    let frontmatter = 
        yaml
        .and_then(|text| serde_yaml::from_str::<Frontmatter>(&text).ok())
        .unwrap_or_default();

    Ok(
        model::Lyric {
            id,
            title: frontmatter.title,
            parts,
        }
    )
}

pub fn parts_from_reader<R: Read>(reader: BufReader<R>) -> LiplResult<(Option<String>, Vec<Vec<String>>)>
{
    let mut lines = reader.lines();
    let mut new_part = true;
    let mut result: Vec<Vec<String>> = vec![];
    let mut yaml: Option<String> = None;
    let mut yaml_start: bool = false;

    let mut line_no: u32 = 0;
    while let Some(line) = lines.next() {
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
