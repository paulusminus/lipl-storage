use std::path::{Path};
use std::io::{Error, ErrorKind};

pub fn parse_command_line() -> Result<String, Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "Te weinig parameters!"));
    }
    let path = &args[1];
    if !Path::new(&path).exists() {
        return Err(Error::new(ErrorKind::NotFound, "Directory bestaat niet"));
    }
    Ok(path.clone())
}
