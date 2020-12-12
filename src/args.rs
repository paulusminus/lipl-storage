use std::path::PathBuf;
use crate::model::{LiplResult, LiplError};

pub fn get_path() -> LiplResult<PathBuf> {
    let mut args = std::env::args();

    if args.len() < 2 {
        return Err(LiplError::Argument("Argument directory missing".to_owned()));
    }

    let path_str = args.nth(1).ok_or_else(|| LiplError::Argument("Cannot parse argument 1".to_owned()))?;
    let path = PathBuf::from(&path_str);
    if !path.exists() {
        return Err(LiplError::Argument("Directory or file missing!".to_owned()));
    }

    Ok(path)
}
