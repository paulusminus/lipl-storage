use std::default::Default;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use bs58::{decode, encode};
use crate::model::{LiplError};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr, SerializeDisplay)]
pub struct Uuid(uuid::Uuid);

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let bytes = self.0.as_bytes();
        write!(f, "{}", encode(bytes).into_string())
    }
}

impl FromStr for Uuid {
    type Err = LiplError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut decoded = [0xFF; 16];
        decode(s).into(&mut decoded)?;
        let uuid = uuid::Uuid::from_slice(&decoded)?;
        Ok(Uuid(uuid))
    }
}

impl Default for Uuid {
    fn default() -> Self {
        let uuid = uuid::Uuid::new_v4();
        Uuid(uuid)
    }
}

// pub mod serde_uuid {
//     use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
//     use crate::model::{Uuid};
    
//     pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         val.to_string().serialize(serializer)
//     }
    
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s: &str = Deserialize::deserialize(deserializer)?;
//         s.parse::<Uuid>().map_err(D::Error::custom)
//     }    
// }

// pub mod serde_vec_uuid {
//     use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
//     use crate::model::{Uuid};
    
//     pub fn serialize<S>(val: &[Uuid], serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let result: Vec<String> = val.iter().map(|uuid| uuid.to_string()).collect() ;
//         result.serialize(serializer)
//     }
    
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Uuid>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let val: Vec<String> = Deserialize::deserialize(deserializer)?;
//         let mut result: Vec<Uuid> = vec![];
//         for s in val {
//             result.push(s.parse::<Uuid>().map_err(D::Error::custom)?);
//         }
//         Ok(result)
//     }   
// }