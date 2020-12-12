use uuid::Uuid;
use std::path::Path;
use bs58::decode;
use crate::model::LiplResult;

pub trait PathBufExt {
    fn to_uuid(&self) -> Uuid;
    fn try_to_uuid(&self) -> LiplResult<Uuid>;
}

impl<T> PathBufExt for T where T: AsRef<Path> {
    fn to_uuid(&self) -> Uuid {
        self.try_to_uuid().unwrap()
    }

    fn try_to_uuid(&self) -> LiplResult<Uuid> {
        let mut decoded = [0xFF; 16];
        decode(self.as_ref().file_stem().unwrap().to_string_lossy().to_string().as_str()).into(&mut decoded)?;
        let uuid = Uuid::from_slice(&decoded)?;
        Ok(uuid)
    }
}
