use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use bs58::{decode, encode};

pub fn serialize<S>(val: &Vec<Uuid>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let vec_value = val.iter().map(|uuid| encode(uuid.as_bytes()).into_string()).collect::<Vec<String>>();
    vec_value.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Uuid>, D::Error>
where
    D: Deserializer<'de>,
{
    let val: Vec<String> = Deserialize::deserialize(deserializer)?;
    let mut result: Vec<Uuid> = vec![];
    for s in val {
        let mut decoded = [0xFF; 16];
        decode(s).into(&mut decoded).map_err(D::Error::custom)?;
        let uuid = Uuid::from_slice(&decoded).map_err(D::Error::custom)?;
        result.push(uuid);
    }
    Ok(result)
}
