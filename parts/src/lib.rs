use lazy_static::lazy_static;
use regex::Regex;

mod st;
pub use st::to_parts_async;

const DOUBLE_LINE: &str = r"\n\s*\n";

lazy_static! {
    static ref DOUBLE_LINE_REGEX: Regex = DOUBLE_LINE.parse().unwrap();
}

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
    let parts = to_parts(text);
    if 
        !parts.is_empty()
        && !parts[0].is_empty()
        && parts[0][0] == yaml_separator.to_owned() 
    {
        Markdown {
            frontmatter: Some(
                parts[0]
                .clone()
                .into_iter()
                .filter(|s| s != yaml_separator)
                .collect::<Vec<_>>()
                .join("\n")
            ),
            parts: parts[1..].to_vec(),
        }
    }
    else {
        Markdown {
            frontmatter: None,
            parts
        }
    }
}

fn trim_end(s: &str) -> &str {
    s.trim_end()
}

fn to_lines(s: &str) -> Vec<String> {
    s.split('\n')
        .map(trim_end)
        .filter(|&s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn to_parts(s: String) -> Vec<Vec<String>> {
    DOUBLE_LINE_REGEX
    .split(&s)
    .map(to_lines)
    .filter(|p| !p.is_empty())
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
    fn test_trim() {
        let test = "\rHallo allema  \t";
        assert_eq!(super::trim_end(test), "\rHallo allema");
    }

    #[test]
    fn test_to_lines() {
        let test = "Hallo\nAllemaal\r\nWat fijn  \n\r\n";
        let result = super::to_lines(test);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "Hallo");
        assert_eq!(result[1], "Allemaal");
        assert_eq!(result[2], "Wat fijn");
    }

    #[test]
    fn test_to_parts() {
        let test = "Hallo\nAllemaal\r\n\nWat fijn  \n\r\n".to_owned();
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
