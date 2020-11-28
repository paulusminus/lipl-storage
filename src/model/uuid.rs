pub use uuid::Uuid;
use std::path::Path;

use super::traits::FromPath;
use super::PathBufExt;

impl FromPath for Uuid {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Uuid, Box<dyn std::error::Error>> {
        Ok(path.as_ref().try_to_uuid()?)
    }
}
