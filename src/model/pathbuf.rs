use std::path::Path;
use crate::model::{LiplResult, Uuid, UuidExt};

pub trait PathBufExt {
    fn to_uuid(&self) -> Uuid;
    fn try_to_uuid(&self) -> LiplResult<Uuid>;
}

impl<T> PathBufExt for T where T: AsRef<Path> {
    fn to_uuid(&self) -> Uuid {
        self.try_to_uuid().unwrap()
    }

    fn try_to_uuid(&self) -> LiplResult<Uuid> {
        Uuid::try_from_base58(&self)
    }
}
