use std::default::Default;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use bs58::{decode, encode};
use crate::error::{RepoError};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr, SerializeDisplay)]
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
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for Uuid {
    type Err = RepoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut decoded = [0xFF; 16];
        decode(s).into(&mut decoded)?;
        let uuid = uuid::Uuid::from_slice(&decoded)?;
        Ok(Uuid(uuid))
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Uuid(
            uuid::Uuid::new_v4()
        )
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Uuid(uuid)
    }
}