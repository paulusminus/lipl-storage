pub struct Markdown {
    pub frontmatter: Option<String>,
    pub parts: Vec<Vec<String>>,
}

impl From<String> for Markdown {
    fn from(s: String) -> Self {
        parse_markdown(s, "---")
    }
}

fn parse_markdown(text: String, yaml_separator: &str) -> Markdown {
    let parts = to_parts(&text);
    if !parts.is_empty() && !parts[0].is_empty() && parts[0][0] == *yaml_separator {
        Markdown {
            frontmatter: Some(
                parts[0]
                    .clone()
                    .into_iter()
                    .filter(|s| s != yaml_separator)
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            parts: parts[1..].to_vec(),
        }
    } else {
        Markdown {
            frontmatter: None,
            parts,
        }
    }
}

pub fn to_parts(input: impl AsRef<str>) -> Vec<Vec<String>> {
    input
        .as_ref()
        .lines()
        .map(|s| s.trim_end().to_string())
        .collect::<Vec<_>>()
        .split(String::is_empty)
        .map(Into::into)
        .filter(|p: &Vec<String>| !p.is_empty())
        .collect()
}

pub fn to_text(parts: &[Vec<String>]) -> String {
    parts
        .iter()
        .map(|lines| lines.join("\n"))
        .collect::<Vec<String>>()
        .join("\n\n")
}

#[cfg(test)]
mod test {
    #[test]
    fn test_to_parts() {
        let test = "Hallo\nAllemaal\r\n\nWat fijn  \n\r\n";
        let result = super::to_parts(test);
        assert_eq!(result.len(), 2);
        assert_eq!(&result[0][0], "Hallo");
        assert_eq!(&result[0][1], "Allemaal");
        assert_eq!(result[0].len(), 2);
        assert_eq!(&result[1][0], "Wat fijn");
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_parse_markdown() {
        let test = "---\nyaml: is_fine\n---\n\nAllemaal\r\n\nWat fijn  \n\r\n".to_owned();
        let result = super::parse_markdown(test, "---");
        assert_eq!(result.parts, vec![vec!["Allemaal"], vec!["Wat fijn"]]);
        assert_eq!(result.frontmatter, Some("yaml: is_fine".to_owned()))
    }

    #[test]
    fn test_parse_markdown_no_content() {
        let test = "---\nyaml: is_fine\n---".to_owned();
        let result = super::parse_markdown(test, "---");
        assert!(result.parts.is_empty());
        assert_eq!(result.frontmatter, Some("yaml: is_fine".to_owned()))
    }
}
