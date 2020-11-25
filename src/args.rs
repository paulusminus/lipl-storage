use std::io::{Error as IOError, ErrorKind};
use std::path::Path;

pub fn get_path() -> Result<String, IOError> {
    let mut args = std::env::args();

    if args.len() < 2 {
        return Err(IOError::new(ErrorKind::Other, "Argument directory missing"));
    }

    let path = args.nth(1).ok_or_else(|| std::io::Error::new(ErrorKind::Other, "Cannot parse argument 1"))?;
    if !Path::new(&path).exists() {
        return Err(IOError::new(ErrorKind::Other, "Directory not found"));
    }

    Ok(path)
}
