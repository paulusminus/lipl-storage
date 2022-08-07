use regex::Regex;
use lazy_static::lazy_static;

const SINGLE_LINE: &str = r"\n";
const DOUBLE_LINE: &str = r"\n\s*\n";

lazy_static! {
    static ref SINGLE_LINE_REGEX: Regex = SINGLE_LINE.parse().unwrap();
    static ref DOUBLE_LINE_REGEX: Regex = DOUBLE_LINE.parse().unwrap();
}

fn to_lines(s: &str) -> Vec<String> {
    SINGLE_LINE_REGEX
    .split(s)
    .map(|line| line.trim().to_owned())
    .filter(|line| !line.is_empty())
    .collect()
}

fn to_parts(s: &str) -> Vec<Vec<String>> {
    DOUBLE_LINE_REGEX.split(s)
    .map(to_lines)
    .filter(|p| !p.is_empty())
    .collect()
}

pub trait StringExt {
    fn to_parts(&self) -> Vec<Vec<String>>;
}

impl StringExt for String {
    fn to_parts(&self) -> Vec<Vec<String>> {
        to_parts(self)
    }
}


#[cfg(test)]
mod test {

    const INPUT: &str = "\nHier\n gebeurt\n het ook wel\n \n 's een keer\n\t\n";

    #[test]
    fn test_to_lines() {
        let output = super::to_lines(INPUT);
        assert_eq!(output[0], "Hier");
        assert_eq!(output[1], "gebeurt");
        assert_eq!(output.len(), 4);
    }

    #[test]
    fn test_to_parts() {
        let output = super::to_parts(INPUT);
        assert_eq!(output.len(), 2);
        assert_eq!(output[0][0], "Hier");
        assert_eq!(output[0][1], "gebeurt");
        assert_eq!(output[1][0], "'s een keer");
    }
}