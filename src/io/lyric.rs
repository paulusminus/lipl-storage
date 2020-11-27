use std::fs::{read_dir, File};
use std::io::Error;
use futures::future::ready;
use futures::io::{AllowStdIo, AsyncRead, AsyncBufReadExt, BufReader};
use futures::stream::{Stream, StreamExt, iter};

use uuid::Uuid;
use crate::model;
use super::fs::get_fs_files;
use crate::model::PathBufExt;

pub async fn get_lyrics(path: &str) -> Result<impl Stream<Item=model::Lyric>, Error> {
    read_dir(path)
    .map(|list|
        iter(get_fs_files(list, "txt"))
        .then(|path_buffer| async move {
            get_lyric(File::open(&path_buffer)?, path_buffer.to_uuid()).await
        })
        .filter_map(|lyric_file| ready(lyric_file.ok()))
    )
}

pub async fn get_lyric(file: impl std::io::Read, id: Uuid) -> Result<model::Lyric, Error> {
    let reader = AllowStdIo::new(file);
    let async_reader = BufReader::new(reader);
    let (yaml, parts) = to_parts_async(async_reader).await?;

    let frontmatter = 
        yaml
        .and_then(|text| serde_yaml::from_str::<model::Frontmatter>(&text).ok())
        .unwrap_or_default();

    Ok(
        model::Lyric {
            id,
            title: frontmatter.title,
            parts,
        }
    )
}

pub async fn to_parts_async<T>(reader: BufReader<T>) -> Result<(Option<String>, Vec<Vec<String>>), Error>
where T: AsyncRead + Unpin
{
    let mut lines = reader.lines();
    let mut new_part = true;
    let mut result: Vec<Vec<String>> = vec![];
    let mut yaml: Option<String> = None;
    let mut yaml_start: bool = false;

    let mut line_no: u32 = 0;
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
