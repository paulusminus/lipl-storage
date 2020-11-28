use std::io::{Error as IOError, ErrorKind};
use std::path::PathBuf;

pub fn get_path() -> Result<PathBuf, IOError> {
    let mut args = std::env::args();

    if args.len() < 2 {
        return Err(IOError::new(ErrorKind::Other, "Argument directory missing"));
    }

    let path_str = args.nth(1).ok_or_else(|| std::io::Error::new(ErrorKind::Other, "Cannot parse argument 1"))?;
    let path = PathBuf::from(&path_str);
    if !path.exists() {
        return Err(IOError::new(ErrorKind::Other, "Directory not found"));
    }

    Ok(path)
}
