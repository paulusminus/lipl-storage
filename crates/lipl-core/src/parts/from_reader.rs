use std::io::{BufRead, BufReader, Read, Error};

pub struct Parts(Vec<Vec<String>>);

impl From<Vec<Vec<String>>> for Parts {
    fn from(value: Vec<Vec<String>>) -> Self {
        Parts(value)
    }
}

impl Parts {
    pub fn parts(&self) -> Vec<Vec<String>> {
        self.0.clone()
    }
}

fn lines<R>(r: R) -> impl Iterator<Item = Result<String, Error>>
where
    R: Read,
{
    BufReader::new(r).lines()
}

fn is_empty(b: bool) -> impl Fn(&Result<String, Error>) -> bool {
    move |r| r.as_ref().ok().map(|s| s.is_empty()) == Some(b)
}

fn next_part(lines: impl Iterator<Item = Result<String, Error>>) -> Result<Vec<String>, std::io::Error> {
    lines
    .map(|line| line.map(|l| l.trim().to_owned()))
    .skip_while(is_empty(true))
    .take_while(is_empty(false))
    .collect::<Result<Vec<String>, Error>>()
}

pub fn parts_from_reader<R>(r: R) -> Result<Parts, Error>
where
    R: Read,
{
    let mut lines = lines(r);
    let mut result = vec![];
    let mut part = next_part(&mut lines)?;
    while !part.is_empty() {
        result.push(part);
        part = next_part(&mut lines)?;
    }

    Ok(result.into())
}

#[cfg(test)]
mod test {
    use super::parts_from_reader;

    #[test]
    fn to_parts1() {
        let test = "\r  Hallo allema  \t";
        assert_eq!(
            parts_from_reader(test.as_bytes()).unwrap().parts(),
            vec![vec!["Hallo allema".to_owned()]]
        );
    }

    #[test]
    fn to_parts2() {
        let test = "\rHallo allema\n \t\nJaJa\r\nNee  \t";
        assert_eq!(
            parts_from_reader(test.as_bytes()).unwrap().parts(),
            vec![
                vec![
                    "Hallo allema".to_owned()],
                vec![
                    "JaJa".to_owned(),
                    "Nee".to_owned()
                ]
            ]
        );
    }
}