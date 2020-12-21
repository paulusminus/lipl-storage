pub use uuid::Uuid;
use bs58::{decode, encode};
use std::path::Path;
use crate::model::{LiplError, LiplResult};

pub trait UuidExt {
    fn to_base58(&self) -> String;
    fn from_base58<P: AsRef<Path>>(p: P) -> Uuid;
    fn try_from_base58<P: AsRef<Path>>(p: P) -> LiplResult<Uuid>;
}

impl UuidExt for Uuid {
    fn to_base58(&self) -> String {
        let bytes = self.as_bytes();
        encode(bytes).into_string()
    }

    fn from_base58<P: AsRef<Path>>(p: P) -> Self {
        Uuid::try_from_base58(p).unwrap()
    }

    fn try_from_base58<P: AsRef<Path>>(p: P) -> LiplResult<Uuid> {
        let mut decoded = [0xFF; 16];

        decode(
            p.as_ref().file_stem()
            .ok_or_else(
                || LiplError::NoPath(p.as_ref().to_path_buf()))?
            .to_string_lossy().to_string().as_str()
        )
        .into(&mut decoded)?;

        let uuid = Uuid::from_slice(&decoded)?;
        Ok(uuid)
    }
}

pub mod serde_uuid {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use crate::model::{Uuid, UuidExt};
    
    pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.to_base58().serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Uuid::try_from_base58(s).map_err(D::Error::custom)
    }    
}

pub mod serde_vec_uuid {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use crate::model::{Uuid, UuidExt};
    
    pub fn serialize<S>(val: &[Uuid], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let result: Vec<String> = val.iter().map(|uuid| uuid.to_base58()).collect() ;
        result.serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Uuid>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: Vec<String> = Deserialize::deserialize(deserializer)?;
        let mut result: Vec<Uuid> = vec![];
        for s in val {
            result.push(Uuid::try_from_base58(s).map_err(D::Error::custom)?);
        }
        Ok(result)
    }   
}