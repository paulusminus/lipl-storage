use uuid::Uuid;
use bs58::encode;

pub trait UuidExt {
    fn to_base58(&self) -> String;
}

impl UuidExt for Uuid {
    fn to_base58(&self) -> String {
        let bytes = self.as_bytes();
        encode(bytes).into_string()
    }
}
