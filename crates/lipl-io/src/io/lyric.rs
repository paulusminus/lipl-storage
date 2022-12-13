use std::io::{Read, BufRead, BufReader};
use lipl_core::{LyricMeta, LyricPost};

pub fn lyricpost_from_reader<R>(reader: R) -> crate::Result<LyricPost>
where R: Read
{
    let buf_reader = BufReader::new(reader);
    let lines = buf_reader.lines();
    let mut new_part = true;
    let mut parts: Vec<Vec<String>> = vec![];
    let mut yaml: Option<String> = None;
    let mut yaml_start: bool = false;

    for (line_no, line) in lines.enumerate() {
        let line_result = line?;
        if line_result == *"---" {
            if line_no == 0 {
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
                v.extend(vec![line_result, "\n".into()]);
            }
            continue;
        }
        
        if line_result.trim().is_empty() {
            new_part = true;
            continue;
        }
        
        if new_part {
            parts.push(vec![line_result.trim().into()]);
            new_part = false;
            continue;
        }

        parts.last_mut().unwrap().push(line_result.trim().into());
    }

    let frontmatter = 
        yaml
        .and_then(|text| 
            serde_yaml::from_str::<LyricMeta>(&text).ok()
        );

    Ok(
        LyricPost {
            title: frontmatter.map(|f| f.title).unwrap_or_default(),
            parts,
        }
    )
}
