use uuid::Uuid;
// use std::path::{PathBuf};
use std::path::Path;
use bs58::decode;
use bs58::decode::Error;

pub trait PathBufExt {
    fn to_uuid(&self) -> Uuid;
    fn try_to_uuid(&self) -> Result<Uuid, Error>;
}

impl<T> PathBufExt for T where T: AsRef<Path> {
    fn to_uuid(&self) -> Uuid {
        let mut decoded = [0xFF; 16];
        decode(self.as_ref().file_stem().unwrap().to_string_lossy().to_string().as_str()).into(&mut decoded).unwrap();
        uuid::Uuid::from_slice(&decoded).unwrap() 
    }

    fn try_to_uuid(&self) -> Result<Uuid, Error> {
        let mut decoded = [0xFF; 16];
        decode(self.as_ref().file_stem().unwrap().to_string_lossy().to_string().as_str()).into(&mut decoded)?;
        Uuid::from_slice(&decoded).map_err(|_| Error::BufferTooSmall)
    }
}
