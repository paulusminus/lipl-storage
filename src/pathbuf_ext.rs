use uuid::Uuid;
use std::path::{PathBuf};
use bs58::decode;

pub trait PathBufExt {
    fn to_uuid(&self) -> Uuid;
}

impl PathBufExt for PathBuf {
    fn to_uuid(&self) -> Uuid {
        let mut decoded = [0xFF; 16];
        decode(self.file_stem().unwrap().to_string_lossy().to_string().as_str()).into(&mut decoded).unwrap();
        uuid::Uuid::from_slice(&decoded).unwrap() 
    }
}
