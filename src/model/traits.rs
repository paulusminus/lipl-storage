use super::{Summary};
use uuid::Uuid;
use std::path::Path;

pub trait HasId {
    fn id(&self) -> Uuid;
}

pub trait HasSummary {
    fn to_summary(&self) -> Summary;
}

pub trait FromPath {
    fn from_path<P: AsRef<Path>>(path_like: P) -> Result<Uuid, Box<dyn std::error::Error>>;
}

pub trait Db<T> {
    fn load(&self) -> Result<T, std::io::Error>;
    fn dump(t: T) -> Result<(), std::io::Error>;
}
