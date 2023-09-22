use crate::error::Error;
use bs58::{decode, encode};
use core::default::Default;
use core::str::FromStr;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(
    Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr, SerializeDisplay,
)]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn inner(&self) -> uuid::Uuid {
        self.0
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let bytes = self.0.as_bytes();
        write!(f, "{}", encode(bytes).into_string())
    }
}

impl std::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}

fn bytes_to_uuid(bytes: Vec<u8>) -> Result<uuid::Uuid, Error> {
    uuid::Uuid::from_slice(&bytes).map_err(Error::from)
}

impl FromStr for Uuid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        decode(s)
            .into_vec()
            .map_err(Error::from)
            .and_then(bytes_to_uuid)
            .map(Uuid::from)
    }
}

impl Default for Uuid {
    fn default() -> Self {
        uuid::Uuid::new_v4().into()
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}
