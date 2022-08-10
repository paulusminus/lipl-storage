use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter, Result};

mod st;
pub use st::to_parts_async;

const DOUBLE_LINE: &str = r"\n\s*\n";

lazy_static! {
    static ref DOUBLE_LINE_REGEX: Regex = DOUBLE_LINE.parse().unwrap();
}

struct Parts(Vec<Vec<String>>);

impl From<String> for Parts {
    fn from(s: String) -> Self {
        Parts(to_parts(s))
    }
}

impl Display for Parts {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", to_text(&self.0))
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
    use super::{to_lines, to_parts, trim_end, Parts};

    #[test]
    fn test_trim() {
        let test = "\rHallo allema  \t";
        assert_eq!(trim_end(test), "\rHallo allema");
    }

    #[test]
    fn test_to_lines() {
        let test = "Hallo\nAllemaal\r\nWat fijn  \n\r\n";
        let result = to_lines(test);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "Hallo");
        assert_eq!(result[1], "Allemaal");
        assert_eq!(result[2], "Wat fijn");
    }

    #[test]
    fn test_to_parts() {
        let test = "Hallo\nAllemaal\r\n\nWat fijn  \n\r\n".to_owned();
        let result = to_parts(test);
        assert_eq!(result.len(), 2);
        assert_eq!(&result[0][0], "Hallo");
        assert_eq!(&result[0][1], "Allemaal");
        assert_eq!(result[0].len(), 2);
        assert_eq!(&result[1][0], "Wat fijn");
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_to_text() {
        let test = "Hallo\nAllemaal\r\n\nWat fijn  \n\r\n".to_owned();
        let result = Parts::from(test).to_string();
        assert_eq!(result, "Hallo\nAllemaal\n\nWat fijn");
    }
}
