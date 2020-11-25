use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use bs58::{decode, encode};

pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bs58_encoded: String = encode(val.as_bytes()).into_string();
    bs58_encoded.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let val: &str = Deserialize::deserialize(deserializer)?;
    let mut decoded = [0xFF; 16];
    decode(val).into(&mut decoded).map_err(D::Error::custom)?;
    Uuid::from_slice(&decoded).map_err(D::Error::custom)
}
